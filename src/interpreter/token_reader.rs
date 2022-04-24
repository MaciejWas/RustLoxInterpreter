use crate::interpreter::errors::{
    LoxResult, 
    LoxError::ParsingError
};
use crate::interpreter::tokens::Token;

use std::cell::Cell;

pub struct TokenReader {
    tokens: Vec<Token>,
    pos: Cell<usize>
}

impl TokenReader {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenReader {
            tokens: tokens,
            pos: Cell::new(0)
        }
    }

    pub fn advance(&self) -> Option<&Token> {
        self.next();
        self.curr_token()
    }

    pub fn curr_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos.get())
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    pub fn next(&self) {
        self.pos.set(self.pos.get() + 1);
    }

    pub fn back(&self) {
        self.pos.set(self.pos.get() - 1);
    }

    pub fn previous(&self) -> LoxResult<&Token> {
        self.tokens.get(self.pos.get() - 1)
            .ok_or(ParsingError("Failed to go back".to_string()))
    }
}
