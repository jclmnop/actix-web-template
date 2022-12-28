use actix_web_template::configuration::Settings;
use actix_web_template::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use actix_web_template::telemetry::{get_subscriber, init_subscriber};

const APP_NAME: &str = "example-app";
const DEFAULT_LOG_LEVEL: &str = "info";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(APP_NAME.into(), DEFAULT_LOG_LEVEL.into());
    init_subscriber(subscriber);
    let configuration = Settings::get_config().expect("Failed to load config");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = TcpListener::bind(configuration.get_address())?;
    run(listener, db_pool)?.await
}
