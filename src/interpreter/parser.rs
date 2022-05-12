use crate::interpreter::{
    errors::{ErrType::ParsingErr, LoxError, LoxResult},
    readers::TokenReader,
    scanner::ScannerOutput,
    tokens::{Equals, Kwd, Punct::*, Token},
};

pub mod pretty_printing;
pub mod structure;

use structure::*;

pub struct Parser {
    token_reader: TokenReader,
}

impl Parser {
    pub fn new(scanner_output: ScannerOutput) -> Self {
        Parser {
            token_reader: TokenReader::new(scanner_output.tokens),
        }
    }

    pub fn parse(&self) -> LoxResult<Program> {
        self.program()
    }

    fn program(&self) -> LoxResult<Program> {
        let mut stmts = Vec::new();

        while let Some(next_token) = self.token_reader.peek() {
            stmts.push(self.statement()?);
            
            if next_token.equals(&Semicolon) {
                self.token_reader.advance();
            } else {
                return self.err_result_at(
                    format!("Expected ; token but found {:?}.", next_token),
                    next_token.pos(),
                );
            }
        }

        let last_token = self.token_reader.curr_token().unwrap();
        if !last_token.equals(&Eof) {
            return self.err_result("Expected Eof token but found nothing.".to_string());
        }

        Ok(stmts)
    }

    fn statement(&self) -> LoxResult<Statement> {
        let first_token = self
            .token_reader
            .peek()
            .ok_or(self.err("Expected next token.".to_string()))?;

        if first_token.equals(&Kwd::Print) {
            return Ok(Or2::Opt2(PrintStmt {
                value: self.expression()?,
            }));
        }

        Ok(Or2::Opt1(self.expression()?))
    }

    fn expression(&self) -> LoxResult<ExprRule> {
        let eq: EqltyRule = self.equality()?;
        Ok(Single { value: eq })
    }

    fn equality(&self) -> LoxResult<EqltyRule> {
        self.abstract_rec_descent(Self::comparison, |t: &Token| {
            t.equals(&EqualEqual) || t.equals(&BangEqual)
        })
    }

    fn comparison(&self) -> LoxResult<CompRule> {
        self.abstract_rec_descent(Self::term, |t: &Token| {
            t.equals(&LessEqual) || t.equals(&GreaterEqual) || t.equals(&Less) || t.equals(&Greater)
        })
    }

    fn term(&self) -> LoxResult<TermRule> {
        self.abstract_rec_descent(Self::factor, |t: &Token| {
            t.equals(&Plus) || t.equals(&Minus)
        })
    }

    fn factor(&self) -> LoxResult<FactorRule> {
        self.abstract_rec_descent(Self::unary, |t: &Token| t.equals(&Star) || t.equals(&Slash))
    }

    fn unary(&self) -> LoxResult<UnaryRule> {
        let first_token = self
            .token_reader
            .advance()
            .ok_or(self.err("Expected next token.".to_string()))?;

        if let Ok(punct) = first_token.as_punct() {
            if punct.can_be_unary_op() {
                let second_token = self
                    .token_reader
                    .advance()
                    .ok_or(self.err("Expected next token".to_string()))?;
                return Ok(Unary {
                    op: None,
                    right: second_token.clone(),
                });
            }
        };

        if matches!(first_token, ValueToken) || matches!(first_token, IdentifierToken) {
            return Ok(Unary {
                op: None,
                right: first_token.clone(),
            });
        }

        self.err_result(format!(
            "Token {:?} can't be interpreted as value or identifier",
            first_token
        ))
    }

    fn abstract_rec_descent<A>(
        &self,
        next_rule: fn(&Self) -> LoxResult<A>,
        token_predicate: fn(&Token) -> bool,
    ) -> LoxResult<Many<A>>
    where
        A: std::fmt::Debug,
    {
        let mut xs = Vec::new();
        let x = next_rule(&self)?;

        while let Some(token) = self.token_reader.advance_if(token_predicate) {
            let x2 = next_rule(&self)?;
            xs.push((token.clone(), x2));
        }

        Ok(Many { first: x, rest: xs })
    }

    fn err_result<A>(&self, text: String) -> LoxResult<A> {
        Err(self.err(text))
    }

    fn err_result_at<A>(&self, text: String, pos: usize) -> LoxResult<A> {
        Err(LoxError {
            msg: text.to_string(),
            pos: pos,
            err_type: ParsingErr,
        })
    }

    fn err(&self, text: String) -> LoxError {
        LoxError {
            msg: text.to_string(),
            pos: match self.token_reader.curr_token() {
                Some(t) => t.pos(),
                None => 0,
            },
            err_type: ParsingErr,
        }
    }
}
