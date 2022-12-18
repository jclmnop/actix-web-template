use crate::health_check;
use actix_web::{web, Route};

/// Contains all information required to add a route to
/// an instance of `actix_web::App`
pub struct ApiRoute {
    /// Path for this request
    path: &'static str,
    /// Request handler
    route: Route,
}

/// GET and POST commands to be parsed by the API
pub enum ApiCommand {
    /// Return 200 if server is running
    HealthCheck,
}

impl ApiCommand {
    /// Path for this request
    pub fn get_path(&self) -> &'static str {
        self.get_api_route().path
    }

    /// Request handler
    pub fn get_route(&self) -> Route {
        self.get_api_route().route
    }

    fn get_api_route(&self) -> ApiRoute {
        match self {
            ApiCommand::HealthCheck => ApiRoute {
                path: "/health_check",
                route: web::get().to(health_check),
            },
        }
    }
}
