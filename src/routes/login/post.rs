use crate::auth::validate_credentials;
use crate::domain::{Credentials, Parseable, Password, Username};
use crate::routes::AuthError;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct FormData {
    pub username: String,
    pub password: Secret<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {}

pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AuthError> {
    //TODO: better error handling?
    let credentials = Credentials {
        username: Username::parse(form.0.username)
            .map_err(|e| AuthError::InvalidCredentials(anyhow::Error::new(e)))?,
        password: Password::parse(form.0.password)
            .map_err(|e| AuthError::InvalidCredentials(anyhow::Error::new(e)))?,
    };
    match validate_credentials(credentials, &pool).await {
        Ok(_) => Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish()),
        Err(e) => Err(e),
    }
}
