use crate::interpreter::LoxError;
use crate::interpreter::parser::FinalOrRecursive::Final;
use super::super::interpreter::text_reader::TextReader;
use super::super::interpreter::scanner::ScannerOutput;
use super::errors::{LoxError::*, LoxResult};
use std::cell::Cell;
use super::tokens::{Token, Token::*, token_types::{Punct, Punct::*, LoxValue}};

pub mod expression_structure;
use expression_structure::*;

struct TokenReader {
    tokens: Box<Vec<Token>>,
    pos: Cell<usize>
}

impl TokenReader {
    fn new(tokens: Box<Vec<Token>>) -> Self {
        TokenReader {
            tokens: tokens,
            pos: Cell::new(0)
        }
    }

    pub fn advance(&self) -> Option<&Token> {
        self.next();
        self.curr_token()
    }

    fn curr_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos.get())
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    fn next(&self) {
        self.pos.set(self.pos.get() + 1);
    }

    fn back(&self) {
        self.pos.set(self.pos.get() - 1);
    }

    fn previous(&self) -> LoxResult<&Token> {
        self.tokens.get(self.pos.get() - 1)
            .ok_or(ParsingError("Failed to go back".to_string()))
    }
}

pub struct Parser {
    text_reader: TextReader,
    token_reader: TokenReader,
    pos: Cell<usize>
}

impl Parser {
    pub fn new(scanner_output: ScannerOutput) -> Self{
        Parser {
            text_reader: scanner_output.reader,
            token_reader: TokenReader::new(scanner_output.tokens),
            pos: Cell::new(0)
        }
    }

    fn err(&self, text: &str) -> LoxError {
        ParsingError(
            text.to_string() +
            &format!("\n\t At position {:?}", self.token_reader.curr_token()).to_string()
        )
    }
}
