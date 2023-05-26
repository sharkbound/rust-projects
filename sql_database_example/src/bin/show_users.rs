use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use sql_database_example::create_connection;
use sql_database_example::models::User;

fn main() {
    use sql_database_example::schema::users::dsl::*;
    let mut connection = create_connection();
    let results = users
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users");

    println!("Found {} users!", results.len());
    for user in &results {
        println!("{:?}", user);
    }
}
