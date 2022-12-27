use actix_web::{web, HttpResponse, Route};
use endpoint_derive::Endpoints;

pub async fn example_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn example_post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Endpoints)]
enum Endpoint {
    #[endpoint(get, "/hello", handler = "example_get")]
    GetEndpoint,
    #[endpoint(post, "/hello_post", handler = "example_post")]
    PostEndpoint,
}

#[test]
fn it_works() {
    assert_eq!(Endpoint::GetEndpoint.get_path(), "/hello");
    assert_eq!(Endpoint::PostEndpoint.get_path(), "/hello_post");
}
