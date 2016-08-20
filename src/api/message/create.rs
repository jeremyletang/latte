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

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct CreateMessage {
    pub body: String,
    pub channel: String,

    pub seconds: i32,
    pub utc_offset: i32,

    pub weekdays: Weekdays,
    pub repeated: i32,
}

impl Into<(Message, Weekday)> for CreateMessage {
    fn into(self) -> (Message, Weekday) {
        let weekdays: Weekday = self.weekdays.into();
        let now = time::timestamp::now() as i32;
        let message = Message {
            id: Some(Uuid::new_v4().to_string()),
            created_at: Some(now),
            updated_at: Some(now),
            user_id: None,
            body: self.body,
            channel: self.channel,
            seconds: self.seconds,
            utc_offset: self.utc_offset,
            weekdays_id: weekdays.id.clone(),
            repeated: self.repeated,
        };
        return (message, weekdays);
    }
}

// post /api/v1/message
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // get the message from the body
    // it must contains exlicitly ONE Message struct
    let m = try_or_json_error!(json::from_body::<CreateMessage, _>(&mut req.body));

    // convert input to db models
    let (mut m, w) = m.into();
    match validate(&m) {
        Ok(_) => {},
        Err(e) => return e,
    }

    // update user_id field
    m.user_id = Some(ctx.user.slack_user_id.clone());

    // m = time_utils::local_message_to_utc_message(m);

    // insert the weekdays + check errors then inser the message
    match weekday_repo::create(db, &w) {
        Ok(_) => {
            match message_repo::create(db, &m) {
                Ok(_) => responses::ok(serde_json::to_string(&ResponseMessage::from((m, w))).unwrap()),
                Err(e) => responses::internal_error(e.description()),
            }
        },
        Err(e) => responses::internal_error(e.description()),
    }

}
