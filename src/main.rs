// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate backit;
extern crate dotenv;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate unicase;

use iron::{Chain, Iron};
use std::env;

mod api;
mod mid;

fn main() {
    let addr = env::var("LATTE_ADDR")
        .expect("cannot init latte api (missing environnement var LATTE_ADDR)");
    let db = env::var("LATTE_DB_ADDR")
        .expect("cannot init latte api (missing environnement var LATTE_DB_ADDR)");

    let mut chain = Chain::new(api::init());
    chain.link_before(backit::middlewares::MetricsMid);
    chain.link_before(mid::SlackTokenMid);
    chain.link_after(backit::middlewares::CorsMid);
    chain.link_after(backit::middlewares::MetricsMid);
    let _ = Iron::new(chain).http(&*addr).unwrap();
}
