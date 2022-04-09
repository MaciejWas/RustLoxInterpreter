use super::super::interpreter::text_reader::TextReader;
use super::super::interpreter::scanner::ScannerOutput;
use super::errors::{LoxError::*, LoxError, LoxResult};
use std::cell::Cell;
use super::tokens::{Token, Token::*, token_types::{Punct, Punct::*, LoxValue}};

pub struct Parser {
    reader: TextReader,
    tokens: Box<Vec<Token>>,
    pos: Cell<usize>
}

impl Parser {
    pub fn new(scanner_output: ScannerOutput) -> Self{
        Parser {
            reader: scanner_output.reader,
            tokens: scanner_output.tokens,
            pos: Cell::new(0)
        }
    }

    fn curr_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos.get())
    }

    fn parsing_error<A>(&self, text: &str) -> LoxResult<A> {
        let curr_token: Option<&Token> = self.curr_token();

        Err(ParsingError(
            text.to_string() + 
            &format!("\n\t At position {}", self.pos.get()).to_string()
        ))
    }

    fn incr_pos(&self) {
        self.pos.set(self.pos.get() + 1);
    }

    fn decr_pos(&self) {
        self.pos.set(self.pos.get() - 1);
    }

    fn previous(&self) -> LoxResult<&Token> {
        self.tokens.get(self.pos.get() - 1)
            .ok_or(ParsingError("Failed to go back".to_string()))
    }

    fn expression(&self) -> LoxResult<Expr> {
        self.equality()
    }

    fn equality(&self) -> LoxResult<Expr> {
        let left: LoxResult<Expr> = self.comparison();
        let mut op: LoxResult<Punct> = self.parsing_error("Operand not found!");
        let mut right: LoxResult<Expr> = self.parsing_error("Right side of expr not found!");

        while let Some(PunctToken(p, _)) = self.tokens.get(self.pos.get()) {
            if *p == EqualEqual || *p == BangEqual {
                op = Ok(*p);
                right = self.comparison();
            }
        }

        Ok(Expr::create_binary(left?, op?, right?))
    }

    fn comparison(&self) -> LoxResult<Expr> {Ok(Expr::Atomic(LoxValue::from_bool(true)))}
}

pub enum Expr {
    Atomic(LoxValue),
    Unary(Punct, Box<Expr>),
    Binary(Box<Expr>, Punct, Box<Expr>)
}

impl Expr {
    pub fn create_unary(op: Punct, right: Expr) -> Expr {
        Expr::Unary(op, Box::new(right))
    }

    pub fn create_binary(left: Expr, op: Punct, right: Expr) -> Self {
        Expr::Binary(Box::new(left), op, Box::new(right))
    }
}

// impl Expr {
//     pub fn accept(&self, visitor: Visitor) {
//         match self {
//             Self::Atomic(_) => visitor.visit_val(self),
//             Self::Unary(_, _) => visitor.visit_un(self),
//             Self::Binary(_, _, _) => visitor.visit_bin(self),
//         }
//     }
// }


// pub fn parse(tokens: Vec<Token>) -> Expr {   
//     return Expr::Atomic(LoxValue::Boolean(false));
// }
