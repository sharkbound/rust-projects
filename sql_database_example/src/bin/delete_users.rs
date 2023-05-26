use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use sql_database_example::create_connection;
use sql_database_example::models::User;

fn main() {
    use sql_database_example::schema::users::dsl::*;
    let mut connection = create_connection();
    let deleted_rows = diesel::delete(users).execute(&mut connection).unwrap_or_default();

    println!("Delete {} rows!", deleted_rows);
}
