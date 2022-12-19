use crate::routes::health_check;
use actix_web::{web, Route};

/// Contains all information required to add a route for a new endpoint to
/// an instance of `actix_web::App`
pub struct EndpointRoute {
    /// Path for this endpoint
    path: &'static str,
    /// Request handler
    handler: Route,
}

/// GET and POST endpoints
pub enum Endpoint {
    /// Return 200 if server is running
    HealthCheck,
}

impl Endpoint {
    /// Path for this request
    pub fn get_path(&self) -> &'static str {
        self.get_route().path
    }

    /// Request handler
    pub fn get_handler(&self) -> Route {
        self.get_route().handler
    }

    fn get_route(&self) -> EndpointRoute {
        match self {
            Endpoint::HealthCheck => EndpointRoute {
                path: "/health_check",
                handler: web::get().to(health_check),
            },
        }
    }
}
