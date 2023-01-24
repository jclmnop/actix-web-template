use crate::domain::parse::{ParseError, Parseable};
use crate::domain::{Email, Name};
use crate::routes::PostFormData;

pub struct PostExampleData {
    name: Name,
    email: Email,
}

impl Parseable<PostFormData> for PostExampleData {
    fn parse(input: PostFormData) -> Result<Self, ParseError> {
        let name = Name::parse(input.name)?;
        let email = Email::parse(input.email)?;
        Ok(PostExampleData { name, email })
    }
}
