use crate::utils::spawn_app;
use actix_web_template::endpoint::Endpoint::ExamplePost;
use serde_urlencoded;

#[tokio::test]
async fn example_post_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = serde_urlencoded::to_string(&[
        ("name", "barry barryfield"),
        ("email", "barry_bazza@barry.com"),
    ])
    .expect("Failed to urlencode string");

    let response = client
        .post(format!("{address}{}", ExamplePost.get_path()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("POST request failed");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn example_post_returns_400_for_invalid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let invalid_bodies = vec![
        (
            serde_urlencoded::to_string(&[("name", "barry barryfield")]).unwrap(),
            "missing email",
        ),
        (
            serde_urlencoded::to_string(&[("email", "barry@barry.com")]).unwrap(),
            "missing name",
        ),
        (String::from(""), "missing both name and email"),
        (
            String::from("name:barry,email:barry@barry.com"),
            "form data is incorrectly encoded",
        ),
    ];

    for (body, error_msg) in invalid_bodies {
        let response = client
            .post(format!("{address}{}", ExamplePost.get_path()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("POST request failed");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 bad request when {}",
            error_msg
        );
    }
}
