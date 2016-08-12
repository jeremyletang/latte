// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::responses;
use iron::{Request, Response, IronResult};
use router::Router;

mod msg;

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    responses::not_found("url not found on this server")
}

pub fn init() -> Router {
    router!(
        get "/api/v1/message/:id" => msg::get,
        get "/api/v1/messages" => msg::list,
        post "/api/v1/message" => msg::create,
        put "/api/v1/message" => msg::update,
        delete "/api/v1/message" => msg::delete,

        any "/" => not_found,
        any "/*" => not_found,
    )
}
