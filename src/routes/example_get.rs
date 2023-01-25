use crate::domain::{Email, ParseError, Parseable};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExampleGetResponse {
    pub email: String,
    pub name: String,
}

struct Record {
    name: String,
    email: String,
}

#[derive(thiserror::Error, Debug)]
pub enum GetError {
    #[error("Invalid email.")]
    InvalidEmail(#[from] ParseError),
    #[error("{0} not found.")]
    EmailNotFound(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

/// Get the data associated with an email address, or return 400
pub async fn example_get(
    email: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, GetError> {
    let email = Email::parse(email.into_inner())?;
    let response = read_db(&email, &pool)
        .await
        .context("Failed to read database.")?;
    match response {
        None => Err(GetError::EmailNotFound(email.as_ref().to_string())),
        Some(record) => Ok(HttpResponse::Ok().json(ExampleGetResponse {
            name: record.name,
            email: record.email,
        })),
    }
}

#[tracing::instrument(name = "Writing new data to database", skip(email, pool))]
async fn read_db(
    email: &Email,
    pool: &PgPool,
) -> Result<Option<Record>, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        SELECT *
        FROM example
        WHERE email = $1
        "#,
        &email.as_ref()
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(match record {
        Some(r) => Some(Record {
            email: r.email,
            name: r.name,
        }),
        None => None,
    })
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
