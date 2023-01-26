use secrecy::Secret;
use crate::routes::AuthError;

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}



