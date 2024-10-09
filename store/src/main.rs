use std::error::Error;

mod start_web_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    database::start_db_server().await?;
    start_web_server::start_web_server().await?;

    println!("Web server ended!");
    Ok(())
}
