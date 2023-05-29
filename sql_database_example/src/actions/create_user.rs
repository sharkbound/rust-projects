// use diesel::prelude::*;

use diesel::associations::HasTable;
use diesel::{RunQueryDsl, sql_query, SqliteConnection};
use crate::models::{NewUser, User};
use crate::schema::users::dsl::users;

pub enum ReturnUser {
    Yes,
    No,
}

pub fn create_user(connection: &mut SqliteConnection, username: &str, user_type: &str, return_user: ReturnUser) -> Option<User> {
    let user = NewUser { username, user_type };
    let result = diesel::insert_into(users::table())
        .values(&user)
        .execute(connection);

    match result {
        Ok(_) => {}
        Err(_) => return None,
    };

    match return_user {
        ReturnUser::No => None,
        ReturnUser::Yes => {
            match sql_query("select * from users where id = last_insert_rowid() limit 1").load::<User>(connection) {
                Ok(mut res) => {
                    if res.is_empty() {
                        return None;
                    }
                    Some(res.remove(0))
                }
                Err(_) => None,
            }
        }
    }
}