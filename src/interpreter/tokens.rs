use std::fmt;
use regex::Regex;
use super::errors::{LoxError::*, LoxError};

pub mod token_types;
use token_types::*;

const VARIABLE_RE: &str = r"[a-zA-Z_'][a-zA-Z0-9_']*"; 
const NUMBER_RE: &str = r"[0-9]+";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    PunctToken(Punct, usize),
    KwdToken(Kwd, usize),
    ValueToken(LoxValue, usize),
    IdentifierToken(Identifier, usize)
}

impl Token {
    pub fn from_string(string: String, pos: usize) -> Result<Self, LoxError> {
        if Regex::new(VARIABLE_RE).unwrap().is_match(&string) {
            Ok(Self::IdentifierToken(Identifier::from(string), pos))
        } else if Regex::new(NUMBER_RE).unwrap().is_match(&string) {
            Ok(Self::ValueToken(LoxValue::from_int(string.parse().expect("Failed to parse string")), pos))
        } else if string.eq("True") {
            Ok(Self::ValueToken(LoxValue::from_bool(true), pos))
        } else if string.eq("False") {
            Ok(Self::ValueToken(LoxValue::from_bool(false), pos))
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

impl Tokenizable for Kwd {
    fn at(self, pos: usize) -> Token {
        Token::KwdToken(self, pos)
    }
}

impl Tokenizable for Identifier {
    fn at(self, pos: usize) -> Token {
        Token::IdentifierToken(self, pos)
    }
}

impl Tokenizable for LoxValue {
    fn at(self, pos: usize) -> Token {
        Token::ValueToken(self, pos)
    }
}