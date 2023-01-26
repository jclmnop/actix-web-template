use crate::routes::AuthError;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{
    Algorithm, Argon2, Error, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

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
