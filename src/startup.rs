use crate::endpoint::{
    example_auth, example_get, example_post, health_check, home, login,
    login_form,
};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use actix_web::web::Data;
use tracing_actix_web::TracingLogger;
use crate::configuration::HmacSecret;

/// Run the server using the provided TCP Listener
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    hmac_secret: HmacSecret,
) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(db_pool);
    // Build the app
    let server = HttpServer::new(move || {
        //TODO: add service in config fn instead
        App::new()
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
