// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::schemas::*;
use diesel::ExpressionMethods;

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, AsChangeset, Identifiable, Queryable, Serialize, Deserialize)]
#[insertable_into(users)]
#[changeset_for(users)]
pub struct User {
    pub id: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub slack_user_id: String,
    pub token_id: String,
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
    pub at: i32,
    pub weekdays: String,
    pub repeated: i32,
}
