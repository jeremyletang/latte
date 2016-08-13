// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Serialize, Deserialize, Debug, Display, Default, Clone)]
pub struct AuthTest {
    /// response status
    pub ok: bool,
    /// url of the slack.
    pub url: String,
    /// name of the team
    pub team: String,
    /// name of the user
    pub user: String,
    /// slack internal id of the team
    pub team_id: String,
    /// slack internal id of the user
    pub user_id: String,
}

const METHOD: &'static str = "auth.test";
