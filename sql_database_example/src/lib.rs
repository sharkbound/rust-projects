use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

pub mod models;
pub mod schema;
pub mod actions;

pub fn create_connection() -> SqliteConnection {
    // cargo install diesel_cli --no-default-features --features sqlite
    // had to use these to help install diesel and get it running with sqlite:
    // https://github.com/diesel-rs/diesel/issues/487
    // https://users.rust-lang.org/t/struggling-with-diesel-sqlite-on-windows/73336
    // can also used "bundled" feature with libsqlite3-sys (but its not recommended)

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to database: {}", database_url));
    connection
}