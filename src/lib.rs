use crate::routes::ApiCommand::HealthCheck;
use actix_web::dev::{Server, ServiceFactory, ServiceRequest};
use actix_web::{App, Error, HttpResponse, HttpServer};
use std::net::TcpListener;

mod routes;

/// Run the server using the provided TCP Listener
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| add_routes(App::new()))
        .listen(listener)?
        .run();
    Ok(server)
}

/// Add all routes for API to the app
fn add_routes<T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>>(
    app: App<T>,
) -> App<T> {
    app.route(HealthCheck.get_path(), HealthCheck.get_route())
}

// Request handlers

/// Response 200 if server is running
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
