// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::middlewares::extract_connection_from_request;
use db::models::User;
use diesel::sqlite::SqliteConnection;
use iron::Request;
use mid::{SlackTokenMid, CacheMid, extract_cache_from_request};
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::sync::{Arc, Mutex};
use notifier::Cache;

pub struct Context {
    pub user: User,
    pub db: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    pub cache: Arc<Mutex<Cache>>,
}

pub fn make_context_from_request(req: &mut Request) -> Context {
    let cache = extract_cache_from_request(req);
    let db = extract_connection_from_request(req);
    let si = req.extensions
        .get::<SlackTokenMid>()
        .expect("cannot get SlackTokenMid from iron extensions");

    Context {
        user: (*si).clone(),
        db: db,
        cache: cache,
    }
}

#[macro_export]
macro_rules! wrap_ctx {
    ($f:expr) => {{
        move |req: &mut Request| {
            let ctx = $crate::api::context::make_context_from_request(req);
            $f(ctx, req)
        }
    }};
}
