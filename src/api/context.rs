// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::middlewares::SqliteConnectionMid;
use diesel::sqlite::SqliteConnection;
use iron::Request;
use mid::{SlackInfo, SlackTokenMid};
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::sync::Arc;

pub struct Context {
    pub infos: SlackInfo,
    pub db: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}

pub fn extract_connection_from_request(req: &mut Request)
                                       -> Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>> {
    let pool = req.extensions
        .get::<SqliteConnectionMid>()
        .expect("cannot get database connection pool from context");
    pool.clone()
}

pub fn make_context_from_request(req: &mut Request) -> Context {
    let db = extract_connection_from_request(req);
    let si = req.extensions
        .get::<SlackTokenMid>()
        .expect("cannot get SlackTokenMid from iron extensions");

    Context {
        infos: (*si).clone(),
        db: db,
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
