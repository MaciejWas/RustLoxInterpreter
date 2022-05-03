use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::LoxError;
use crate::interpreter::errors::ErrType::TokenizingErr;
use crate::interpreter::errors::LoxResult;
use std::fmt;
use regex::Regex;

pub use kwds::Kwd;
pub use lox_values::LoxValue;
pub use puncts::Punct;

pub mod kwds;
pub mod lox_values;
pub mod puncts;

const VARIABLE_RE: &str = r"^[a-zA-Z_'][a-zA-Z0-9_']*$"; 
const NUMBER_RE: &str = r"[0-9]+";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    PunctToken(Punct, usize),
    KwdToken(Kwd, usize),
    ValueToken(LoxValue, usize),
    IdentifierToken(String, usize)
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
        } else if string.starts_with("\"") && string.ends_with("\"") {
            Ok(Self::ValueToken(LoxValue::from(string), pos))
        } else {
            Self::tokenizing_err(format!("Did not understand {}", string), pos)
        }
    }

    pub fn pos(&self) -> usize {
        match self {
            Self::PunctToken(_, pos) => *pos,
            Self::KwdToken(_, pos)   => *pos,
            Self::ValueToken(_, pos) => *pos,
            Self::IdentifierToken(_, pos) => *pos,
        }
    }

    pub fn as_punct(&self) -> LoxResult<Punct> {
        match self {
            Self::PunctToken(punct, _) => Ok(punct.clone()),
            _ => LoxError::new_err(format!("{:?} is not lox value", self), self.pos(), LogicError)
        }
    }

    pub fn as_lox_value(&self) -> LoxResult<LoxValue> {
        match self {
            Self::ValueToken(lox_val, _) => Ok(lox_val.clone()),
            _ => LoxError::new_err(format!("{:?} is not lox value", self), self.pos(), LogicError)
        }
    }

    pub fn eq_punct(&self, punct: Punct) -> bool {
        match self {
            Self::PunctToken(p, _) => p == &punct,
            _ => false
        }
    }

    pub fn tokenizing_err<A>(text: String, pos: usize) -> LoxResult<A> {
        LoxError::new_err(text.to_string(), pos, TokenizingErr)
    }
}

impl fmt::Display for Token {   
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[Token: {:?}]", self)
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
