use actix_web_template::startup::run;
use std::net::TcpListener;

/// Spawn an instance of the app using a random available port and return the
/// address used, including the selected port.
pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
