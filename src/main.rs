// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, serde_macros, dotenv_macros, log)]
#![allow(unused_attributes)]

#[macro_use]
extern crate backit;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate log;
#[macro_use]
extern crate router;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate unicase;
extern crate uuid;

use iron::{Chain, Iron};
use std::env;

mod api;
mod db;
mod mid;
mod slack;
mod notifier;

const LATTE_ADDR: &'static str = "LATTE_ADDR";
const LATTE_DATABASE_URL: &'static str = "LATTE_DATABASE_URL";

fn main() {
    let _ = env_logger::init();
    let addr = env::var(LATTE_ADDR)
        .expect(&*format!("cannot init latte api (missing environnement var {})", LATTE_ADDR));
    let db_addr = env::var(LATTE_DATABASE_URL)
        .expect(&*format!("cannot init latte api (missing environnement var {})", LATTE_DATABASE_URL));

    // create the cache
    let cache = notifier::make_cache(&*db_addr);

    let mut chain = Chain::new(api::init());
    chain.link_before(backit::middlewares::MetricsMid);
    chain.link_before(backit::middlewares::SqliteConnectionMid::new(db_addr));
    chain.link_before(mid::CacheMid::new(cache.clone()));
    chain.link_before(mid::SlackTokenMid);
    chain.link_after(backit::middlewares::CorsMid);
    chain.link_after(backit::middlewares::MetricsMid);
    let _ = Iron::new(chain).http(&*addr).unwrap();
}
