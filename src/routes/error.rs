use std::fmt::Formatter;
use crate::domain::ParseError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::header::LOCATION;

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

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Invalid username and/or password.")]
    InvalidCredentials(#[source] anyhow::Error),//AuthError::InvalidCredentials),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}

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

    // fn error_response(&self) -> HttpResponse {
    //     let mut response = HttpResponse::new(self.status_code());
    //     match self {
    //         AuthError::InvalidCredentials(_) => {
    //             let header_value =
    //                 HeaderValue::from_str(r#"Basic realm="publish""#)
    //                     .map_err(|e| {
    //                         return AuthError::UnexpectedError(anyhow!(
    //                             "Failed to parse auth header for response: {e}"
    //                         ))
    //                         .error_response();
    //                     })
    //                     .unwrap();
    //             response.headers_mut().insert(
    //                 actix_web::http::header::WWW_AUTHENTICATE,
    //                 header_value,
    //             );
    //             response
    //         }
    //         _ => response,
    //     }
    // }
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        StatusCode::SEE_OTHER
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header((LOCATION, "/login"))
            .finish()
    }

}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<AuthError> for LoginError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::InvalidCredentials(e) => {Self::InvalidCredentials(e)}
            AuthError::UnexpectedError(e) => {Self::UnexpectedError(e)}
        }
    }
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
