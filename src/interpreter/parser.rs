use crate::interpreter::LoxError;
use crate::interpreter::text_reader::TextReader;
use crate::interpreter::token_reader::TokenReader;
use crate::interpreter::scanner::ScannerOutput;
use crate::interpreter::errors::{LoxError::*, LoxResult};
use crate::interpreter::tokens::Token;

use std::cell::Cell;

pub mod expression_structure;
pub mod pretty_printing;
pub mod evaluating;

use expression_structure::*;
use pretty_printing::{PrettyPrint};


pub struct Parser {
    text_reader: TextReader,
    token_reader: TokenReader,
}

impl Parser {
    pub fn new(scanner_output: ScannerOutput, text: String) -> Self {
        Parser {
            text_reader: TextReader::new(text),
            token_reader: TokenReader::new(scanner_output.tokens),
        }
    }

    pub fn parse(&self) -> LoxResult<ExprRule> {
        self.expression()
    }

    fn expression(&self) -> LoxResult<ExprRule> {
       let eq: EqltyRule = self.equality()?;
       Ok( Single {value: eq} )
    }

    fn equality(&self) -> LoxResult<EqltyRule> {
        self.abstract_rec_descent(Self::comparison, Token::is_eq_or_neq)
    }

    fn comparison(&self) -> LoxResult<CompRule> {
        self.abstract_rec_descent(Self::term, Token::is_comparison)
    }

    fn term(&self) -> LoxResult<TermRule> {
        self.abstract_rec_descent(Self::factor, Token::is_plus_minus)
    }

    fn factor(&self) -> LoxResult<FactorRule> {
        self.abstract_rec_descent(Self::unary, Token::is_mul_div)
    }

    fn unary(&self) -> LoxResult<UnaryRule> {
        let first_token = self.token_reader.advance()
            .ok_or(self.err("Expected next token."))?;

        if first_token.is_neg() {
            let second_token = self.token_reader.advance()
                .ok_or(self.err("Expected next token."))?;
            return Ok( Unary { op: Some(first_token.clone()), right: second_token.clone()  } );
        }

        Ok( Unary { op: None, right: first_token.clone() } )
    }


    fn abstract_rec_descent<A>(&self, next_rule: fn(&Self) -> LoxResult<A>, token_predicate: fn(&Token) -> bool ) -> LoxResult<Many<A>> where A: std::fmt::Debug{
        let mut xs = Vec::new();
        let x = next_rule(&self)?;
 //       println!("Found match for rec descent: {:?}", x);

        while let Some(token) = self.token_reader.advance_if(token_predicate) {
            let x2 = next_rule(&self)?;
            xs.push((token.clone(), x2));
        }

        Ok ( Many { first: x, rest: xs } )
    
    }

    fn err(&self, text: &str) -> LoxError {
        ParsingError(
            text.to_string() +
            &format!("\n\t At position {:?}", self.token_reader.curr_token()).to_string()
        )
    }
}
