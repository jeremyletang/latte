// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use db::models::Weekday;
use diesel::result::Error as DieselError;
use diesel::sqlite::SqliteConnection;

pub fn get(db: &mut SqliteConnection, get_id: &str) -> Result<Weekday, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::weekdays::dsl::{weekdays, id};
    // search message with the provided id.
    weekdays.filter(id.eq(get_id)).first::<Weekday>(db)
}

pub fn create(db: &mut SqliteConnection, w: &Weekday) -> Result<usize, DieselError> {
    use diesel::{self, ExecuteDsl};
    use db::schemas::weekdays;
    diesel::insert(w).into(weekdays::table).execute(db)
}

pub fn update(db: &mut SqliteConnection, m: &Weekday) -> Result<Weekday, DieselError> {
    use diesel::SaveChangesDsl;
    m.save_changes::<Weekday>(db)
}

pub fn delete(db: &mut SqliteConnection, delete_id: &str)
    -> Result<usize, DieselError> {
    use diesel::{self, ExecuteDsl, FilterDsl, ExpressionMethods};
    use db::schemas::weekdays::dsl::{weekdays, id};
    diesel::delete(weekdays.filter(id.eq(delete_id))).execute(db)
}
