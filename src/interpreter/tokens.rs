use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::ErrType::TokenizingErr;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::LoxError;
use regex::Regex;
use std::fmt;

pub use kwds::Kwd;
pub use lox_values::LoxValue;
pub use puncts::Punct;

pub mod kwds;
pub mod lox_values;
pub mod puncts;

const VARIABLE_RE: &str = r"^[a-zA-Z_'][a-zA-Z0-9_']*$";
const NUMBER_RE: &str = r"^[0-9]+$";

#[derive(Clone, Eq, PartialEq)]
pub enum Token {
    PunctToken(Punct, usize),
    KwdToken(Kwd, usize),
    ValueToken(LoxValue, usize),
    IdentifierToken(String, usize),
}

impl Token {
    pub fn from_string(string: String, pos: usize) -> LoxResult<Self> {
        if string.eq("True") {
            Ok(Self::ValueToken(LoxValue::from(true), pos))
        } else if string.eq("False") {
            Ok(Self::ValueToken(LoxValue::from(false), pos))
        } else if Kwd::is_valid(&string) {
            let kwd = Kwd::from(&string, pos)?;
            Ok(Self::KwdToken(kwd, pos))
        } else if Regex::new(VARIABLE_RE).unwrap().is_match(&string) {
            Ok(Self::IdentifierToken(string, pos))
        } else if Regex::new(NUMBER_RE).unwrap().is_match(&string) {
            let number: i32 = string.parse().expect("Failed to parse string as number");
            Ok(Self::ValueToken(LoxValue::from(number), pos))
        } else if string.starts_with('\"') && string.ends_with('\"') {
            Ok(Self::ValueToken(LoxValue::from(string), pos))
        } else {
            Self::tokenizing_err(format!("Did not understand {}", string), pos)
        }
    }

    pub fn pos(&self) -> usize {
        match self {
            Self::PunctToken(_, pos) => *pos,
            Self::KwdToken(_, pos) => *pos,
            Self::ValueToken(_, pos) => *pos,
            Self::IdentifierToken(_, pos) => *pos,
        }
    }

    pub fn as_punct(&self) -> LoxResult<Punct> {
        match self {
            Self::PunctToken(punct, _) => Ok(punct.clone()),
            _ => LoxError::new_err(format!("{:?} is not a punct", self), self.pos(), LogicError),
        }
    }

    pub fn as_lox_value(&self) -> LoxResult<LoxValue> {
        match self {
            Self::ValueToken(lox_val, _) => Ok(lox_val.clone()),
            _ => LoxError::new_err(
                format!("{:?} is not a lox value", self),
                self.pos(),
                LogicError,
            ),
        }
    }

    pub fn tokenizing_err<A>(text: String, pos: usize) -> LoxResult<A> {
        LoxError::new_err(text.to_string(), pos, TokenizingErr)
    }

    pub fn can_be_unary_op(&self) -> bool {
        match self {
            Self::PunctToken(p, _) => [Punct::Minus].contains(p),
            _ => false,
        }
    }

    pub fn satisfies_or<Pred, ErrMaker>(&self, pred: Pred, err: ErrMaker) -> LoxResult<()>
    where
        Pred: Fn(&Self) -> bool,
        ErrMaker: Fn(&Token) -> LoxError,
    {
        if pred(self) {
            return Ok(());
        }

        Err(err(self))
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::ValueToken(x, _) => write!(f, "{:?}", x,),
            Self::IdentifierToken(x, _) => write!(f, "{:?}", x,),
            Self::KwdToken(x, _) => write!(f, "{:?}", x),
            Self::PunctToken(x, _) => write!(f, "{:?}", x),
        }
    }
}

pub trait Equals<A> {
    fn equals(&self, a: &A) -> bool;
}

impl Equals<Kwd> for Token {
    fn equals(&self, kwd: &Kwd) -> bool {
        match self {
            Self::KwdToken(k, _) => k == kwd,
            _ => false,
        }
    }
}

impl Equals<Punct> for Token {
    fn equals(&self, punct: &Punct) -> bool {
        match self {
            Self::PunctToken(p, _) => p == punct,
            _ => false,
        }
    }
}

pub trait Tokenizable {
    fn at(self, pos: usize) -> Token;
}

impl Tokenizable for Punct {
    fn at(self, pos: usize) -> Token {
        Token::PunctToken(self, pos)
    }
}

impl Tokenizable for String {
    fn at(self, pos: usize) -> Token {
        Token::from_string(self, pos).unwrap() // unsafe lol
    }
}

impl Tokenizable for LoxValue {
    fn at(self, pos: usize) -> Token {
        Token::ValueToken(self, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use quickcheck::quickcheck;

    quickcheck! {
        fn quickcheck_token_from(s: String) -> bool {
            let _ = Token::from_string(s, 0);
            true
        }
    }
}
