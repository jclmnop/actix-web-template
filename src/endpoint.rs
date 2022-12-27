use crate::routes::{example_get, example_post, health_check};
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
    /// Submit data via HTML form and update/add a database entry
    ExamplePost,
    ExampleGet,
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
            Endpoint::ExamplePost => EndpointRoute {
                path: "/example_post",
                handler: web::post().to(example_post),
            },
            Endpoint::ExampleGet => EndpointRoute {
                path: "/example_get/{email}",
                handler: web::get().to(example_get),
            },
        }
    }
}
