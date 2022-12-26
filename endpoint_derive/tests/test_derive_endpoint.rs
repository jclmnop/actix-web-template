use actix_web::{web, Route, HttpResponse};
use endpoint_derive::Endpoints;

pub async fn example_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn example_post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Endpoints)]
enum Endpoint {
    #[get]
    #[endpoint_path = "/hello"]
    #[handler = "example_get"]
    GetEndpoint,
    #[post]
    #[endpoint_path = "/hello"]
    #[handler = "example_post"]
    PostEndpoint,
}

#[test]
fn it_works() {
    assert_eq!(Endpoint::GetEndpoint.get_path(), "/hello");
    assert_eq!(Endpoint::PostEndpoint.get_path(), "/hello");
}