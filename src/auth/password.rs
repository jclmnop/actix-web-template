use crate::routes::AuthError;
use crate::telemetry::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::string::ToString;

// TODO: unit tests for compute_password_hash() and verify_password_hash()
// TODO: integration tests for validate_credentials()

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<String, AuthError> {
    let mut username: Option<String> = None;
    let mut stored_hash =
        compute_password_hash(Secret::new("password".to_string()))
            .context("Failed to hash default password")?;
    if let Some(stored_credentials) =
        get_stored_credentials(&credentials.username, pool).await?
    {
        username = Some(stored_credentials.username);
        stored_hash = stored_credentials.password;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(stored_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task")??;

    username
        .ok_or_else(|| anyhow::anyhow!("Unknown username"))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<Credentials>, anyhow::Error> {
    let credentials = sqlx::query!(
        r#"
        SELECT username, password
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Failed to query database for credentials")?
    .map(|row| Credentials {
        username: username.to_string(),
        password: Secret::new(row.password),
    });
    Ok(credentials)
}

#[tracing::instrument(
    name = "Verify hash",
    skip(expected_password_hash, received_password)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    received_password: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash =
        PasswordHash::new(expected_password_hash.expose_secret())
            .context("Failed to parse PHC string from stored password hash.")?;
    Argon2::default()
        .verify_password(
            received_password.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

fn compute_password_hash(
    password: Secret<String>,
) -> Result<Secret<String>, AuthError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)
            .context("Failed to create new params for hashing.")?,
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)
    .context("Failed to hash password")?
    .to_string();
    Ok(Secret::new(password_hash))
}
