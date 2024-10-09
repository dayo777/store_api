use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn start_db_server() -> surrealdb::Result<()> {
    // initialize environment variables
    dotenvy::dotenv().expect("Unable to get DB environment variables.");
    let hostname = std::env::var("DB_HOSTNAME").expect("Unable to retrieve hostname.");
    let namespace = std::env::var("DB_NAMESPACE").expect("Unable to retrieve namespace.");
    let name = std::env::var("DB_NAME").expect("Unable to retrieve database name.");
    let username = std::env::var("DB_USERNAME").expect("Unable to retrieve user.");
    let password = std::env::var("DB_PASSWORD").expect("Unable to retrieve password.");

    DB.connect::<Ws>(hostname.as_str()).await?;
    DB.signin(Root {
        username: username.as_str(),
        password: password.as_str(),
    })
    .await?;
    DB.use_ns(namespace.as_str()).use_db(name.as_str()).await?;
    Ok(())
}
