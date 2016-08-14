// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::{responses, json};
use db::models::Message;
use diesel::{self, LoadDsl, ExecuteDsl, SaveChangesDsl, FindDsl, FilterDsl, ExpressionMethods};
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;
use time;
use uuid::Uuid;

fn timestamp () -> i32 {
    time::get_time().sec as i32
}

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
    use db::schemas::messages::dsl::*;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    match messages.load::<Message>(db) {
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
    let mut m: Message = match json::from_body(&mut req.body) {
        Ok(g) => g,
        Err(e) => return Ok(Response::with((e.status(), e.as_json()))),
    };

    // create some mandatory fields
    m.id = Some(Uuid::new_v4().to_string());
    m.created_at = Some(timestamp());
    m.updated_at = Some(timestamp());

    // insert the value + check error
    match diesel::insert(&m).into(messages::table).execute(db) {
        Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}

// put /api/v1/message
pub fn update(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // one
    let mut m: Message = match json::from_body(&mut req.body) {
        Ok(g) => g,
        Err(e) => return Ok(Response::with((e.status(), e.as_json()))),
    };

    // update time of the model
    m.updated_at = Some(timestamp());

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
