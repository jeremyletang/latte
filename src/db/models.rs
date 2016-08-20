// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::schemas::*;
use diesel::ExpressionMethods;
use std::convert::Into;

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, AsChangeset, Identifiable, Queryable, Serialize, Deserialize)]
#[insertable_into(users)]
#[changeset_for(users)]
pub struct User {
    pub id: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub token_id: String,
    pub slack_user_id: String,
}

impl User {
    pub fn from_slack_ids<S1, S2>(user_id: S1, token_id: S2) -> User
        where S1: Into<String>, S2: Into<String> {
        User {
            id: None,
            created_at: None,
            updated_at: None,
            slack_user_id: user_id.into(),
            token_id: token_id.into(),
        }
    }
}

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, AsChangeset, Identifiable, Queryable, Serialize, Deserialize)]
#[insertable_into(messages)]
#[changeset_for(messages)]
pub struct Message {
    pub id: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub user_id: Option<String>,
    pub body: String,
    pub channel: String,

    pub seconds: i32,
    pub utc_offset: i32,

    pub weekdays_id: String,
    pub repeated: i32,
}

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, AsChangeset, Identifiable, Queryable, Serialize, Deserialize)]
#[insertable_into(weekdays)]
#[changeset_for(weekdays)]
pub struct Weekday {
    pub id: String,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}
