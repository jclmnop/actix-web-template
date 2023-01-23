use crate::{init_request_trace, routes};
use actix_web::{get, post, web, Responder};
use proc_macros::add_path_const;
use sqlx::PgPool;

/// Get the data associated with an email address, or return 400
#[add_path_const]
#[get("/example_get/{email}")]
pub async fn example_get(
    email: web::Path<String>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    init_request_trace!("Processing new GET request", %email);
    routes::example_get(email, pool).await
}

/// Add a new entry to database via urlencoded web form
#[add_path_const]
#[post("/example_post")]
pub async fn example_post(
    form: web::Form<routes::PostFormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    init_request_trace!("Processing new POST request", %form.name, %form.email);
    routes::example_post(form, pool).await
}

/// Response 200 if server is running
#[add_path_const]
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    init_request_trace!("Health check");
    routes::health_check().await
}
