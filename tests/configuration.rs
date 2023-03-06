use actix_web_template::configuration::Settings;

#[test]
fn config_loads_without_error() {
    let settings =
        Settings::get_config().expect("Failed to load configuration");
    assert_eq!(
        format!("{}:{}", settings.app.host, settings.app.port),
        settings.get_address()
    );
}
