use crate::interpreter::LoxError;
use crate::interpreter::text_reader::TextReader;
use crate::interpreter::token_reader::TokenReader;
use crate::interpreter::scanner::ScannerOutput;
use crate::interpreter::errors::{LoxError::*, LoxResult};
use crate::interpreter::tokens::Token;

use std::cell::Cell;

pub mod expression_structure;
use expression_structure::*;

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
        let mut comparisons = Vec::new();
        let first_comp: CompRule = self.comparison()?;

        while let Some(token) = self.token_reader.advance_if(Token::is_eq_or_neq) {
            let next_comp = self.comparison()?;
            comparisons.push((token.clone(), next_comp));
        }

        Ok( Many { first: first_comp, rest: comparisons } )
    }

    fn comparison(&self) -> LoxResult<CompRule> {
        let mut terms = Vec::new();
        let first_term = self.term()?;

        while let Some(token) = self.token_reader.advance_if(Token::is_comparison) {
            let next_term = self.term()?;
            terms.push((token.clone(), next_term));
        }
        Ok ( Many { first: first_term, rest: terms } )
    }

    fn term(&self) -> LoxResult<TermRule> {
        let mut factors = Vec::new();
        let first_factor = self.factor()?;

        while let Some(token) = self.token_reader.advance_if(Token::is_plus_minus) {
            let next_factor = self.factor()?;
            factors.push((token.clone(), next_factor));
        }
        Ok ( Many { first: first_factor, rest: factors } )
    }

    fn factor(&self) -> LoxResult<FactorRule> {
        let mut unarys = Vec::new();
        let first_unary = self.unary()?;

        while let Some(token) = self.token_reader.advance_if(Token::is_mul_div) {
            let next_unary = self.unary()?;
            unarys.push((token.clone(), next_unary));
        }

        Ok ( Many { first: first_unary, rest: unarys } )
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


    fn abstract_rec_descent<A>(&self, next_rule: fn(&Self) -> LoxResult<A>, token_predicate: fn(&Token) -> bool ) {
        let mut xs = Vec::new();
        let x = self.next_rule()?;

        while let Some(token) = self.token_reader.advance_if(Token::is_mul_div) {
            let next_unary = self.unary()?;
            unarys.push((token.clone(), next_unary));
        }

        Ok ( Many { first: first_unary, rest: unarys } )
    
    }

    fn err(&self, text: &str) -> LoxError {
        ParsingError(
            text.to_string() +
            &format!("\n\t At position {:?}", self.token_reader.curr_token()).to_string()
        )
    }
}
