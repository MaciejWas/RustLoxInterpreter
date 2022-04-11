// use super::super::interpreter::parser::expression_structure::*;
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

    fn expression(&self) -> LoxResult<ExprNode> {
        let eq = self.equality()?;
        Ok(ExprNode { eq: eq })
    }

    fn equality(&self) -> LoxResult<EqNode> {
        let left: CompNode = self.comparison()?;
        let mut node: Option<EqNode> = None;
        
        while let Some(token) = self.token_reader.advance() {
            if token.is_eq_or_neq() {
                let right = self.comparison()?;
                node = Parser::recursive_node(node, left, *token, right)
            } else {
                return Err(self.err("Expected new comparison but no :("));
            }
        }

        node.ok_or(self.err("Equality node not found!"))
    }

    fn comparison(&self) -> LoxResult<CompNode> {
        let left: TermNode = self.term()?;
        let mut node: Option<CompNode> = None;
        
        while let Some(token) = self.token_reader.advance() {
            if token.is_comparison() {
                let right = self.term()?;
                node = Parser::recursive_node(node, left, *token, right)
            } else {
                return Err(self.err("Expected new comparison but no :("));
            }
        }

        node.ok_or(self.err("Comparison node not found!"))
    }

    fn term(&self) -> LoxResult<TermNode> {
        let left: FactorNode = self.factor()?;
        let mut node: Option<TermNode> = None;
        
        while let Some(token) = self.token_reader.advance() {
            if token.is_plus_minus() {
                let right = self.factor()?;
                node = Parser::recursive_node(node, left, *token, right)
            } else {
                return Err(self.err("Expected new comparison but no :("));
            }
        }

        node.ok_or(self.err("Term not found!"))
    }

    fn factor(&self) -> LoxResult<FactorNode> {
        let left: UnaryNode = self.unary()?;
        let node: Option<FactorNode> = None;
        
        while let Some(token) = self.token_reader.advance() {
            if token.is_mul_div() {
                let right = self.unary()?;
                node = Parser::recursive_node(node, left, *token, right)
            } else {
                return Err(self.err("Expected new comparison but no :("));
            }
        }

        node.ok_or(self.err("Factor node not found!"))
    }

    fn unary(&self) -> LoxResult<UnaryNode> {
        let mut op: Option<Token> = None;
        let primary: Primary;

        let token: &Token = self.token_reader.advance()
            .ok_or(self.err("Token not found!"))?;
        
        if token.is_neg() {
            op = Some(token.clone());
            primary = self.primary()?;
        } else {
            self.token_reader.back();
            primary = self.primary()?;
        }

        Ok( UnaryNode::of(op, primary) )
    }

    fn primary(&self) -> LoxResult<Primary> {
        let err_msg = "Primary expressio not found!";
        match self.token_reader.advance() {
            Some(token) => match token {
                ValueToken(_, _)      => Ok(Primary::of(token.clone())),
                IdentifierToken(_, _) => Ok(Primary::of(token.clone())),
                _                     => Err(self.err(err_msg))
            },
            None        => Err(self.err(err_msg))
        }
    }

    fn recursive_node<SubNode>(
        node: Option<Node<SubNode>>,
        left: SubNode, op: Token,
        right: SubNode) -> Option<Node<SubNode>>
    {
        match node {
            None       => Some(Node::new(left, op, Final(right))),
            Some(node) => {
                node.replace_right(op, right);
                Some(node)
            }
        }
    }

    fn err(&self, text: &str) -> LoxError {
        ParsingError(
            text.to_string() +
            &format!("\n\t At position {:?}", self.token_reader.curr_token()).to_string()
        )
    }
}
