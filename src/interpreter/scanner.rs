//! The scanner. Basically a pure function from a `String` to a `Vec<Token>`.

use crate::interpreter::errors::position::Position;
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
            let is_eof = token.equals(&Eof);
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(ScannerOutput { tokens: tokens })
    }

    fn next_token(&self) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();

        match self.reader.advance() {
            Some(c) => match c {
                '(' => Ok(LeftParen.at(pos)),
                ')' => Ok(RightParen.at(pos)),
                '{' => Ok(LeftBrace.at(pos)),
                '}' => Ok(RightBrace.at(pos)),
                ',' => Ok(Comme.at(pos)),
                '.' => Ok(Dot.at(pos)),
                '-' => Ok(Minus.at(pos)),
                '+' => Ok(Plus.at(pos)),
                ';' => Ok(Semicolon.at(pos)),
                '*' => Ok(Star.at(pos)),
                '!' => self.handle_bang(),
                '=' => self.handle_eq(),
                '>' => self.handle_gr(),
                '<' => self.handle_le(),
                '/' => self.handle_slash(),
                ' ' => self.next_token(),
                '\t' => self.next_token(),
                '\n' => self.next_token(),
                '\r' => self.next_token(),
                _ => {
                    if is_valid_variable_char(c) {
                        self.handle_literal(c)
                    } else {
                        ErrBuilder::new()
                            .at(pos)
                            .of_type(ScanningErr)
                            .with_message(format!("Unrecognized character {}", *c as u32))
                            .to_result()
                    }
                }
            },
            None => Ok(Eof.at(pos)),
        }
    }

    fn handle_literal(&self, first_char: &char) -> LoxResult<Token> {
        if *first_char == '"' {
            return self.handle_string_literal(first_char);
        }

        self.handle_var_or_val_literal(first_char)
    }

    fn handle_string_literal(&self, first_char: &char) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();

        let mut buffer = String::new();
        buffer.push(*first_char);
        while let Some(c) = self.reader.advance() {
            buffer.push(*c);
            if *c == '"' {
                break;
            }
        }
        Ok(buffer.at(pos))
    }

    fn handle_var_or_val_literal(&self, first_char: &char) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();
        let mut buffer = String::new();
        buffer.push(*first_char);

        while let Some(c) = self.reader.peek() {
            if !is_valid_variable_char(c) {
                break;
            }
            self.reader.advance();
            buffer.push(*c);
        }
        Token::from_string(buffer, pos)
    }

    fn handle_bang(&self) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();

        match self.reader.peek() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(BangEqual.at(pos.clone()))),
                _ => Ok(Bang.at(pos)),
            },
            None => unexpected_eof_err(pos),
        }
    }

    fn handle_eq(&self) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();

        match self.reader.peek() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(EqualEqual.at(pos.clone()))),
                _ => Ok(Equal.at(pos)),
            },
            None => unexpected_eof_err(pos),
        }
    }

    fn handle_le(&self) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();

        match self.reader.peek() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(LessEqual.at(pos.clone()))),
                _ => Ok(Less.at(pos)),
            },
            None => unexpected_eof_err(pos),
        }
    }

    fn handle_gr(&self) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();

        match self.reader.advance() {
            Some(c) => match c {
                '=' => self.advance_and(|| Ok(GreaterEqual.at(pos.clone()))),
                _ => Ok(Greater.at(pos)),
            },
            None => unexpected_eof_err(pos),
        }
    }

    fn handle_slash(&self) -> LoxResult<Token> {
        let pos = self.reader.curr_pos();

        match self.reader.advance() {
            Some(c) => match c {
                '/' => self.handle_comment(),
                _ => Ok(Slash.at(pos)),
            },
            None => unexpected_eof_err(pos),
        }
    }

    fn handle_comment(&self) -> LoxResult<Token> {
        self.reader.advance_until(|c: &char| {print!("{} - ", c); *c == '\n'});
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

fn unexpected_eof_err<A>(pos: Position) -> LoxResult<A> {
    ErrBuilder::new()
        .at(pos)
        .of_type(ScanningErr)
        .expected_but_found("next character", "end of file")
        .to_result()
}
