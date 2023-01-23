use actix_web::HttpResponse;
use crate::init_request_trace;

/// Response 200 if server is running
pub async fn health_check() -> HttpResponse {
    init_request_trace!("Health check");
    HttpResponse::Ok().finish()
}
