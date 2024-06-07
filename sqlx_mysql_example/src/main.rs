use async_std::stream::StreamExt;
use sqlx::{FromRow, MySql, MySqlPool, Pool};

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://root:ROOT@localhost:3306/rust_item_store").await?;
    create_items_tables(&pool).await?;
    insert_default_items(&pool).await?;

    // lazy iterator
    let mut iterator = sqlx::query_as!(Item, "SELECT * FROM items").fetch(&pool);
    while let Some(item) = iterator.next().await {
        match item {
            Ok(item) => println!("ITEM [LAZY] | {:?}", item),
            Err(e) => eprintln!("Error retrieving item: {}", e),
        }
    };

    // lazy iterator with map
    let mut iterator = sqlx::query_as!(Item, "SELECT * FROM items").fetch(&pool).map(|x| x.and_then(|y| Ok(y.id)));
    while let Some(item) = iterator.next().await {
        match item {
            Ok(item) => println!("ITEM [MAP ONLY ID] | {:?}", item),
            Err(e) => eprintln!("Error retrieving item: {}", e),
        }
    };


    println!("{:?}", pool);
    Ok(())
}

async fn insert_default_items(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    create_item(&pool, "Milk", 100, 20).await?;
    create_item(&pool, "Raisins", 5, 200).await?;
    Ok(())
}

async fn create_items_tables(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS items (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL UNIQUE,
            price INT NOT NULL,
            quantity INT NOT NULL
        )"#,
    ).execute(pool).await?;
    Ok(())
}

async fn create_item(pool: &MySqlPool, name: &str, price: i32, quantity: i32) -> Result<(), sqlx::Error> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM items WHERE name = ? LIMIT 1)", name).fetch_one(pool).await? == 1;
    if exists {
        return Ok(());
    }
    sqlx::query!(
        r#" INSERT IGNORE INTO items (name, price, quantity) values (?, ?, ?) "#,
        name,
        price,
        quantity,
    ).execute(pool).await?;
    Ok(())
}

#[derive(FromRow, Debug)]
struct Item {
    id: i32,
    name: String,
    price: i32,
    quantity: i32,
}