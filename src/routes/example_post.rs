use crate::telemetry::init_request_trace;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct PostFormData {
    name: String,
    email: String,
}

pub async fn example_post(
    form: web::Form<PostFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // let request_id = Uuid::new_v4();
    // let request_span = tracing::info_span!(
    //     "Processing new POST request",
    //     %request_id,
    //     %form.name,
    //     %form.email,
    // );
    // let _request_span_guard = request_span.enter();
    init_request_trace!("Processing new POST request", %form.name, %form.email);
    let query_span = tracing::info_span!("Writing new data to database");

    match sqlx::query!(
        r#"
        INSERT INTO example (id, email, name, added_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
