use std::fmt;

pub enum LoxError {
    ParsingError(String)
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::ParsingError(text) => text
        };
        write!(f, "Hey, you f$#@ed up! \n\t see -> {}", text)
    }
}
