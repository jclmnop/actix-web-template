use crate::domain::Parseable;
use crate::domain::{self, ParseError, PostData};
use crate::routes::PostError;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

//TODO:
//  - error chaining?

#[derive(serde::Deserialize)]
pub struct PostExampleForm {
    pub name: String,
    pub email: String,
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
