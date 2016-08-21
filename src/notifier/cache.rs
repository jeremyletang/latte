// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use db::models::Message;
use std::collections::HashMap;
use std::convert::{From, Into};
use notifier::error::Error;

const HOUR_MIN: usize = 0;
const HOUR_MAX: usize = 23;
const MINUTE_MIN: usize = 0;
const MINUTE_MAX: usize = 59;

// This contains the minimal informations to send a slack message
#[derive(Clone)]
pub struct SmallMessage {
    pub body: String,
    pub slack_user_id: String,
    pub slack_token_id: String,
    pub channel: String,
}

impl<S> From<(Message, S)> for SmallMessage
    where S: Into<String> {
    fn from((msg, token): (Message, S))
        -> SmallMessage  {
        SmallMessage {
            body: msg.body,
            slack_user_id: msg.user_id,
            slack_token_id: token.into(),
            channel: msg.channel,
        }
    }
}

#[derive(Clone)]
pub struct MinuteCache {
    // the hashmap contains the id of the message as a String
    // and the SMallMessag
    messages: HashMap<String, SmallMessage>,
}

impl MinuteCache {
    pub fn new() -> MinuteCache {
        MinuteCache {
            messages: HashMap::new(),
        }
    }

    pub fn upsert<S>(&mut self, message_id: S, message: SmallMessage)
        -> Result<(), Error> where S: Into<String> {
            let _ = self.messages.insert(message_id.into(), message);
            return Ok(());
    }

    pub fn remove<S>(&mut self, message_id: S) -> Result<(), Error> where S: Into<String> {
        match self.messages.remove(&message_id.into()) {
            Some(_) => Ok(()),
            None => Err(Error::UnknownMessageId),
        }
    }
}

#[derive(Clone)]
pub struct HourCache {
    minutes: Vec<MinuteCache>,
}

impl HourCache {
    pub fn new() -> HourCache {
        HourCache {
            minutes: vec![MinuteCache::new(); MINUTE_MAX],
        }
    }

    pub fn upsert<S>(&mut self, minute: usize, message_id: S, message: SmallMessage)
        -> Result<(), Error> where S: Into<String> {
            if minute > MINUTE_MAX || minute < MINUTE_MIN {
                return Err(Error::MinuteOutOfRange);
            }
            return self.minutes[minute].upsert(message_id, message);
    }

    pub fn remove<S>(&mut self, minute: usize, message_id: S)
        -> Result<(), Error> where S: Into<String> {
        if minute > MINUTE_MAX || minute < MINUTE_MIN {
            return Err(Error::MinuteOutOfRange);
        }
        return self.minutes[minute].remove(message_id);
    }
}

#[derive(Clone)]
pub struct Cache {
    hours: Vec<HourCache>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            hours: vec![HourCache::new(); HOUR_MAX],
        }
    }

    // update or insert if the hour and minute do not change
    pub fn upsert<S>(&mut self,
                     (hour, minute): (usize, usize),
                     message_id: S,
                     message: SmallMessage)
                     -> Result<(), Error>
                     where S: Into<String> {
        // check input hour
        if hour > HOUR_MAX || hour < HOUR_MIN {
            return Err(Error::HourOutOfRange);
        }
        return self.hours[hour].upsert(minute, message_id, message);
    }

    // update and insert with a new hour
    // this will first make an insert to the new hour/minute
    // then remove the old one from the cache
    pub fn upsert_with_time<S>(&mut self,
                               (old_hour, old_minute): (usize, usize),
                               (hour, minute): (usize, usize),
                               message_id: S,
                               message: SmallMessage)
                               -> Result<(), Error>
                               where S: Into<String> + Clone {
        // check input hour
        if hour > HOUR_MAX || hour < HOUR_MIN ||
            old_hour > HOUR_MAX || old_hour < HOUR_MIN {
                return Err(Error::HourOutOfRange);
        }
        if minute > MINUTE_MAX ||  minute < MINUTE_MIN ||
            old_minute > MINUTE_MAX ||  old_minute < MINUTE_MIN {
                return Err(Error::MinuteOutOfRange);
        }
        // insert the new one
        let _ = self.hours[hour].upsert(minute, message_id.clone(), message);
        // remove the old one
        return self.hours[old_hour].remove(old_minute, message_id);
    }

    pub fn remove<S>(&mut self, (hour, minute): (usize, usize), message_id: S)
        -> Result<(), Error> where S: Into<String> {
        // check input hour
        if hour > HOUR_MAX || hour < HOUR_MIN {
            return Err(Error::HourOutOfRange);
        }
        return self.hours[hour].remove(minute, message_id);
    }

    pub fn flush_and_update_with(&mut self, oth: Cache) {
        self.hours = oth.hours;
    }
}
