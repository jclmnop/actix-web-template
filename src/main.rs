use actix_web_template::configuration::Settings;
use actix_web_template::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = Settings::get_config().expect("Failed to load config");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = TcpListener::bind(configuration.get_address())?;
    run(listener, db_pool)?.await
}
