use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

//TODO:
//  - validate email/name with domain stuff
//      - PostExampleData { name: PostName, email: PostEmail }
//      - PostName::parse(name: String) -> Result<Self, anyhow::Error>
//      - PostEmail::parse(email: String) -> Result<Self, anyhow::Error>
//  - ExamplePostError
//      - ValidationError
//      - DatabaseError
//  - error chaining?
//  - impl ResponseError for ExamplePostError
//  - Result<HttpResponse, ExamplePostError>
//  - anyhow::Error

#[derive(serde::Deserialize)]
pub struct PostFormData {
    pub name: String,
    pub email: String,
}

pub async fn example_post(
    form: web::Form<PostFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match write_db(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Writing new data to database", skip(form, pool))]
async fn write_db(
    form: &PostFormData,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO example (id, email, name, added_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
