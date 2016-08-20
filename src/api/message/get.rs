// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
// use api::time_utils;
use api::message::common::ResponseMessage;
use backit::{responses, json, time};
use db::models::{Message, Weekday};
use db::repositories::message as message_repo;
use db::repositories::weekday as weekday_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;
use uuid::Uuid;

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
            // m = time_utils::utc_message_to_local_message(m);
            match weekday_repo::get(db, &*m.weekdays_id) {
                Ok(w) => responses::ok(serde_json::to_string(&ResponseMessage::from((m, w))).unwrap()),
                Err(e) => responses::not_found(format!("id do not exist in database {}", e.description()))
            }

        },
        Err(e) => responses::not_found(format!("id do not exist in database {}", e.description())),
    }
}
