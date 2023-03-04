use crate::configuration::HmacSecret;
use crate::domain::ParseError;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;
use sha2::Sha256;
use std::fmt::Formatter;

//TODO: must be a way to remove duplicate code re: `impl std::fmt::Debug`,
//      maybe a Derive(ErrorChain) macro?

#[derive(thiserror::Error)]
pub enum GetError {
    #[error(transparent)]
    InvalidEmail(#[from] ParseError),
    #[error("{0} not found.")]
    EmailNotFound(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(thiserror::Error)]
pub enum PostError {
    #[error(transparent)]
    InputValidationError(#[from] ParseError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(thiserror::Error)]
pub enum AuthError {
    #[error("Invalid username and/or password.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub type LoginError = InternalError<AuthError>;

impl ResponseError for GetError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetError::InvalidEmail(_) => StatusCode::BAD_REQUEST,
            GetError::EmailNotFound(_) => StatusCode::NOT_FOUND,
            GetError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for GetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PostError {
    fn status_code(&self) -> StatusCode {
        match self {
            PostError::InputValidationError(_) => StatusCode::BAD_REQUEST,
            PostError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for PostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            AuthError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_msg_to_query_string(error_msg: &String) -> String {
    format!("error={}", urlencoding::Encoded::new(error_msg))
}

/// Create a urlencoded error query param for the `error_msg`, tagged with an
/// HMAC tag generated using the `secret`.
pub fn hmac_tagged_error_query(
    secret: &HmacSecret,
    error_msg: String,
) -> String {
    // Convert error message to a valid query param
    let query_string = error_msg_to_query_string(&error_msg);

    // Use 'secret' to generate HMAC tag so error query param can be
    // verified as authentic to avoid XSS
    let hmac_tag = {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(secret.0.expose_secret().as_bytes())
                .expect("Error parsing HMAC");
        mac.update(query_string.as_bytes());
        mac.finalize().into_bytes()
    };

    format!("{query_string}&tag={hmac_tag:x}")
}

/// Verify that the HMAC `tag` encodes the `query_string` using the `secret`
pub fn verify_hmac_query(
    tag: &String,
    query_string: &String,
    secret: &HmacSecret,
) -> Result<(), anyhow::Error> {
    let tag = hex::decode(tag)?;

    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.0.expose_secret().as_bytes())?;
    mac.update(query_string.as_bytes());
    mac.verify_slice(&tag)?;

    Ok(())
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }
    Ok(())
}
