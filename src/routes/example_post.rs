use crate::domain::Parseable;
use crate::domain::{self, ParseError, PostData};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

//TODO:
//  - error chaining?
//  - Same for GET

#[derive(serde::Deserialize)]
pub struct PostExampleForm {
    pub name: String,
    pub email: String,
}

#[derive(thiserror::Error, Debug)]
pub enum PostError {
    #[error(transparent)]
    InputValidationError(#[from] ParseError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub async fn example_post(
    form: web::Form<PostExampleForm>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let post_data = form
        .0
        .try_into()
        .context("Failed to parse data from form.")?;
    write_db(&post_data, &pool)
        .await
        .context("Failed to write to db.")?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Writing new data to database", skip(form, pool))]
async fn write_db(form: &PostData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO example (id, email, name, added_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email.as_ref(),
        form.name.as_ref(),
        Utc::now(),
    )
    .execute(pool)
    .await?;
    Ok(())
}

impl TryFrom<PostExampleForm> for PostData {
    type Error = ParseError;

    fn try_from(value: PostExampleForm) -> Result<Self, Self::Error> {
        let name = domain::Name::parse(value.name)?;
        let email = domain::Email::parse(value.email)?;
        Ok(PostData { name, email })
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
