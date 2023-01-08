use crate::endpoint::{example_get, example_post, health_check};
use actix_web::dev::{Server, ServiceFactory, ServiceRequest};
use actix_web::{web, App, Error, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

/// Run the server using the provided TCP Listener
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || build_app(App::new(), db_pool.clone()))
        .listen(listener)?
        .run();
    Ok(server)
}

//TODO: replace with config
/// Build the actix-web application
fn build_app<T>(app: App<T>, db_pool: PgPool) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    let connection_pool = web::Data::new(db_pool);
    app.service(health_check)
        .service(example_get)
        .service(example_post)
        .app_data(connection_pool)
}
