use surrealdb::engine::remote::ws::{Ws};
use surrealdb::Surreal;

mod models;

// #[async_std::main]
#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: "root",
    }).await?;
    // ns = namespace, db = database; which store and groups tables
    db.use_ns("testing").use_db("stores").await?;
    Ok(())
}
