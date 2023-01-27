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
async fn unathorised_for_invalid_password() {
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
async fn unathorised_for_invalid_username() {
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
