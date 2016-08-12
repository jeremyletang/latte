// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::Context;
use backit::responses;
use iron::{Request, Response, IronResult};

// get /api/v1/message/:id
pub fn get(_: Context, _: &mut Request) -> IronResult<Response> {
    responses::ok("yolo")
}

// get /api/v1/messages
pub fn list(_: Context, _: &mut Request) -> IronResult<Response> {
    responses::ok("yolo")
}

// post /api/v1/message
pub fn create(_: Context, _: &mut Request) -> IronResult<Response> {
    responses::ok("yolo")
}

// put /api/v1/message
pub fn update(_: Context, _: &mut Request) -> IronResult<Response> {
    responses::ok("yolo")
}

// delete /api/v1/message
pub fn delete(_: Context, _: &mut Request) -> IronResult<Response> {
    responses::ok("yolo")
}
