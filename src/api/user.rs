// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::responses;
use db::models::User;
use diesel::{FindDsl, LoadDsl};
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;

pub fn get(ctx: Context, req: &mut Request) -> IronResult<Response> {
    use db::schemas::users::dsl::users;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    // search user with the provided id.
    let result: Result<User, _> = users.find(id).first(db);

    // check if the request is executed with succes
    match result {
        Ok(u) => responses::ok(serde_json::to_string(&u).unwrap()),
        Err(e) => responses::bad_request(format!("id do not exist in database {}", e.description())),
    }
}

// get /api/v1/users
pub fn list(ctx: Context, _: &mut Request) -> IronResult<Response> {
    use db::schemas::users::dsl::*;
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    match users.load::<User>(db) {
        Ok(g) => responses::ok(serde_json::to_string(&g).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
