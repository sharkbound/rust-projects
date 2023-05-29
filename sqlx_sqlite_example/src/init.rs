use std::path::Path;
use rusqlite::Connection;

pub fn create_connection() -> rusqlite::Result<Connection> {
    Connection::open(Path::new("test.sqlite"))
}

pub fn init_tables(conn: &Connection) -> rusqlite::Result<usize> {
    conn.execute(r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            name VARCHAR(255) NOT NULL,
            type VARCHAR(255) NOT NULL
        );
    "#, ())
}

pub fn init() -> Connection {
    let conn = create_connection().unwrap();
    init_tables(&conn).unwrap();
    conn
}
