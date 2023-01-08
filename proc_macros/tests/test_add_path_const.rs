use actix_web::{get, post, HttpResponse};
use proc_macros::add_path_const;

#[test]
fn example_get_compiles() {
    #[add_path_const]
    #[get("/path_get")]
    pub async fn example_get() -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    assert_eq!("/path_get", example_get::PATH);
}

#[test]
fn example_post_compiles() {
    #[add_path_const]
    #[post("/path_post")]
    pub async fn example_post() -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    assert_eq!("/path_post", example_post::PATH);
}

// TODO: trybuild tests to ensure error messages are correct
// #[test]
// #[should_panic]
// fn does_not_compile_without_valid_method_attr() {
//     #[add_path_const]
//     pub async fn example_get() -> HttpResponse {
//         HttpResponse::Ok().finish()
//     }
// }
