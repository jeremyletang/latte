// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::{responses, json};
use db::models::Message;
use diesel::{self, LoadDsl, ExecuteDsl, FilterDsl, ExpressionMethods};
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
    use db::schemas::messages::dsl::*;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let get_id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    // only load messages with the given id
    let results = messages.filter(id.eq(get_id)).load::<Message>(db);

    // check if the request is executed with succes
    let results = match results {
        Ok(g) => g,
        Err(e) => return responses::internal_error(e.description()),
    };

    // the request is successful, but we get an vec
    if results.len() == 0 {
        responses::bad_request("id do not exist in database")
    } else {
        responses::ok(serde_json::to_string(&results[0]).unwrap())
    }
}

// get /api/v1/messages
pub fn list(ctx: Context, _: &mut Request) -> IronResult<Response> {
    use db::schemas::messages::dsl::*;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    let results = match messages.load::<Message>(db) {
        Ok(g) => g,
        Err(e) => return responses::internal_error(e.description()),
    };

    responses::ok(serde_json::to_string(&results).unwrap())
}

// post /api/v1/message
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    use db::schemas::messages;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    let mut m: Message = match json::from_body(&mut req.body) {
        Ok(g) => g,
        Err(e) => return Ok(Response::with((e.status(), e.as_json()))),
    };
    m.id = Some(Uuid::new_v4().to_string());
    m.created_at = Some(timestamp());
    m.updated_at = Some(timestamp());
    let insert_m = diesel::insert(&m)
        .into(messages::table)
        .execute(db);

    match insert_m {
        Ok(_) => responses::ok(serde_json::to_string(&m).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}

// put /api/v1/message
pub fn update(_: Context, _: &mut Request) -> IronResult<Response> {
    responses::ok("yolo")
}

// delete /api/v1/message
pub fn delete(_: Context, _: &mut Request) -> IronResult<Response> {
    responses::ok("yolo")
}
