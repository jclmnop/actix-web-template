use secrecy::ExposeSecret;
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

#[test]
fn config_env_vars_expand_without_error() {
    let env_var = "HMAC_SECRET";
    let env_val = "invalid-hmac-key-for-config-test";
    std::env::set_var(env_var, env_val);

    let settings =
        Settings::get_config().expect("Failed to load config");

    assert_eq!(
        settings.app.hmac_secret.expose_secret(),
        &String::from(env_val)
    );
}

#[test]
fn int_config_env_vars_correctly_parsed() {
    let env_var = "DB_PORT";
    let env_val = 8000;
    std::env::set_var(env_var, env_val.to_string());

    let settings =
        Settings::get_config().expect("Failed to load config");

    assert_eq!(
        settings.database.port,
        env_val
    );
}
