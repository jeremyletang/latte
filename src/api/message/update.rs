// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use api::message::common::{ResponseMessage, Weekdays, validate};
use api::time_utils;
use backit::{responses, json, time};
use db::models::{Message, Weekday};
use db::repositories::message as message_repo;
use db::repositories::weekday as weekday_repo;
use iron::{Request, Response, IronResult};
use serde_json;
use std::convert::From;
use std::error::Error;

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct UpdateMessage {
    pub id: String,

    pub body: Option<String>,
    pub channel: Option<String>,

    pub seconds: Option<i32>,
    pub utc_offset: Option<i32>,

    pub weekdays: Option<Weekdays>,
    pub repeated: Option<i32>,
}

impl UpdateMessage {
    pub fn apply(self, m: &mut Message, w: &mut Weekday) {
        if self.id != m.id.clone() {
            return
        }
        if let Some(body) = self.body {
            m.body = body;
        }
        if let Some(channel) = self.channel {
            m.channel = channel;
        }
        if let Some(seconds) = self.seconds {
            m.seconds = seconds;
        }
        if let Some(utc_offset) = self.utc_offset {
            m.utc_offset = utc_offset
        }
        if let Some(weekday) = self.weekdays {
            if let Some(monday) = weekday.monday { w.monday = monday }
            if let Some(tuesday) = weekday.tuesday { w.tuesday = tuesday }
            if let Some(wednesday) = weekday.wednesday { w.wednesday = wednesday }
            if let Some(thursday) = weekday.thursday { w.thursday = thursday }
            if let Some(friday) = weekday.friday { w.friday = friday }
            if let Some(saturday) = weekday.saturday { w.saturday = saturday }
            if let Some(sunday) = weekday.sunday { w.sunday = sunday }
        }
        if let Some(repeated) = self.repeated {
            m.repeated = repeated;
        }
    }
}


// put /api/v1/message
pub fn update(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // one match only
    let um = try_or_json_error!(json::from_body::<UpdateMessage, _>(&mut req.body));

    // get the message
    let m = match message_repo::get(db, &*um.id) {
        Ok(old) => {
            if ctx.user.slack_user_id != old.user_id.clone() {
                return responses::bad_request("cannot update a message owned by another user");
            } else {
                old
            }
        },
        Err(e) => return responses::bad_request(format!("message do not exist, {}", e.description())),
    };

    // the get the associated weekdays
    let w = weekday_repo::get(db, &*m.weekdays_id).ok().unwrap();

    // convert them to the time of the user
    let (mut m, mut w) = time_utils::utc_message_to_local_message(m, w);

    // validate and update the update time for each struct
    match validate(&m) {
        Ok(_) => {},
        Err(e) => return e,
    }

    // update time of the model
    m.updated_at = Some(time::timestamp::now() as i32);
    w.updated_at = Some(time::timestamp::now() as i32);

    // apply change to the models
    um.apply(&mut m, &mut w);

    // convert back to utc
    let (m, w) = time_utils::local_message_to_utc_message(m, w);

    // save changes,
    // first the weekdays
    match weekday_repo::update(db, &w) {
        Ok(_) => {
            // then if everything is alright the message
            match message_repo::update(db, &m) {
                Ok(_) => responses::ok(serde_json::to_string(&ResponseMessage::from((m, w))).unwrap()),
                Err(e) => responses::internal_error(e.description()),
            }
        },
        Err(e) => responses::internal_error(e.description()),
    }
}
