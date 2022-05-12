use crate::interpreter::tokens::Token;
use crate::interpreter::readers::reader::ReaderBase;

use std::cell::Cell;

pub struct TokenReader {
    tokens: Vec<Token>,
    pos: Cell<usize>,
}

impl ReaderBase<Token> for TokenReader {
    fn from_vec(tokens: Vec<Token>) -> Self {
        TokenReader {
            tokens: tokens,
            pos: Cell::new(0),
        }
    }

    fn advance(&self) -> Option<()> {
        if self.peek().is_some() {
            self.pos.set(self.pos.get() + 1);
            return Some(())
        }

        None
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos.get())
    }

    fn pos(&self) -> usize {
        self.pos.get()
    } // where self.advance(); self.pos() is the same as self.pos() + 1

    fn curr(&self) -> &Token {
        self.tokens.get(self.pos.get())
            .unwrap_or_else(|| panic!("Position {:?} is out of bounds for tokens", self.pos.get()))
    }
}