use crate::endpoint::{example_get, example_post, health_check};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Run the server using the provided TCP Listener
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
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
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
