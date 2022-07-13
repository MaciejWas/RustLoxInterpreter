use crate::interpreter::tokens::Token;

use super::ReaderBase;

use std::cell::Cell;

pub struct TokenReader {
    pub tokens: Vec<Token>,
    pos: Cell<usize>,
}

impl ReaderBase<Token> for TokenReader {
    fn from_vec(tokens: Vec<Token>) -> Self {
        TokenReader {
            tokens: tokens,
            pos: Cell::new(0),
        }
    }

    fn advance(&self) -> Option<&Token> {
        let pos = self.pos.get();
        let output = self.tokens.get(pos);
        if output.is_some() {
            self.pos.set(pos + 1);
        }
        output
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    fn previous(&self) -> Option<&Token> {
        if self.pos.get() >= 1 {
            return self.tokens.get(self.pos.get() - 1);
        }
        None
    }

    fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.pos() + n)
    }
}
