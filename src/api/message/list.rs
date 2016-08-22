// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use api::time_utils;
use api::message::common::ResponseMessage;
use backit::responses;
use db::repositories::message as message_repo;
use db::repositories::weekday as weekday_repo;
use iron::{Request, Response, IronResult};
use serde_json;
use std::error::Error;

// get /api/v1/messages
pub fn list(ctx: Context, _: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    match message_repo::list_for_slack_user(db, &*ctx.user.slack_user_id) {
        Ok(lm) => {
            // convert back the message to the timezone of the user
            let lrm: Vec<ResponseMessage> = lm.into_iter().map(|m| {
                let w = weekday_repo::get(db, &*m.weekdays_id).ok().unwrap();
                let (m, w) = time_utils::utc_message_to_local_message(m, w);
                ResponseMessage::from((m, w))
            }).collect();

            responses::ok(serde_json::to_string(&lrm).unwrap())
        },
        Err(e) => responses::internal_error(e.description()),
    }
}
