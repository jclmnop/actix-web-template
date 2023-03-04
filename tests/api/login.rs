use crate::utils::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn error_flash_msg_is_set_on_failed_login_attempt() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "incorrect_username",
        "password": "incorrect_password"
    });
    let response = app.post_login(&login_body).await;

    assert_is_redirect_to(&response, "/login");
}
