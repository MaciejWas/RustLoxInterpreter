use crate::interpreter::LoxError;
use crate::interpreter::readers::{TextReader, TokenReader};
use crate::interpreter::scanner::ScannerOutput;
use crate::interpreter::errors::{LoxError::*, LoxResult};
use crate::interpreter::tokens::{
    Token
    Punct::*
};

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
        self.abstract_rec_descent(Self::comparison, |t: Token| t.eq_punct(EqualEqual)
                                                            || t.eq_punct(BandEqual))
    }

    fn comparison(&self) -> LoxResult<CompRule> {
        self.abstract_rec_descent(Self::term, |t: Token| t.eq_punct(LessEqual) 
                                                      || t.eq_punct(GreaterEqual)
                                                      || t.eq_punct(Less) 
                                                      || t.eq_punct(Greater))
    }

    fn term(&self) -> LoxResult<TermRule> {
        self.abstract_rec_descent(Self::factor, |t: Token| t.eq_punct(Plus)
                                                        || t.eq_punct(Minus))
    }

    fn factor(&self) -> LoxResult<FactorRule> {
        self.abstract_rec_descent(Self::unary, |t: Token| t.eq_punct(Star) 
                                                       || t.eq_punct(Slash))
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

        while let Some(token) = self.token_reader.advance_if(token_predicate) {
            let x2 = next_rule(&self)?;
            xs.push((token.clone(), x2));
        }

        Ok ( Many { first: x, rest: xs } )
    
    }

    fn err(&self, text: &str) -> LoxError {
        LoxError {
            msg: text.to_string(),
            pos: self.token_reader.curr_token()
                                  .pos(),
            err_type: ErrType::ParsingErr
        }
    }
}
