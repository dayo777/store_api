use std::error::Error;
use tracing_appender::rolling;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

mod start_web_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // run the script to create the `logs` folder before creating this part
    let dir_path = "../../logs";
    if std::fs::metadata(&dir_path).is_err() {
        std::fs::create_dir_all(dir_path)?;
    }

    let file_appender = rolling::daily("logs", "store-api.log");
    let (appender, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_filter(LevelFilter::INFO),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(appender)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    tracing::debug!("Starting store server");
    database::start_db_server().await?;
    start_web_server::start_web_server().await?;

    tracing::debug!("Stopping store server");
    Ok(())
}
