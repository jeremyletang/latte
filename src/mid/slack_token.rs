// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::json::Error as JsonError;
use iron::{BeforeMiddleware, Request, IronResult, typemap};
use iron::error::IronError;
use std::error::Error;

pub struct SlackTokenMid;

#[derive(Clone)]
pub struct SlackInfo {
    pub token: String,
    pub user_id: String,
}

impl typemap::Key for SlackTokenMid {
    type Value = SlackInfo;
}

fn make_slack_info(token: &str) -> Result<SlackInfo, JsonError> {
    return Ok(SlackInfo{token: token.to_string(), user_id: "".to_string()})
}

impl BeforeMiddleware for SlackTokenMid {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        match req.headers.get_raw("X-Slack-Token") {
            Some(h) => {
                if h.len() != 1 {
                    let err = JsonError::bad_request("invalid X-Slack-Token");
                    return Err(IronError::new(err.clone(), (err.status(), err.as_json())));
                }
                let token = match String::from_utf8(h[0].clone()) {
                    Ok(t) => t,
                    Err(e) => {
                        let err = JsonError::bad_request("invalid X-Slack-Token");
                        return Err(IronError::new(err.clone(), (err.status(), err.as_json())));
                    }
                };
                let si = match make_slack_info(&*token) {
                    Ok(si) => si,
                    Err(e) => return Err(IronError::new(e.clone(), (e.status(), e.as_json())))
                };
                req.extensions.insert::<SlackTokenMid>(si);
                Ok(())
            },
            None => {
                let err = JsonError::bad_request("missing X-Slack-Token");
                Err(IronError::new(err.clone(), (err.status(), err.as_json())))
            }
        }

    }
}
