use sqlx_sqlite_example::init::init;

macro_rules! ask {
    ($var:ident, $query:expr) => {
        paste::paste! {
            let mut $var = String::new();
            println!("{}", $query);
            std::io::stdin().read_line(&mut $var).unwrap();
            let $var = $var.trim();
        }
    };
}


fn main() {
    let conn = init();

    ask!(name, "What is your name? ");
    ask!(usertype, "What is your user type? ");

    let result = conn.execute(r#"
        insert into users (name, type) values (?1,?2)
    "#, (&name, &usertype));

    println!("{:?}", result);
}