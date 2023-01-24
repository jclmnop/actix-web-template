use crate::domain::util::{ParseError, Parseable};
use validator::validate_email;

#[derive(Debug)]
pub struct Email(String);

impl Parseable for Email {
    fn parse(s: String) -> Result<Self, ParseError> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(ParseError::Invalid(format!("{} is not a valid email", &s)))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use super::Parseable;
    use claims::{assert_err};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_email_is_invalid() {
        let email = " ".to_string();
        assert_err!(Email::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_invalid() {
        let email = "johnsmithataol.com".to_string();
        assert_err!(Email::parse(email));
    }

    #[test]
    fn email_missing_subject_is_invalid() {
        let email = "@aol.com".to_string();
        assert_err!(Email::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = rand::thread_rng();
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
    }
}