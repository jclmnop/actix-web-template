use crate::routes::{example_get, example_post, health_check};
use actix_web::{web, Route};
use endpoint_derive::Endpoints;

/// GET and POST endpoints
#[derive(Endpoints)]
pub enum Endpoint {
    /// Return 200 if server is running
    #[endpoint(get, "/health_check", handler = "health_check")]
    HealthCheck,
    /// Submit data via HTML form and update/add a database entry
    #[endpoint(post, "/example_post", handler = "example_post")]
    ExamplePost,
    #[endpoint(get, "/example_get/{email}", handler = "example_get")]
    ExampleGet,
}
