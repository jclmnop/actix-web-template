use crate::utils::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn error_flash_msg_is_set_on_failed_login_attempt() {
    const ERROR_MSG: &str = "Invalid username and/or password.";
    let error_html = format!(r#"<p><i>{ERROR_MSG}</i></p>"#);

    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "incorrect_username",
        "password": "incorrect_password"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");

    // Flash cookie is set correctly
    let flash_cookie =
        response.cookies().find(|c| c.name() == "_flash").unwrap();
    assert_eq!(flash_cookie.value(), ERROR_MSG);

    // HTML error msg is rendered correctly
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(&error_html));

    // HTML error msg disappears on refresh
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(&error_html));
}
