use crate::interpreter::LoxError;
use crate::interpreter::text_reader::TextReader;
use crate::interpreter::token_reader::TokenReader;
use crate::interpreter::scanner::ScannerOutput;
use crate::interpreter::errors::{LoxError::*, LoxResult};
use super::tokens::{
    Token, Token::*, token_types::{Punct, Punct::*, LoxValue}
};

use std::cell::Cell;

pub mod expression_structure;
use expression_structure::*;


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

    fn expression(&self) -> LoxResult<ExprNode> {
        self.equality()
    }

    fn err(&self, text: &str) -> LoxError {
        ParsingError(
            text.to_string() +
            &format!("\n\t At position {:?}", self.token_reader.curr_token()).to_string()
        )
    }
}
