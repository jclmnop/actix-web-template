use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use std::fmt::Write;

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    // This would need to be inserted into the login.html body
    let mut error_html = String::new();
    for m in flash_messages.iter().filter(|m| m.level() == Level::Error) {
        writeln!(error_html, "<p><i>{}</i></p>", m.content())
            .expect("Failed to write flash message.")
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            //TODO: must be a way to insert error_html into include_str!("login.html")
            r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Login</title>
    </head>
    <body>
        {error_html}
        <form action="/login" method="post">
            <label>Username
                <input
                    type="text"
                    placeholder="Enter Username"
                    name="username"
                >
            </label>

            <label>Password
                <input
                    type="password"
                    placeholder="Enter Password"
                    name="password"
                >
            </label>

            <button type="submit">Login</button>
        </form>
    </body>
</html>
            "#
        ))
}
