// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::json;
use hyper::Client;

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

pub fn call(token: &str) -> Result<AuthTest, json::Error> {
    let client = Client::new();
    // make the url
    let url = format!("{}{}?token={}", ::slack::SLACK_BASE_URL, METHOD, token);
    match client.get(&*url).send() {
        Ok(mut r) => {
            match json::from_body::<AuthTest, _>(&mut r) {
                Ok(at) => Ok(at),
                Err(e) => {
                    let estr = format!("error authenticating with slack {}", e);
                    Err(json::Error::internal_error(estr))
                }
            }
        },
        Err(e) => {
            let estr = format!("error while calling slack api {}", e);
            Err(json::Error::internal_error(&*estr))
        }
    }
}
