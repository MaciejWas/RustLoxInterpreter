use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    ParsingError(String, usize),
    EvaluatingError(String, usize),
    TokenizingError(String, usize),
    ScanningError(String, usize),
    LogicError(String, usize),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (text, pos) = match self {
            Self::ParsingError(t, p)    => (t, p),
            Self::TokenizingError(t, p) => (t, p),
            Self::ScanningError(t, p)   => (t, p),
            Self::EvaluatingError(t, p) => (t, p),
            Self::LogicError(t, p)      => (t, p),
        };
        write!(f, "We have a problem!\n\t{}\n\t (at pos {}) ", text, pos)
    }
}

pub type LoxResult<A> = Result<A, LoxError>;

pub fn eval_err<A>(msg: String, pos: usize) -> LoxResult<A> {
    Err(LoxError::EvaluatingError(msg, pos))
}

pub fn logic_err<A>(msg: String, pos: usize) -> LoxResult<A> {
    Err(LoxError::LogicError(msg, pos))
}


