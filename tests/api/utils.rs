use actix_web_template::configuration::{DatabaseSettings, Settings};
use actix_web_template::startup::run;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

const TEST_HOST: &str = "127.0.0.1";
// Port 0 selects a random available port
const TEST_PORT: u16 = 0;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Spawn an instance of the app using a random available port and return the
/// address used, including the selected port.
pub async fn spawn_app() -> TestApp {
    let test_address = format!("{TEST_HOST}:{TEST_PORT}");
    let listener = TcpListener::bind(test_address).expect("Failed to bind to random port");
    let actual_port = listener.local_addr().unwrap().port();
    let address = format!("http://{TEST_HOST}:{actual_port}");

    let mut configuration = Settings::get_config().expect("Failed to load config");

    // Randomise database name so new database is used at start of each test
    configuration.database.database_name = Uuid::new_v4().to_string();
    // Create new database with randomised name
    let db_pool = configure_database(&configuration.database).await;

    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

async fn configure_database(db_config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&db_config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let db_pool = PgPool::connect(&db_config.connection_string())
        .await
        .expect("Failed to connect to newly created database.");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate newly created database");

    db_pool
}
