use sql_database_example::actions::{create_user, ReturnUser};
use sql_database_example::create_connection;

fn main() {
    let mut conn = create_connection();
    dbg!(create_user(&mut conn, "user", "type", ReturnUser::Yes));
}