use sqlx_sqlite_example::init::init;

fn main() {
    let conn = init();
    println!("{:?}", conn.execute("delete from users", ()));
}