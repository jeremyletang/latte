// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::{responses, time};
use iron::{Response, IronResult};
use uuid::Uuid;

const DAY_SECONDS: i32 = 86400;

fn validate_hour(seconds: i32) -> bool {
    (seconds > 0) && (seconds < DAY_SECONDS)
}

pub fn validate(m: &::db::models::Message) -> Result<(), IronResult<Response>> {
    if !validate_hour(m.seconds) {
        return Err(responses::bad_request("invalid hour format, hour must be in the range 0-23, and minutes 0-59"));
    }

    return Ok(());
}

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Weekdays {
    pub monday: Option<bool>,
    pub tuesday: Option<bool>,
    pub wednesday: Option<bool>,
    pub thursday: Option<bool>,
    pub friday: Option<bool>,
    pub saturday: Option<bool>,
    pub sunday: Option<bool>,
}

fn to_bool(b: Option<bool>) -> bool {
    match b {
        Some(true) => true,
        Some(false) => false,
        None => false,
    }
}

impl Into<::db::models::Weekday> for Weekdays {
    fn into(self) -> ::db::models::Weekday {
        let weekdays_id = Uuid::new_v4().to_string();
        let now = time::timestamp::now() as i32;
        ::db::models::Weekday {
            id: weekdays_id,
            created_at: Some(now),
            updated_at: Some(now),
            monday: to_bool(self.monday),
            tuesday: to_bool(self.tuesday),
            wednesday: to_bool(self.wednesday),
            thursday: to_bool(self.thursday),
            friday: to_bool(self.friday),
            saturday: to_bool(self.saturday),
            sunday: to_bool(self.sunday),
        }
    }
}

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub id: String,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub body: String,
    pub channel: String,

    pub seconds: i32,
    pub utc_offset: i32,

    pub weekdays: ::db::models::Weekday,
    pub repeated: i32,
}

impl From<(::db::models::Message, ::db::models::Weekday)> for ResponseMessage {
    fn from(t: (::db::models::Message, ::db::models::Weekday)) -> ResponseMessage {
        ResponseMessage {
            id: t.0.id.unwrap(),
            created_at: t.0.created_at,
            updated_at: t.0.updated_at,
            body: t.0.body,
            channel: t.0.channel,
            seconds: t.0.seconds,
            utc_offset: t.0.utc_offset,
            weekdays: t.1,
            repeated: t.0.repeated,
        }
    }
}
