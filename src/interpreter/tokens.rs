use std::fmt;
use regex::Regex;
use super::errors::{LoxError::*, LoxError, LoxResult, logic_err};
use token_types::*;

pub mod token_types;

const VARIABLE_RE: &str = r"^[a-zA-Z_'][a-zA-Z0-9_']*$"; 
const NUMBER_RE: &str = r"[0-9]+";

fn is_valid_kwd(string: &String) -> bool {
    match Kwd::from(string) {
        Ok(kwd_token) => true,
        _             => false
    }
}

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
            Ok(Self::ValueToken(LoxValue::from_bool(true), pos))
        } else if string.eq("False") {
            Ok(Self::ValueToken(LoxValue::from_bool(false), pos))
        } else if is_valid_kwd(&string) {
            let kwd = Kwd::from(&string)?;
            Ok(Self::KwdToken(kwd, pos))
        } else if Regex::new(VARIABLE_RE).unwrap().is_match(&string) {
            Ok(Self::IdentifierToken(string, pos))
        } else if Regex::new(NUMBER_RE).unwrap().is_match(&string) {
            Ok(Self::ValueToken(LoxValue::from_int(string.parse().expect("Failed to parse string")), pos))
        } else if string.starts_with("\"") && string.ends_with("\"") {
            Ok(Self::ValueToken(LoxValue::from_string(string), pos))
        } else {
            Err(ParsingError(format!("Did not understand {}", string)))
        }
    }

    pub fn is_eq_or_neq(&self) -> bool {
        match self {
            Self::PunctToken(punct, _) => punct.is_eq_or_neq(),
            _ => false
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
            Self::PunctToken(punct, _) => punct.is_comparison(),
            _ => false
        }
    }

    pub fn is_plus_minus(&self) -> bool {
        match self {
            Self::PunctToken(punct, _) => punct.is_plus_minus(),
            _ => false
        }
    }

    pub fn is_mul_div(&self) -> bool {
        match self {
            Self::PunctToken(punct, _) => punct.is_mul_div(),
            _ => false
        }
    }

    pub fn is_neg(&self) -> bool {
        match self {
            Self::PunctToken(punct, _) => punct.is_neg(),
            _ => false
        }
    }

    pub fn is_eof(&self) -> bool {
        match self {
            Self::PunctToken(punct, _) => Punct::Eof == *punct,
            _ => false
        }
    }

    pub fn pos(&self) -> usize {
        match self {
            Self::PunctToken(_, pos) => pos,
            Self::KwdToken(_, pos)   => pos,
            Self::ValueToken(_, pos) => pos,
            Self::Identifier(_, pos) => pos,
        }
    }

    pub fn as_punct(&self) -> LoxResult<Punct> {
        match self {
            Self::PunctToken(punct, _) => Ok(punct.clone()),
            _ => logic_err(format!("{:?} is not punct", self))
        }
    }

    pub fn as_lox_value(&self) -> LoxResult<LoxValue> {
        match self {
            Self::ValueToken(lox_val, _) => Ok(lox_val.clone()),
            _ => logic_err(format!("{:?} is not lox value", self))
        }
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
