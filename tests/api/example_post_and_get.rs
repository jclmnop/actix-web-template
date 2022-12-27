use crate::utils::spawn_app;
use actix_web_template::endpoint::Endpoint::{ExampleGet, ExamplePost};
use actix_web_template::endpoint::Endpoints;
use actix_web_template::routes::ExampleGetResponse;
use serde_urlencoded;
use strfmt::strfmt;
use urlencoding;

//TODO: break this test down into 3 tests
// (will only work if they can somehow be peformed sequentially with same db instance)
#[tokio::test]
async fn example_post_returns_200_for_valid_form_data_and_get_returns_new_data() {
    const NAME: &str = "barry barryfield";
    const EMAIL: &str = "barry_bazza@barry.com";
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    let post_body = serde_urlencoded::to_string(&[("name", NAME), ("email", EMAIL)])
        .expect("Failed to urlencode POST request");

    let post_response = client
        .post(format!("{address}{}", ExamplePost.get_path()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await
        .expect("POST request failed");

    let status = post_response.status().as_u16();
    assert_eq!(200, status);

    let new_entry = sqlx::query!(r#"SELECT * FROM example;"#)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch updated table");

    assert_eq!(NAME, new_entry.name);
    assert_eq!(EMAIL, new_entry.email);

    let email = EMAIL;
    let text_response = client
        .get(format!(
            "{address}{}",
            strfmt!(ExampleGet.get_path(), email).unwrap()
        ))
        .send()
        .await
        .expect("GET request failed")
        .text()
        .await
        .expect("Failed to parse text from get response");

    println!("{:?}", text_response);
    let parsed_response: ExampleGetResponse =
        serde_json::from_str(&*text_response).expect("Error parsing json from text");
    assert_eq!(NAME, parsed_response.name);
    assert_eq!(EMAIL, parsed_response.email);
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

    let _ = sqlx::query!(r#"SELECT * FROM example;"#)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch updated table");
}

#[tokio::test]
async fn example_get_returns_404_for_nonexistant_data() {
    const EMAIL: &str = "does_not_exist@foo.com";
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "{address}{}",
            strfmt!(ExampleGet.get_path(), email => EMAIL).unwrap()
        ))
        .send()
        .await
        .expect("GET request failed");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn example_get_returns_404_for_basic_injection() {
    const BAD_EMAIL: &str = "barry_bazza@barry.com OR 1=1;";
    const NAME: &str = "barry barryfield";
    const EMAIL: &str = "barry_bazza@barry.com";
    let bad_email: String = urlencoding::encode(BAD_EMAIL).into_owned();
    let test_app = spawn_app().await;
    let address = test_app.address;
    let client = reqwest::Client::new();

    let post_body = serde_urlencoded::to_string(&[("name", NAME), ("email", EMAIL)])
        .expect("Failed to urlencode POST request");

    let post_response = client
        .post(format!("{address}{}", ExamplePost.get_path()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(post_body)
        .send()
        .await
        .expect("POST request failed");

    let status = post_response.status().as_u16();
    assert_eq!(200, status);

    let new_entry = sqlx::query!(r#"SELECT * FROM example;"#)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch updated table");

    assert_eq!(NAME, new_entry.name);
    assert_eq!(EMAIL, new_entry.email);

    let bad_request_response = client
        .get(format!(
            "{address}{}",
            strfmt!(ExampleGet.get_path(), email => bad_email).unwrap()
        ))
        .send()
        .await
        .expect("GET request failed");

    assert_eq!(404, bad_request_response.status().as_u16());
}
