use actix_web_template::configuration::Settings;

#[test]
fn config_loads_without_error() {
    let settings = Settings::get_config().expect("Failed to load config");
    assert_eq!(
        format!("{}:{}", settings.host, settings.application_port),
        settings.get_address()
    );
}
