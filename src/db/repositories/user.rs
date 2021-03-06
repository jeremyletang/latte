// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::{json, time};
use db::models::User;
use diesel::result::Error as DieselError;
use std::error::Error;
use uuid::Uuid;
use diesel::sqlite::SqliteConnection;

pub fn create(db: &mut SqliteConnection, mut u: User) -> Result<User, json::Error> {
    use diesel::{self, ExecuteDsl};
    use db::schemas::users;

    // create some mandatory fields
    u.id = Uuid::new_v4().to_string();
    u.created_at = Some(time::timestamp::now() as i32);
    u.updated_at = Some(time::timestamp::now() as i32);

    // insert the value + check error
    match diesel::insert(&u).into(users::table).execute(db) {
        Ok(_) => Ok(u),
        Err(e) => Err(json::Error::internal_error(e.description())),
    }
}

pub fn update(db: &mut SqliteConnection, mut u: User) -> Result<User, json::Error> {
    use diesel::SaveChangesDsl;
    // update time of the model
    u.updated_at = Some(time::timestamp::now() as i32);

    match u.save_changes::<User>(db) {
        Ok(_) => Ok(u),
        Err(e) => Err(json::Error::internal_error(e.description())),
    }
}

pub fn get(db: &mut SqliteConnection, get_id: &str) -> Result<User, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::users::dsl::{users, id};
    // search user with the provided id.
    users.filter(id.eq(get_id)).first::<User>(db)
}

pub fn list(db: &mut SqliteConnection) -> Result<Vec<User>, DieselError> {
    use diesel::LoadDsl;
    use db::schemas::users::dsl::users;
    // search user with the provided id.
    users.load::<User>(db)
}

pub fn get_from_token(db: &mut SqliteConnection, token: &str) -> Option<User> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::users::dsl::{users, token_id};
    match users.filter(token_id.eq(token)).first(db) {
        Ok(u) => Some(u),
        Err(_) => None,
    }
}

pub fn get_from_slack_user_id(db: &mut SqliteConnection, get_id: &str) -> Result<User, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::users::dsl::{users, slack_user_id};
    // search user with the provided id.
    users.filter(slack_user_id.eq(get_id)).first::<User>(db)
}
