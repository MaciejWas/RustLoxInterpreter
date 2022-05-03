use crate::interpreter::tokens::Punct::*;
use crate::interpreter::tokens::Token;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::errors::ErrType::ParsingErr;
use crate::interpreter::LoxError;
use crate::interpreter::readers::TokenReader;
use crate::interpreter::scanner::ScannerOutput;

pub mod expression_structure;
pub mod pretty_printing;
pub mod evaluating;

use expression_structure::*;

pub struct Parser {
    token_reader: TokenReader,
}

impl Parser {
    pub fn new(scanner_output: ScannerOutput) -> Self {
        Parser {
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
        self.abstract_rec_descent(Self::comparison, |t: &Token| t.eq_punct(EqualEqual)
                                                             || t.eq_punct(BangEqual))
    }

    fn comparison(&self) -> LoxResult<CompRule> {
        self.abstract_rec_descent(Self::term, |t: &Token| t.eq_punct(LessEqual) 
                                                       || t.eq_punct(GreaterEqual)
                                                       || t.eq_punct(Less) 
                                                       || t.eq_punct(Greater))
    }

    fn term(&self) -> LoxResult<TermRule> {
        self.abstract_rec_descent(Self::factor, |t: &Token| t.eq_punct(Plus)
                                                         || t.eq_punct(Minus))
    }

    fn factor(&self) -> LoxResult<FactorRule> {
        self.abstract_rec_descent(Self::unary, |t: &Token| t.eq_punct(Star) 
                                                        || t.eq_punct(Slash))
    }

    fn unary(&self) -> LoxResult<UnaryRule> {
        let first_token = self.token_reader.advance()
            .ok_or(self.err("Expected next token.".to_string()))?;

        if first_token.eq_punct(Minus) {
            let second_token = self.token_reader.advance()
                                                .ok_or(self.err("Expected next token.".to_string()))?;
            if let Token::ValueToken(_, _) = second_token {
                return Ok( Unary { op: Some(first_token.clone()), right: second_token.clone()  } );
            }

            self.err(format!("{:?} is not a valid Lox value.", second_token));
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

    fn err(&self, text: String) -> LoxError {
        LoxError {
            msg: text.to_string(),
            pos: self.token_reader.curr_token()
                                  .unwrap()
                                  .pos(),
            err_type: ParsingErr
        }
    }
}
