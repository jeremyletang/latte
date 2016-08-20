// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use api::message::common::{ResponseMessage, Weekdays, validate};
// use api::time_utils;
use backit::{responses, json, time};
use db::models::{Message, Weekday};
use db::repositories::message as message_repo;
use db::repositories::weekday as weekday_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::convert::{From, Into};
use std::error::Error;
use uuid::Uuid;

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
