// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod auth;
pub mod chat;

const SLACK_BASE_URL: &'static str = "https://slack.com/api/";

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlackError {
    pub ok: bool,
    pub error: String,
}
