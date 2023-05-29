use rusqlite::Connection;
use sqlx_sqlite_example::init::init;

#[derive(Debug)]
pub struct User {
    id: u32,
    name: String,
    usertype: String,
}

fn query_users(conn: &Connection) -> rusqlite::Result<Vec<User>> {
    Ok(conn
        .prepare("select id, name, type from users")?
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                usertype: row.get(2)?,
            })
        })?
        .flatten()
        .collect::<Vec<_>>()
    )
}


fn main() {
    let conn = init();
    for user in query_users(&conn).unwrap() {
        println!("{:?}", user);
    }
}