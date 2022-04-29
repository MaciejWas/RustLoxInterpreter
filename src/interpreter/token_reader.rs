use crate::interpreter::errors::{
    LoxResult, 
    LoxError::ParsingError
};
use crate::interpreter::tokens::Token;

use std::cell::Cell;

pub struct TokenReader {
    tokens: Vec<Token>,
    pos: Cell<usize>,
    has_started: Cell<bool>
}

impl TokenReader {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenReader {
            tokens: tokens,
            pos: Cell::new(0),
            has_started: Cell::new(false)
        }
    }

    
    pub fn advance(&self) -> Option<&Token> {
        if let Some(token) = self.peek() {
            self.step_forward();
            return Some(token);
        }

        None
    }

    pub fn advance_if(&self, predicate: fn(&Token) -> bool) -> Option<&Token> {
        let next_token = self.peek()?;
        if predicate(next_token) {
            return self.advance();
        }

        None
    }

    fn step_forward(&self) {
        if self.has_started.get() {
            self.pos.set(self.pos.get() + 1);
        } else {
            self.has_started.set(true);
        }
    }

    pub fn curr_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos.get())
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.has_started.get() {
            return self.tokens.get(self.pos.get() + 1);  
        } else {
            return self.tokens.get(self.pos.get())
        }
    }
}
