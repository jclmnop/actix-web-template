use crate::domain::parse::{ParseError, Parseable};
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

const MAX_LENGTH: usize = 256;
const FORBIDDEN_CHARS: [char; 10] =
    ['/', '@', '\\', '(', ')', '>', '<', '{', '}', '='];

#[derive(Debug)]
pub struct Name(String);

impl Parseable<String> for Name {
    fn parse(s: String) -> Result<Self, ParseError> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > MAX_LENGTH;
        let forbidden_chars: Vec<char> = s
            .chars()
            .unique()
            .filter(|&c| FORBIDDEN_CHARS.contains(&c))
            .collect();
        let contains_forbidden_chars = !forbidden_chars.is_empty();

        if is_empty {
            Err(ParseError::Empty)
        } else if is_too_long {
            Err(ParseError::TooLong(MAX_LENGTH))
        } else if contains_forbidden_chars {
            Err(ParseError::ContainsInvalidChars(format!(
                "{forbidden_chars:?}"
            )))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn long_name_is_invalid() {
        let name = "a".repeat(MAX_LENGTH + 1);
        assert_err!(Name::parse(name));
    }

    #[test]
    fn shorter_name_is_valid() {
        let name = "a".repeat(MAX_LENGTH);
        assert_ok!(Name::parse(name));
    }

    #[test]
    fn character_limit_correctly_counts_graphemes() {
        let name = "씨".repeat(MAX_LENGTH);
        assert_ok!(Name::parse(name));

        let name = "씨".repeat(MAX_LENGTH + 1);
        assert_err!(Name::parse(name));
    }

    #[test]
    fn empty_string_is_invalid() {
        let name = "".to_string();
        assert_err!(Name::parse(name));

        let name = "  ".to_string();
        assert_err!(Name::parse(name));
    }

    #[test]
    fn verboten_chars_are_invalid() {
        for verboten_char in FORBIDDEN_CHARS {
            let name = format!("DAS IST {} VERBOTEN", &verboten_char);
            assert_err!(Name::parse(name));
        }
    }
}
