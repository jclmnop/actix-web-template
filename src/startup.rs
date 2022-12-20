use crate::endpoint::Endpoint::{ExamplePost, HealthCheck};
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
fn build_app<T>(app: App<T>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    app.route(HealthCheck.get_path(), HealthCheck.get_handler())
        .route(ExamplePost.get_path(), ExamplePost.get_handler())
}
