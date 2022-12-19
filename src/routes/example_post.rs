use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn example_post(form: web::Form<FormData>) -> HttpResponse {
    println!("name: {}", form.name);
    println!("email: {}", form.email);
    HttpResponse::Ok().finish()
}
