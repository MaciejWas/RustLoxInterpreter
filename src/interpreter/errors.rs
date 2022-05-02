use std::fmt;

pub enum ErrType {
    ParsingErr, EvalErr, TokenizingErr, ScanningErr, LogicError
}

#[derive(Debug)]
pub enum LoxError {
    msg: String,
    err_type: ErrType,
    pos: usize
}

impl LoxError {
    pub fn generate_err_msg(&self, text: &String) -> String {
        let prelude: String = text[(pos - 10)..(pos+10)]
            .clone();

        let err_msg: String = format!("{}", self);

        prelude.add(err_msg)
    }

    pub fn new_err<A>(msg: String, pos: usize, err_type: ErrType) -> LoxResult<A> {
        Ok({ msg: msg, err_type: err_type, pos: pos })
    }
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