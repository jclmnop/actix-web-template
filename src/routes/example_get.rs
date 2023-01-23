use actix_web::{web, HttpResponse};
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
) -> HttpResponse {
    match read_db(&email, &pool).await {
        Ok(response) => match response {
            None => HttpResponse::NotFound().finish(),
            Some(record) => HttpResponse::Ok().json(ExampleGetResponse {
                name: record.name,
                email: record.email,
            }),
        },
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Writing new data to database", skip(email, pool))]
async fn read_db(
    email: &String,
    pool: &PgPool,
) -> Result<Option<Record>, sqlx::Error> {
    let record = sqlx::query!(
        r#"
        SELECT *
        FROM example
        WHERE email = $1
        "#,
        &email
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
