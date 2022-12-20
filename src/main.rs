use actix_web_template::startup::run;
use std::net::TcpListener;
use actix_web_template::configuration::Settings;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = Settings::get_config().expect("Failed to load config");
    let listener = TcpListener::bind(configuration.get_address())?;
    run(listener)?.await
}
