use crate::domain::{ParseError, Parseable};
use actix_web::http::header::HeaderMap;
use anyhow::{anyhow, Context};
use base64::engine::general_purpose::STANDARD as base64_decoder;
use base64::Engine;
use itertools::Itertools;
use secrecy::{ExposeSecret, Secret};
use std::str::FromStr;
use std::string::ToString;

pub struct Credentials {
    pub username: Username,
    pub password: Password,
}

pub struct Username(String);

pub struct Password(Secret<String>);

impl Credentials {
    pub fn decode_from_basic_authentication_header(
        headers: &HeaderMap,
    ) -> Result<Self, anyhow::Error> {
        let header_value = headers
            .get("Authorization")
            .context("The 'Authorization' header was missing")?
            .to_str()
            .context("'Authorization' header was not a valid UTF-8 string")?;
        let base64_encoded_credentials = header_value
            .strip_prefix("Basic ")
            .context("Authorization scheme was not 'Basic'")?;
        let decoded_credentials = base64_decoder
            .decode(base64_encoded_credentials)
            .context("Failed to base64-decode 'Basic' credentials")?;
        let decoded_credentials = String::from_utf8(decoded_credentials)
            .context("Decoded credentials string is invalid UTF-8")?;

        Self::from_str(&decoded_credentials)
    }
}

impl FromStr for Credentials {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut credentials = s.splitn(2, ':');
        let username = credentials
            .next()
            .ok_or_else(|| {
                anyhow!("Username not provided in 'Basic' auth header.")
            })?
            .to_string();
        let password = credentials
            .next()
            .ok_or_else(|| {
                anyhow!("Password not provided in 'Basic' auth header.")
            })?
            .to_string();

        Ok(Self {
            username: Username::parse(username)
                .context("Invalid username format.")?,
            password: Password::parse(password)
                .context("Invalid password format.")?,
        })
    }
}

impl Username {
    const MAX_LENGTH: usize = 50;
    const LEGAL_CHARS: &'static str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
}

impl Parseable<String> for Username {
    fn parse(s: String) -> Result<Self, ParseError> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.chars().count() > Self::MAX_LENGTH;
        let forbidden_chars: Vec<char> = s
            .chars()
            .unique()
            .filter(|&c| !Self::LEGAL_CHARS.chars().contains(&c))
            .collect();
        let contains_forbidden_chars = !forbidden_chars.is_empty();

        if is_empty {
            Err(ParseError::Empty)
        } else if is_too_long {
            Err(ParseError::TooLong(Self::MAX_LENGTH))
        } else if contains_forbidden_chars {
            Err(ParseError::ContainsInvalidChars(format!(
                "{forbidden_chars:?}"
            )))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Password {
    const MAX_LENGTH: usize = 50;
    const MIN_LENGTH: usize = 10;
    const LEGAL_CHARS: &'static str = r#"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 "!#$%&'()*+,-./:;<=>?@[\]^_`{|}~"#;
}

impl Parseable<String> for Password {
    fn parse(s: String) -> Result<Self, ParseError> {
        let is_too_short = s.chars().count() < Self::MIN_LENGTH;
        let is_too_long = s.chars().count() > Self::MAX_LENGTH;
        let forbidden_chars: Vec<char> = s
            .chars()
            .unique()
            .filter(|&c| !Self::LEGAL_CHARS.chars().contains(&c))
            .collect();
        let contains_forbidden_chars = !forbidden_chars.is_empty();

        if is_too_short {
            Err(ParseError::Empty)
        } else if is_too_long {
            Err(ParseError::TooLong(Self::MAX_LENGTH))
        } else if contains_forbidden_chars {
            Err(ParseError::ContainsInvalidChars(format!(
                "{forbidden_chars:?}"
            )))
        } else {
            Ok(Self(Secret::new(s)))
        }
    }
}

impl Parseable<Secret<String>> for Password {
    fn parse(input: Secret<String>) -> Result<Self, ParseError> {
        let password = input.expose_secret().clone();
        Password::parse(password)
    }
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}
