use std::fmt;
use regex::Regex;
use super::errors::{LoxError::*, LoxError};

const VARIABLE_PATTERN: &str = r"[a-zA-Z_'][a-zA-Z0-9_']*"; 
const VARIABLE_RE: Regex = Regex::new(VARIABLE_PATTERN).unwrap();
const STRING_RE: Regex = Regex::new("\".*\"").unwrap();
const NUMBER_RE: Regex = Regex::new(r"-?[0-9][0-9,\.]+").unwrap();

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comme, Dot, Minus, Plus, Semicolon, Slash, Star,
  
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
  
    // Literals.
    Identifier, LoxString, LoxNumber,
  
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
  
    Eof, Comment
}

impl TokenType {
    pub fn at(self, pos: usize) -> Token {
        Token { ttype: self, lexeme: "".to_string(), pos: pos}
    }
}

impl fmt::Display for TokenType {   
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    pos: usize
}

impl Token {
    pub fn new(tt: TokenType, lexeme: String, pos: usize) -> Self {
        Token {ttype: tt, lexeme: lexeme, pos: pos}
    }

    pub fn is_of_type(&self, t: TokenType) -> bool {
        self.ttype == t
    }

    pub fn from_string(string: String, pos: usize) -> Result<Self, LoxError> {
        if VARIABLE_RE.is_match(&string) {
            Ok(Self::new(TokenType::Identifier, string, pos))
        } else if string.starts_with('"') && string.ends_with('"') {
            Ok(Self::new(TokenType::LoxString, string, pos))
        } else if NUMBER_RE.is_match(&string) {
            Ok(Self::new(TokenType::LoxNumber, string, pos))
        } else {
            Err(ParsingError(format!("Did not understand {}", string)))
        }
    }
}

impl fmt::Display for Token {   
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[Token: {}, {} | at {}]", self.ttype, self.lexeme, self.pos)
    }
}