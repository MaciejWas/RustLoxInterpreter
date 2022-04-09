use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    ParsingError(String),
    TokenizingError(String),
    ScanningError(String)
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text: &String = match self {
            Self::ParsingError(t) => t,
            Self::TokenizingError(t) => t,
            Self::ScanningError(t) => t
        };
        write!(f, "{}", text)
    }
}

pub type LoxResult<A> = Result<A, LoxError>;
