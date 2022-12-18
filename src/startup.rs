use crate::routes::Endpoint::HealthCheck;
use actix_web::dev::{Server, ServiceFactory, ServiceRequest};
use actix_web::{App, Error, HttpServer};
use std::net::TcpListener;

/// Run the server using the provided TCP Listener
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| build_app(App::new()))
        .listen(listener)?
        .run();
    Ok(server)
}

/// Build the actix-web application
fn build_app<T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>>(
    app: App<T>,
) -> App<T> {
    app.route(HealthCheck.get_path(), HealthCheck.get_handler())
}
