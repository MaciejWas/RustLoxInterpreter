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
        self.next();
        if let Some(curr) = self.curr_token() {
            if !curr.is_eof() {
                return Some(curr);
            }
        }

        None
    }

    pub fn advance_if(&self, predicate: fn(&Token) -> bool) -> Option<&Token> {
        let next = self.advance()?;
        println!("Trying to advance with {:?}", next);
        if predicate(next) {
            return Some(next);
        }
        println!("doesnt match predicate");
        None
    }

    pub fn curr_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos.get())
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    pub fn next(&self) {
        if self.has_started.get() {
            self.pos.set(self.pos.get() + 1);
        } else {
            self.has_started.set(true);
        }
    }

    pub fn back(&self) {
        self.pos.set(self.pos.get() - 1);
    }

    pub fn previous(&self) -> LoxResult<&Token> {
        self.tokens.get(self.pos.get() - 1)
            .ok_or(ParsingError("Failed to go back".to_string()))
    }
}
