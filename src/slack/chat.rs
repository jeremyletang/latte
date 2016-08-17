// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod post_message {

    use backit::json;
    use hyper::Client;
    use slack::SlackError;

    #[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Message {
        #[serde(rename = "type")]
        pub type_: String,
        pub user: String,
        pub text: String,
        pub ts: String,
    }

    #[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ChatPostMessage {
        pub ok: bool,
        pub channel: String,
        pub ts: String,
        pub message: Option<Message>,
    }

    const METHOD: &'static str = "chat.postMessage";
    // chat.postMessage\?token\=xoxs-3815959245-47261519462-68846856535-6efa9faee4\&channel\=U1D7PF9DL\&text\=yolo

    pub fn call(token: &str, channel: &str, text: &str) -> Result<ChatPostMessage, json::Error> {
        let client = Client::new();
        // make the url
        let url = format!("{}{}?token={}&channel={}&text={}",
            ::slack::SLACK_BASE_URL, METHOD, token, channel, text);
        match client.get(&*url).send() {
            Ok(mut r) => {
                match json::from_body::<ChatPostMessage, _>(&mut r) {
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

}
