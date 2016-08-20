// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::responses;
use iron::{Request, Response, IronResult};
use router::Router;

#[macro_use]
pub mod context;
pub mod message;
pub mod user;
// pub mod time_utils;

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    responses::not_found("url not found on this server")
}

pub fn init() -> Router {
    router!(
        get "/api/v1/message/:id" => wrap_ctx!(message::get),
        get "/api/v1/messages" => wrap_ctx!(message::list),
        post "/api/v1/message" => wrap_ctx!(message::create),
        put "/api/v1/message" => wrap_ctx!(message::update),
        delete "/api/v1/message/:id" => wrap_ctx!(message::delete),

        get "/api/v1/user/:id" => wrap_ctx!(user::get),
        get "/api/v1/users" => wrap_ctx!(user::list),

        any "/" => not_found,
        any "/*" => not_found,
    )
}
