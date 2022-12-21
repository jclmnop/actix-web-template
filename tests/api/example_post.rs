use crate::utils::spawn_app;
use actix_web_template::endpoint::Endpoint::ExamplePost;
use serde_urlencoded;

#[tokio::test]
async fn example_post_returns_200_for_valid_form_data() {
    const NAME: &str = "barry barryfield";
    const EMAIL: &str = "barry_bazza@barry.com";
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    let body = serde_urlencoded::to_string(&[("name", NAME), ("email", EMAIL)])
        .expect("Failed to urlencode string");

    let response = client
        .post(format!("{address}{}", ExamplePost.get_path()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("POST request failed");

    let status = response.status().as_u16();
    assert_eq!(200, status);

    //TODO: replace with a GET request
    let new_entry = sqlx::query!(r#"SELECT * FROM example;"#)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch updated table");

    assert_eq!(NAME, new_entry.name);
    assert_eq!(EMAIL, new_entry.email);
}

#[tokio::test]
async fn example_post_returns_400_for_invalid_form_data() {
    let test_app = spawn_app().await;
    let address = test_app.address;
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

#[tokio::test]
#[should_panic(expected = "Failed to fetch updated table: RowNotFound")]
async fn db_not_updated_after_failed_post_attempt() {
    let test_app = spawn_app().await;
    let address = test_app.address;
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

    //TODO: replace with a GET request
    let _ = sqlx::query!(r#"SELECT * FROM example;"#)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch updated table");
}
