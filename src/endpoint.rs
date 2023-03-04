use crate::auth::validate_request_auth;
use crate::configuration::HmacSecret;
use crate::routes::{
    AuthError, LoginError, PostError,
};
use crate::{init_request_trace, routes};
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
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
    form: web::Form<routes::PostExampleForm>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostError> {
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

/// Response 200 if 'Basic' authorisation credentials are valid
#[add_path_const]
#[get("/example_auth")]
pub async fn example_auth(
    request: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    init_request_trace!("Validate Basic credentials");
    validate_request_auth(request, &pool).await?;
    Ok(HttpResponse::Ok().finish())
}

#[add_path_const]
#[get("/home")]
pub async fn home() -> HttpResponse {
    init_request_trace!("Home Page");
    routes::home().await
}

#[add_path_const]
#[get("/login")]
pub async fn login_form(
    query: Option<web::Query<routes::login::QueryParams>>,
    secret: web::Data<HmacSecret>,
) -> HttpResponse {
    init_request_trace!("Get login form");
    routes::login::login_form(query, secret).await
}

#[add_path_const]
#[post("/login")]
pub async fn login(
    form: web::Form<routes::login::FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, LoginError> {
    init_request_trace!("Login Attempt", %form.username);
    let login_result = routes::login::login(form, pool).await;
    match login_result {
        Ok(response) => Ok(response),
        Err(e) => {
            let response = HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .finish();

            Err(InternalError::from_response(e, response))
        }
    }
}
