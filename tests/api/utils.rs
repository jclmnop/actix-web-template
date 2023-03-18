use actix_web_template::auth::compute_password_hash;
use actix_web_template::configuration::{
    DatabaseSettings, HmacSecret, Settings,
};
use actix_web_template::endpoint::admin_dashboard;
use actix_web_template::startup::run;
use actix_web_template::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use secrecy::{ExposeSecret, Secret};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

const TEST_HOST: &str = "127.0.0.1";
// Port 0 selects a random available port
const TEST_PORT: u16 = 0;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::sink,
        );
        init_subscriber(subscriber);
    };
});

/// Spawn an instance of the app using a random available port and return the
/// address used, including the selected port.
///
/// Use `TEST_LOG=true` to view all log outputs from tests.
/// For example: `TEST_LOG=true cargo test` or `TEST_LOG=true cargo test | bunyan`
/// if you'd like to prettify log output through the bunyan cli app.
pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let test_address = format!("{TEST_HOST}:{TEST_PORT}");
    let listener =
        TcpListener::bind(test_address).expect("Failed to bind to random port");
    let actual_port = listener.local_addr().unwrap().port();
    let address = format!("http://{TEST_HOST}:{actual_port}");

    let mut configuration =
        Settings::get_config().expect("Failed to load configuration");

    // Randomise database name so new database is used at start of each test
    configuration.database.database_name = Uuid::new_v4().to_string();
    // Create new database with randomised name
    let db_pool = configure_database(&configuration.database).await;

    let server = run(
        listener,
        db_pool.clone(),
        HmacSecret(configuration.app.hmac_secret),
        configuration.redis_uri,
    )
    .await
    .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    let api_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .expect("Failed to build reqwest client");

    let test_app = TestApp {
        address,
        db_pool,
        test_user: TestUser::generate(),
        api_client,
    };
    test_app.test_user.store(&test_app.db_pool).await;

    test_app
}

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    // Response is a redirect
    assert_eq!(response.status().as_u16(), 303);

    // Response redirects to `location`
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}/login", &self.address))
            .form(body)
            .send()
            .await
            .expect("Failed to execute login request")
    }

    pub async fn get_login_html(&self) -> String {
        self.api_client
            .get(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to request login form")
            .text()
            .await
            .expect("Failed to parse HTML from login form")
    }

    pub async fn get_admin_dashboard(&self) -> String {
        self.api_client
            .get(&format!("{}{}", &self.address, admin_dashboard::PATH))
            .send()
            .await
            .expect("Failed to get admin dashboard")
            .text()
            .await
            .expect("Failed to parse HTML from admin dashboard")
    }
}

pub struct TestUser {
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    pub async fn store(&self, pool: &PgPool) {
        let password = self.password.clone();
        let password_hash = actix_web::rt::task::spawn_blocking(move || {
            compute_password_hash(Secret::new(password))
        })
        .await
        .unwrap()
        .unwrap();

        sqlx::query!(
            "INSERT INTO users (username, password)\
            VALUES ($1, $2)",
            self.username,
            password_hash.expose_secret()
        )
        .execute(pool)
        .await
        .expect("Failed to store test user");
    }
}

async fn configure_database(db_config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&db_config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(
            format!(r#"CREATE DATABASE "{}";"#, db_config.database_name)
                .as_str(),
        )
        .await
        .expect("Failed to create database");

    let db_pool = PgPool::connect_with(db_config.with_db())
        .await
        .expect("Failed to connect to newly created database.");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate newly created database");

    db_pool
}
