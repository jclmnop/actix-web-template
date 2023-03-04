use crate::configuration::HmacSecret;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use crate::routes::{error_msg_to_query_string, verify_hmac_query};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

pub async fn login_form(
    query: Option<web::Query<QueryParams>>,
    secret: web::Data<HmacSecret>,
) -> HttpResponse {
    // This would need to be inserted into the login.html body
    let _error_html = match query {
        None => "".into(),
        Some(query) => {
            match query.0.verify(&secret) {
                Ok(error_msg) => {
                    // TODO: move formatting of html_errors to another function
                    format!("<p><i>{}</i></p>", htmlescape::encode_minimal(&error_msg))
                }
                Err(verification_error) => {
                    tracing::warn!(
                        error.message = %verification_error,
                        error.cause_chain = ?verification_error,
                        "Failed to verify query params using HMAC tag"
                    );
                    "".into()
                }
            }
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let query_string = error_msg_to_query_string(&self.error);
        verify_hmac_query(&self.tag, &query_string, secret)?;
        Ok(self.error)
    }
}
