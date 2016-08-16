// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use db::models::Message;
use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
use diesel::sqlite::SqliteConnection;

pub fn get(db: &mut SqliteConnection, get_id: &str) -> Result<Message, ::diesel::result::Error> {
    use db::schemas::messages::dsl::{messages, id};
    // search message with the provided id.
    messages.filter(id.eq(get_id)).first::<Message>(db)
}
