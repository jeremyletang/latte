// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
// use api::time_utils;
use self::common::{validate};
use backit::{responses, json, time};
use db::models::{Message, Weekday};
use db::repositories::message as message_repo;
use db::repositories::weekday as weekday_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;
use uuid::Uuid;


pub use self::create::create;
pub use self::get::get;
pub use self::list::list;
pub use self::update::update;
pub use self::delete::delete;

mod common;
mod create;
mod get;
mod list;
mod update;
mod delete;
