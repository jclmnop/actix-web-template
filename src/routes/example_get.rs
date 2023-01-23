use crate::telemetry::init_request_trace;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::Instrument;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ExampleGetResponse {
    pub email: String,
    pub name: String,
}

/// Get the data associated with an email address, or return 400
pub async fn example_get(
    email: web::Path<String>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    init_request_trace!("Processing new GET request", %email);
    let query_span = tracing::info_span!("Querying data from database");

    let email = email.to_string();
    let entry = sqlx::query!(
        r#"
        SELECT *
        FROM example
        WHERE email = $1
        "#,
        &email
    )
    .fetch_optional(pool.get_ref())
    .instrument(query_span)
    .await;

    match entry {
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
