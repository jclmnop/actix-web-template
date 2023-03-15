use crate::configuration::HmacSecret;
use crate::endpoint::{
    example_auth, example_get, example_post, health_check, home, login,
    login_form,
};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Run the server using the provided TCP Listener
pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    hmac_secret: HmacSecret,
    redis_uri: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let secret_key = Key::from(hmac_secret.0.expose_secret().as_bytes());
    let connection_pool = Data::new(db_pool);
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework =
        FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    // Build the app
    let server = HttpServer::new(move || {
        //TODO: add service in configuration fn instead
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(example_get)
            .service(example_post)
            .service(example_auth)
            .service(home)
            .service(login_form)
            .service(login)
            .app_data(connection_pool.clone())
            .app_data(Data::new(hmac_secret.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
