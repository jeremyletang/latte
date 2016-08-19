// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::json::Error as JsonError;
use db::models::User;
use db::repositories::user as user_repo;
use diesel::sqlite::SqliteConnection;
use iron::{BeforeMiddleware, Request, IronResult, typemap};
use iron::error::IronError;
use notifier::Cache;
use std::sync::{Arc, Mutex};

pub struct CacheMid {
    cache: Arc<Mutex<Cache>>
}

impl CacheMid {
    pub fn new(cache: Arc<Mutex<Cache>>) -> CacheMid {
        CacheMid {
            cache: cache,
        }
    }
}

impl typemap::Key for CacheMid {
    type Value = Arc<Mutex<Cache>>;
}

impl BeforeMiddleware for CacheMid {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let cache = self.cache.clone();
        req.extensions.insert::<CacheMid>(cache);
        Ok(())
    }
}

pub fn extract_cache_from_request(req: &mut Request) -> Arc<Mutex<Cache>> {
    let cache = req.extensions
        .get::<CacheMid>()
        .expect("cannot get database connection pool from context");
    cache.clone()
}
