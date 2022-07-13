use crate::interpreter::errors::position::Position;
use crate::interpreter::errors::ErrBuilder;
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
    PunctToken(Punct, Position),
    KwdToken(Kwd, Position),
    ValueToken(LoxValue, Position),
    IdentifierToken(String, Position),
}

pub fn position_of(token: &Token) -> Position {
    match token {
        Token::PunctToken(_, pos) => pos,
        Token::KwdToken(_, pos) => pos,
        Token::ValueToken(_, pos) => pos,
        Token::IdentifierToken(_, pos) => pos,
    }
    .clone()
}

impl Token {
    pub fn from_string(string: String, position: Position) -> LoxResult<Self> {
        if string.eq("true") {
            Ok(Self::ValueToken(LoxValue::from(true), position))
        } else if string.eq("false") {
            Ok(Self::ValueToken(LoxValue::from(false), position))
        } else if Kwd::is_valid(&string) {
            let kwd = Kwd::from(&string, position.clone())?;
            Ok(Self::KwdToken(kwd, position))
        } else if Regex::new(VARIABLE_RE).unwrap().is_match(&string) {
            Ok(Self::IdentifierToken(string, position))
        } else if Regex::new(NUMBER_RE).unwrap().is_match(&string) {
            let number: i16 = string.parse().expect("Failed to parse string as number");
            Ok(Self::ValueToken(LoxValue::from(number), position))
        } else if string.starts_with('\"') && string.ends_with('\"') {
            Ok(Self::ValueToken(LoxValue::from(string), position))
        } else {
            Err(Self::tokenizing_err()
                .with_message(format!("Did not understand {}", string))
                .with_pos(position)
                .build())
        }
    }

    pub fn as_punct(&self) -> LoxResult<Punct> {
        match self {
            Self::PunctToken(punct, _) => Ok(punct.clone()),
            _ => ErrBuilder::new()
                .at(position_of(self))
                .is_not(self, "a lox value")
                .to_result(),
        }
    }
    
    pub fn is_identifier(&self) -> bool {
        match self {
            Self::IdentifierToken(_, _) => true,
            _ => false
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Self::ValueToken(_, _) => true,
            _ => false
        }
    }

    pub fn as_lox_value(&self) -> LoxResult<LoxValue> {
        match self {
            Self::ValueToken(lox_val, _) => Ok(lox_val.clone()),
            _ => ErrBuilder::new()
                .at(position_of(self))
                .is_not(self, "a lox value")
                .to_result(),
        }
    }

    pub fn tokenizing_err() -> ErrBuilder {
        ErrBuilder::new().of_type(TokenizingErr)
    }

    pub fn can_be_unary_op(&self) -> bool {
        match self {
            Self::PunctToken(p, _) => [Punct::Minus].contains(p),
            _ => false,
        }
    }

    pub fn satisfies_or<Pred, ErrMaker>(&self, pred: Pred, err: ErrMaker) -> LoxResult<&Self>
    where
        Pred: Fn(&Self) -> bool,
        ErrMaker: Fn(&Token) -> LoxError,
    {
        if pred(self) {
            return Ok(self);
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
    fn at(self, pos: Position) -> Token;
}

impl Tokenizable for Punct {
    fn at(self, pos: Position) -> Token {
        Token::PunctToken(self, pos)
    }
}

impl Tokenizable for String {
    fn at(self, pos: Position) -> Token {
        Token::from_string(self, pos).unwrap() // unsafe lol
    }
}

impl Tokenizable for LoxValue {
    fn at(self, pos: Position) -> Token {
        Token::ValueToken(self, pos)
    }
}
