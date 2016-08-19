// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::Into;
use std::sync::{Arc, Mutex};

pub use self::cache::Cache;

mod cache;
mod error;

pub fn make_cache<S>(db_addr: S) -> Arc<Mutex<Cache>> where S: Into<String> {
    return Arc::new(Mutex::new(Cache::new()));
}
