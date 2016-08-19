// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use api::time_utils;
use backit::{responses, json, time};
use db::models::Message;
use db::repositories::message as message_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;
use uuid::Uuid;

const DAY_SECONDS: i32 = 86400;

fn validate_hour(seconds: i32) -> bool {
    (seconds > 0) && (seconds < DAY_SECONDS)
}

fn validate_days(m: &Message) -> bool {
    m.monday.is_some() || m.tuesday.is_some() || m.wednesday.is_some() || m.thursday.is_some()
    || m.friday.is_some() || m.saturday.is_some() || m.sunday.is_some()
}

fn validate(m: &Message) -> Result<(), IronResult<Response>> {
    if !validate_hour(m.seconds) {
        return Err(responses::bad_request("invalid hour format, hour must be in the range 0-23, and minutes 0-59"));
    }

    if !validate_days(m) {
        return Err(responses::bad_request("you need to specify at least one day to send the message"));
    }

    return Ok(());
}

// get /api/v1/message/:id
pub fn get(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    // check if the request is executed with succes
    match message_repo::get(db, &*id) {
        Ok(mut m) => {
            if ctx.user.slack_user_id != m.user_id.clone().unwrap() {
                return responses::bad_request("cannot get a message owned by another user");
            }
            m = time_utils::utc_message_to_local_message(m);
            responses::ok(serde_json::to_string(&m).unwrap())
        },
        Err(e) => responses::bad_request(format!("id do not exist in database {}", e.description())),
    }
}

// get /api/v1/messages
pub fn list(ctx: Context, _: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    match message_repo::list_for_slack_user(db, &*ctx.user.slack_user_id) {
        Ok(mut lm) => {
            // convert back the message to the timezone of the user
            lm = lm.into_iter().map(|m| time_utils::utc_message_to_local_message(m)).collect();
            responses::ok(serde_json::to_string(&lm).unwrap())
        },
        Err(e) => responses::internal_error(e.description()),
    }
}

// post /api/v1/message
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // get the message from the body
    // it must contains exlicitly ONE Message struct
    let mut m = try_or_json_error!(json::from_body::<Message, _>(&mut req.body));

    match validate(&m) {
        Ok(_) => {},
        Err(e) => return e,
    }

    // create some mandatory fields
    m.id = Some(Uuid::new_v4().to_string());
    m.created_at = Some(time::timestamp::now() as i32);
    m.updated_at = Some(time::timestamp::now() as i32);
    m.user_id = Some(ctx.user.slack_user_id.clone());

    m = time_utils::local_message_to_utc_message(m);

    // insert the value + check error
    match message_repo::create(db, &m) {
        Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}

// put /api/v1/message
pub fn update(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // one match only
    let mut m = try_or_json_error!(json::from_body::<Message, _>(&mut req.body));

    match validate(&m) {
        Ok(_) => {},
        Err(e) => return e,
    }

    // update time of the model
    m.updated_at = Some(time::timestamp::now() as i32);

    match m.id {
        Some(ref id_to_update) => {
            match message_repo::get(db, &*id_to_update) {
                Ok(old) => {
                    if ctx.user.slack_user_id != old.user_id.clone().unwrap() {
                        return responses::bad_request("cannot update a message owned by another user");
                    }
                },
                Err(e) => return responses::bad_request(format!("message do not exist, {}", e.description())),
            }

            match message_repo::update(db, &m) {
                Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
                Err(e) => responses::internal_error(e.description()),
            }
        },
        None => responses::bad_request("id field is mandatory")
    }
}

// delete /api/v1/message
pub fn delete(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let delete_id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    // check if the user exist, delete it
    match message_repo::get(db, &*delete_id) {
        Ok(m) => {
            if ctx.user.slack_user_id != m.user_id.clone().unwrap() {
                return responses::bad_request("cannot delete a message owned by another user");
            }
            match message_repo::delete(db, &*delete_id) {
                Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
                Err(e) => responses::internal_error(e.description()),
            }
        },
        Err(e) => responses::bad_request(format!("id do not exist in database {}", e.description())),
    }
}
