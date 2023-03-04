use crate::configuration::HmacSecret;
use crate::endpoint::{
    example_auth, example_get, example_post, health_check, home, login,
    login_form,
};
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Run the server using the provided TCP Listener
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    hmac_secret: HmacSecret,
) -> Result<Server, std::io::Error> {
    let connection_pool = Data::new(db_pool);
    let message_store = CookieMessageStore::builder(Key::from(
        hmac_secret.0.expose_secret().as_bytes(),
    ))
    .build();
    let message_framework =
        FlashMessagesFramework::builder(message_store).build();
    // Build the app
    let server = HttpServer::new(move || {
        //TODO: add service in config fn instead
        App::new()
            .wrap(message_framework.clone())
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
