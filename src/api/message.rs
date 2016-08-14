// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::{responses, json, time};
use db::models::Message;
use diesel::{self, LoadDsl, ExecuteDsl, SaveChangesDsl, FindDsl, FilterDsl, ExpressionMethods};
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;
use uuid::Uuid;

// get /api/v1/message/:id
pub fn get(ctx: Context, req: &mut Request) -> IronResult<Response> {
    use db::schemas::messages::dsl::messages;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    // search user with the provided id.
    let result: Result<Message, _> = messages.find(id).first(db);

    // check if the request is executed with succes
    match result {
        Ok(m) => responses::ok(serde_json::to_string(&m).unwrap()),
        Err(e) => responses::bad_request(format!("id do not exist in database {}", e.description())),
    }
}

// get /api/v1/messages
pub fn list(ctx: Context, _: &mut Request) -> IronResult<Response> {
    use db::schemas::messages::dsl::{messages, user_id};
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    match messages.filter(user_id.eq(ctx.user.slack_user_id.clone())).load::<Message>(db) {
        Ok(g) => responses::ok(serde_json::to_string(&g).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}

// post /api/v1/message
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    use db::schemas::messages;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // get the message from the body
    // it must contains exlicitly ONE Message struct
    let mut m = try_or_json_error!(json::from_body::<Message, _>(&mut req.body));

    // create some mandatory fields
    m.id = Some(Uuid::new_v4().to_string());
    m.created_at = Some(time::timestamp::now() as i32);
    m.updated_at = Some(time::timestamp::now() as i32);
    m.user_id = Some(ctx.user.slack_user_id.clone());

    // insert the value + check error
    match diesel::insert(&m).into(messages::table).execute(db) {
        Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}

// put /api/v1/message
pub fn update(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // one match only
    let mut m = try_or_json_error!(json::from_body::<Message, _>(&mut req.body));

    // update time of the model
    m.updated_at = Some(time::timestamp::now() as i32);

    match m.id {
        Some(_) => {
            match m.save_changes::<Message>(db) {
                Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
                Err(e) => responses::internal_error(e.description()),
            }
        },
        None => responses::bad_request("id field is mandatory")
    }
}

// delete /api/v1/message
pub fn delete(ctx: Context, req: &mut Request) -> IronResult<Response> {
    use db::schemas::messages::dsl::{messages, id};
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let delete_id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    // first get the user
    // search user with the provided id.
    let result: Result<Message, _> = messages.find(delete_id.clone()).first(db);

    // check if the user exist, delete it
    match result {
        Ok(m) => {
            match diesel::delete(messages.filter(id.eq(delete_id))).execute(db) {
                Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
                Err(e) => responses::internal_error(e.description()),
            }
        },
        Err(e) => responses::bad_request(format!("id do not exist in database {}", e.description())),
    }
}
