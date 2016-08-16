// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use db::models::Message;
use diesel::result::Error as DieselError;
use diesel::sqlite::SqliteConnection;

pub fn get(db: &mut SqliteConnection, get_id: &str) -> Result<Message, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::messages::dsl::{messages, id};
    // search message with the provided id.
    messages.filter(id.eq(get_id)).first::<Message>(db)
}

pub fn list_for_slack_user(db: &mut SqliteConnection, slack_user_id: &str)
    -> Result<Vec<Message>, ::diesel::result::Error> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::messages::dsl::{messages, user_id};
    messages.filter(user_id.eq(slack_user_id)).load::<Message>(db)
}

pub fn create(db: &mut SqliteConnection, m: &Message) -> Result<usize, DieselError> {
    use diesel::{self, ExecuteDsl};
    use db::schemas::messages;
    diesel::insert(m).into(messages::table).execute(db)
}

pub fn update(db: &mut SqliteConnection, m: &Message) -> Result<Message, DieselError> {
    use diesel::SaveChangesDsl;
    m.save_changes::<Message>(db)
}

pub fn delete(db: &mut SqliteConnection, delete_id: &str)
    -> Result<usize, DieselError> {
    use diesel::{self, ExecuteDsl, FilterDsl, ExpressionMethods};
    use db::schemas::messages::dsl::{messages, id};
    diesel::delete(messages.filter(id.eq(delete_id))).execute(db)
}
