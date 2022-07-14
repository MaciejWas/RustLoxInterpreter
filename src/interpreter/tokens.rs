use crate::interpreter::errors::{
    position::Position, ErrBuilder, ErrType::TokenizingErr, LoxError, LoxResult,
};
use regex::Regex;

pub use kwds::Kwd;
pub use lox_values::LoxValue;
pub use puncts::Punct;

pub mod kwds;
pub mod lox_values;
pub mod puncts;

const VARIABLE_RE: &str = r"^[a-zA-Z_'][a-zA-Z0-9_']*$";
const NUMBER_RE: &str = r"^[0-9]+$";

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenValue {
    Punct(Punct),
    Kwd(Kwd),
    Val(LoxValue),
    Id(String),
}

impl From<Punct> for TokenValue {
    fn from(p: Punct) -> Self {
        Self::Punct(p)
    }
}

impl From<LoxValue> for TokenValue {
    fn from(p: LoxValue) -> Self {
        Self::Val(p)
    }
}

impl From<Kwd> for TokenValue {
    fn from(p: Kwd) -> Self {
        Self::Kwd(p)
    }
}

/// Token value enhanced with a position
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub val: TokenValue,
    pub pos: Position,
}

impl Token {
    pub fn new(tok_val: TokenValue, pos: Position) -> Self {
        Token {
            val: tok_val,
            pos: pos,
        }
    }

    pub fn from_string(string: String, position: Position) -> LoxResult<Self> {
        let create_from = |tv: TokenValue| {
            Ok(Token {
                val: tv,
                pos: position,
            })
        };
        if string.eq("true") {
            create_from(TokenValue::Val(LoxValue::from(true)))
        } else if string.eq("false") {
            create_from(TokenValue::Val(LoxValue::from(false)))
        } else if Kwd::is_valid(&string) {
            let kwd = Kwd::from(&string, position.clone())?;
            create_from(TokenValue::from(kwd))
        } else if Regex::new(NUMBER_RE).unwrap().is_match(&string) {
            let number: i16 = string.parse().expect("Failed to parse string as number");
            create_from(TokenValue::from(LoxValue::from(number)))
        } else if string.starts_with('\"') && string.ends_with('\"') {
            create_from(TokenValue::from(LoxValue::from(string)))
        } else if Regex::new(VARIABLE_RE).unwrap().is_match(&string) {
            create_from(TokenValue::Id(string))
        } else {
            Err(Self::tokenizing_err()
                .with_message(format!("Did not understand {}", string))
                .with_pos(position)
                .build())
        }
    }

    pub fn as_punct(&self) -> LoxResult<Punct> {
        match &self.val {
            TokenValue::Punct(p) => Ok(p.clone()),
            _ => ErrBuilder::new()
                .at(self.pos)
                .is_not(self, "a lox value")
                .to_result(),
        }
    }

    pub fn is_identifier(&self) -> bool {
        match &self.val {
            TokenValue::Id(_) => true,
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        match self.val {
            TokenValue::Val(_) => true,
            _ => false,
        }
    }

    pub fn as_lox_value(&self) -> LoxResult<LoxValue> {
        match &self.val {
            TokenValue::Val(lox_val) => Ok(lox_val.clone()),
            _ => ErrBuilder::new()
                .at(self.pos)
                .is_not(self, "a lox value")
                .to_result(),
        }
    }

    pub fn tokenizing_err() -> ErrBuilder {
        ErrBuilder::new().of_type(TokenizingErr)
    }

    pub fn can_be_unary_op(&self) -> bool {
        match &self.val {
            TokenValue::Punct(p) => [Punct::Minus].contains(&p),
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

pub trait Equals<A> {
    fn equals(&self, a: A) -> bool;
}

impl Equals<Kwd> for Token {
    fn equals(&self, kwd: Kwd) -> bool {
        match &self.val {
            TokenValue::Kwd(k) => *k == kwd,
            _ => false,
        }
    }
}

impl Equals<Punct> for Token {
    fn equals(&self, punct: Punct) -> bool {
        match &self.val {
            TokenValue::Punct(p) => *p == punct,
            _ => false,
        }
    }
}

pub trait Tokenizable {
    fn at(self, pos: Position) -> Token;
}

impl Tokenizable for Punct {
    fn at(self, pos: Position) -> Token {
        Token {
            val: TokenValue::from(self),
            pos: pos,
        }
    }
}

impl Tokenizable for LoxValue {
    fn at(self, pos: Position) -> Token {
        Token {
            val: TokenValue::from(self),
            pos: pos,
        }
    }
}
