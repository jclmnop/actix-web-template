use crate::utils::spawn_app;
use actix_web_template::endpoint::example_auth;
use reqwest::StatusCode;

#[tokio::test]
async fn success_for_valid_credentials() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let address = test_app.address;

    let auth_response = client
        .get(format!("{address}{}", example_auth::PATH))
        .basic_auth(
            test_app.test_user.username,
            Some(test_app.test_user.password),
        )
        .send()
        .await
        .expect("Failed to execute request");

    assert!(auth_response.status().is_success());
}

#[tokio::test]
async fn unauthorised_for_invalid_password() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let address = test_app.address;

    let auth_response = client
        .get(format!("{address}{}", example_auth::PATH))
        .basic_auth(test_app.test_user.username, Some("wrong_password"))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::UNAUTHORIZED, auth_response.status());
}

#[tokio::test]
async fn unauthorised_for_invalid_username() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let address = test_app.address;

    let auth_response = client
        .get(format!("{address}{}", example_auth::PATH))
        .basic_auth("bob", Some(test_app.test_user.password))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(StatusCode::UNAUTHORIZED, auth_response.status());
}

macro_rules! timeit {
    ($code:block) => {{
        let now = ::std::time::Instant::now();
        $code
        let elapsed = now.elapsed();
        elapsed
    }};
}

// Check that user enumeration can't be performed with timing attacks
// #[tokio::test]
// async fn response_time_for_invalid_and_valid_username_is_similar() {
//     let test_app = spawn_app().await;
//     let client = reqwest::Client::new();
//     let address = test_app.address;
//
//     let time_with_valid_user = timeit!({
//         client
//             .get(format!("{address}{}", example_auth::PATH))
//             .basic_auth(&test_app.test_user.username, Some("invalid_password"))
//             .send()
//             .await
//             .expect("Failed to execute request");
//     })
//     .as_millis();
//
//     let time_with_invalid_user = timeit!({
//         client
//             .get(format!("{address}{}", example_auth::PATH))
//             .basic_auth("bob", Some("invalid_password"))
//             .send()
//             .await
//             .expect("Failed to execute request");
//     })
//     .as_millis();
//
//     println!(
//         "\tValid: {time_with_valid_user}ms\n\tInvalid: {time_with_invalid_user}ms"
//     );
//
//     // We accept 50% difference, it's much smaller in --release version anyway
//     assert!(time_with_invalid_user as f32 / time_with_valid_user as f32 > 0.5);
//     assert!(time_with_valid_user as f32 / time_with_invalid_user as f32 > 0.5);
// }
