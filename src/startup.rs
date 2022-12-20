use crate::endpoint::Endpoint::{ExamplePost, HealthCheck};
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

/// Build the actix-web application
fn build_app<T>(app: App<T>, db_pool: PgPool) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    let connection_pool = web::Data::new(db_pool);
    app.route(HealthCheck.get_path(), HealthCheck.get_handler())
        .route(ExamplePost.get_path(), ExamplePost.get_handler())
        .app_data(connection_pool)
}
