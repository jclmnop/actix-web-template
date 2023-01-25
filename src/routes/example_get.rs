use crate::domain::{Email, Parseable};
use crate::routes::GetError;
use actix_web::{web, HttpResponse};
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

#[tracing::instrument(name = "Reading data from database", skip(email, pool))]
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
    .await?;

    Ok(match record {
        Some(r) => Some(Record {
            email: r.email,
            name: r.name,
        }),
        None => None,
    })
}
