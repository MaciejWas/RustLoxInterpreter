use crate::interpreter::errors::ErrType::ParsingErr;
use crate::interpreter::errors::ErrType::ScanningErr;
use crate::interpreter::errors::{LoxError, LoxResult};
use crate::interpreter::readers::TextReader;
use crate::interpreter::tokens::Equals;
use crate::interpreter::tokens::{Punct, Punct::*, Token, Tokenizable};
use crate::interpreter::readers::reader::ReaderBase;

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
                        self.handle_literal()
                    } else {
                        Self::scanning_err(
                            format!("Unrecognized character. Wtf do you mean by {:?}", c),
                            curr_pos,
                        )
                    }
                }
            },
            None => Ok(Eof.at(self.reader.pos())),
        }
    }

    fn handle_literal(&self) -> LoxResult<Token> {
        let mut buffer = String::new();
        self.reader.back(); // Function is called only after the reader finds the firse letter, so we have to go back
        let start = self.reader.pos();
        loop {
            match self.reader.advance() {
                Some(c) => {
                    if is_valid_variable_char(c) {
                        buffer.push(c)
                    } else {
                        self.reader.back();
                        break;
                    }
                }
                None => break,
            }
        }
        Token::from_string(buffer, start)
    }

    fn handle_bang(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => Ok(BangEqual.at(self.reader.pos())),
                _ => self.go_back_and_return(Bang),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_eq(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => Ok(EqualEqual.at(self.reader.pos())),
                _ => self.go_back_and_return(Equal),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_le(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => Ok(LessEqual.at(self.reader.pos())),
                _ => self.go_back_and_return(Less),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_gr(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => Ok(GreaterEqual.at(self.reader.pos())),
                _ => self.go_back_and_return(Greater),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_slash(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '/' => self.handle_comment(),
                _ => self.go_back_and_return(Slash),
            },
            None => unexpected_eof_err(self.reader.pos()),
        }
    }

    fn handle_comment(&self) -> LoxResult<Token> {
        self.reader.advance_until_newline();
        self.next_token()
    }

    fn go_back_and_return(&self, punct: Punct) -> LoxResult<Token> {
        self.reader.back().expect("Failed to go back");
        return Ok(punct.at(self.reader.pos()));
    }

    fn scanning_err<A>(text: String, pos: usize) -> LoxResult<A> {
        LoxError::new_err(text.to_string(), pos, ParsingErr)
    }
}

fn is_valid_variable_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '\'' || *c == '_' || *c == '"'
}

fn unexpected_eof_err<A>(pos: usize) -> LoxResult<A> {
    LoxError::new_err("Unexpected eof.".to_string(), pos, ScanningErr)
}
