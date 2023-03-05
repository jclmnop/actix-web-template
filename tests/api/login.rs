use crate::utils::{assert_is_redirect_to, spawn_app};
use actix_web_template::endpoint::{admin_dashboard, login};
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

    assert_is_redirect_to(&response, login::PATH);

    // HTML error msg is rendered correctly
    let html_page = app.get_login_html().await;
    assert!(html_page.contains(&error_html));

    // HTML error msg disappears on refresh
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(&error_html));
}

#[tokio::test]
async fn redirect_to_admin_dashboard_on_login_success() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, admin_dashboard::PATH);

    let html_page = app.get_admin_dashboard().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
