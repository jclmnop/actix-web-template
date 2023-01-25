pub trait Parseable<T>
where
    Self: Sized,
{
    fn parse(input: T) -> Result<Self, ParseError>;
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Input exceeds character limit of {0}.")]
    TooLong(usize),
    #[error("Input contains the following invalid characters: {0}")]
    ContainsInvalidChars(String),
    #[error("Input is empty.")]
    Empty,
    #[error("{0}")]
    Invalid(String),
}
