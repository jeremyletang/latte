// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::responses;
use backit::middlewares::SqliteConnectionMid;
use diesel::sqlite::SqliteConnection;
use iron::{Request, Response, IronResult};
use mid::{SlackInfo, SlackTokenMid};
use router::Router;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::sync::Arc;

mod msg;

pub struct Context {
    pub infos: SlackInfo,
    pub db: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    responses::not_found("url not found on this server")
}

pub fn extract_connection_from_request(req: &mut Request)
                                       -> Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>> {
    let pool = req.extensions
        .get::<SqliteConnectionMid>()
        .expect("cannot get database connection pool from context");
    pool.clone()
}

fn make_context_from_request(req: &mut Request) -> Context {
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
macro_rules! wrapper {
    ($f:expr) => {{
        move |req: &mut Request| {
            let ctx = make_context_from_request(req);
            $f(ctx, req)
        }
    }};
}

pub fn init() -> Router {
    router!(
        get "/api/v1/message/:id" => wrapper!(msg::get),
        get "/api/v1/messages" => wrapper!(msg::list),
        post "/api/v1/message" => wrapper!(msg::create),
        put "/api/v1/message" => wrapper!(msg::update),
        delete "/api/v1/message" => wrapper!(msg::delete),

        any "/" => not_found,
        any "/*" => not_found,
    )
}
