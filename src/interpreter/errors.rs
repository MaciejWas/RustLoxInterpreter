use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    ParsingError(String),
    EvaluatingError(String),
    TokenizingError(String),
    ScanningError(String),
    LogicError(String),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text: &String = match self {
            Self::ParsingError(t) => t,
            Self::TokenizingError(t) => t,
            Self::ScanningError(t) => t,
            Self::EvaluatingError(t) => t,
            Self::LogicError(t) => t,
        };
        write!(f, "{}", text)
    }
}

pub type LoxResult<A> = Result<A, LoxError>;

pub fn eval_err<A>(msg: String) -> LoxResult<A> {
    Err(LoxError::EvaluatingError(msg))
}

pub fn logic_err<A>(msg: String) -> LoxResult<A> {
    Err(LoxError::LogicError(msg))
}


