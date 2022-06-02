use crate::interpreter::errors::{ErrBuilder, ErrType::ScanningErr, LoxResult};

use crate::interpreter::readers::{Reader, TextReader};
use crate::interpreter::tokens::{Equals, Punct::*, Token, Tokenizable};

pub struct ScannerOutput {
    pub tokens: Vec<Token>,
}

pub struct Scanner {
    reader: TextReader,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            reader: TextReader::from_vec(source.chars().collect()),
        }
    }

    pub fn scan(self) -> LoxResult<ScannerOutput> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            tokens.push(token.clone());
            if token.equals(&Eof) {
                break;
            }
        }

        Ok(ScannerOutput { tokens: tokens })
    }

    fn next_token(&self) -> LoxResult<Token> {
        let curr_pos = self.reader.pos();
        let err_builder = ErrBuilder::at(curr_pos).with_type(ScanningErr);

        match self.reader.advance() {
            Some(c) => match c {
                '(' => Ok(LeftParen.at(curr_pos)),
                ')' => Ok(RightParen.at(curr_pos)),
                '{' => Ok(LeftBrace.at(curr_pos)),
                '}' => Ok(RightBrace.at(curr_pos)),
                ',' => Ok(Comme.at(curr_pos)),
                '.' => Ok(Dot.at(curr_pos)),
                '-' => Ok(Minus.at(curr_pos)),
                '+' => Ok(Plus.at(curr_pos)),
                ';' => Ok(Semicolon.at(curr_pos)),
                '*' => Ok(Star.at(curr_pos)),
                '!' => self.handle_bang(),
                '=' => self.handle_eq(),
                '>' => self.handle_gr(),
                '<' => self.handle_le(),
                '/' => self.handle_slash(),
                ' ' => self.next_token(),
                '\t' => self.next_token(),
                '\n' => self.next_token(),
                _ => {
                    if is_valid_variable_char(c) {
                        self.handle_literal(c)
                    } else {
                        Err(err_builder
                            .with_message(format!("Unrecognized character `{c}`"))
                            .build())
                    }
                }
            },
            None => Ok(Eof.at(self.reader.pos())),
        }
    }

    fn handle_literal(&self, first_char: &char) -> LoxResult<Token> {
        let mut buffer = String::new();
        buffer.push(*first_char);
        let start = self.reader.pos();

        let is_string_literal = *first_char == '*';

        while let Some(c) = self.reader.peek() {
            if !is_valid_variable_char(c) && !is_string_literal {
                break;
            }
            self.reader.advance();
            buffer.push(*c);

            if *c == '"' {
                break;
            }
        }
        Token::from_string(buffer, start)
    }

    fn handle_bang(&self) -> LoxResult<Token> {
        match self.reader.peek() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(BangEqual.at(self.reader.pos()))),
                _ => Ok(Bang.at(self.reader.pos())),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_eq(&self) -> LoxResult<Token> {
        match self.reader.peek() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(EqualEqual.at(self.reader.pos()))),
                _ => Ok(Equal.at(self.reader.pos())),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_le(&self) -> LoxResult<Token> {
        match self.reader.peek() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(LessEqual.at(self.reader.pos()))),
                _ => Ok(Less.at(self.reader.pos())),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_gr(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(GreaterEqual.at(self.reader.pos()))),
                _ => Ok(Greater.at(self.reader.pos())),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_slash(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '/' => self.handle_comment(),
                _ => Ok(Slash.at(self.reader.pos())),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_comment(&self) -> LoxResult<Token> {
        self.reader.advance_until(|c: &char| *c == '\n');
        self.next_token()
    }

    fn advance_and<A, F>(&self, func: F) -> LoxResult<A>
    where
        F: Fn() -> LoxResult<A>,
    {
        self.reader.advance();
        func()
    }
}

fn is_valid_variable_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '\'' || *c == '_' || *c == '"'
}

fn unexpected_eof_err<A>(pos: usize) -> LoxResult<A> {
    Err(ErrBuilder::at(pos)
        .with_type(ScanningErr)
        .expected_but_found("next character", "end of file")
        .build())
}
