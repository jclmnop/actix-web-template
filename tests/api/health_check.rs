use crate::utils::spawn_app;
use actix_web_template::endpoint::Endpoints;
use actix_web_template::endpoint::PublicEndpoint::HealthCheck;

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{address}{}", HealthCheck.get_path()))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
