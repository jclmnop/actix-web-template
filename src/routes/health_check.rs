use actix_web::HttpResponse;

/// Response 200 if server is running
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
