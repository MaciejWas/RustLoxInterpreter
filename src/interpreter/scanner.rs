use super::tokens::{Tokenizable, Token, Token::*, token_types::{Punct, Punct::*, Kwd::*}};
use super::errors::{LoxResult, LoxError};
use super::text_reader::TextReader;

pub struct ScannerOutput {
    pub tokens: Vec<Token>
}

pub struct Scanner {
    reader: TextReader,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            reader: TextReader::new(source)
        }
    }

    pub fn scan(self) -> LoxResult<ScannerOutput> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            tokens.push(token.clone());
            if is_eof(&token) { break }
        }
        
        Ok(ScannerOutput{
            tokens: tokens
        })
    }

    fn next_token(
        &self,
    ) -> LoxResult<Token> {
        loop {
            match self.reader.advance() {
                Some(c) => match c {
                    '('  => return Ok(LeftParen.at(self.reader.get_pos())),
                    ')'  => return Ok(RightParen.at(self.reader.get_pos())),
                    '{'  => return Ok(LeftBrace.at(self.reader.get_pos())),
                    '}'  => return Ok(RightBrace.at(self.reader.get_pos())),
                    ','  => return Ok(Comme.at(self.reader.get_pos())),
                    '.'  => return Ok(Dot.at(self.reader.get_pos())),
                    '-'  => return Ok(Minus.at(self.reader.get_pos())),
                    '+'  => return Ok(Plus.at(self.reader.get_pos())),
                    ';'  => return Ok(Semicolon.at(self.reader.get_pos())),
                    '*'  => return Ok(Star.at(self.reader.get_pos())),
                    '!'  => return self.handle_bang(),
                    '='  => return self.handle_eq(),
                    '>'  => return self.handle_gr(),
                    '<'  => return self.handle_le(),
                    '/'  => return self.handle_slash(),
                    ' '  => return self.next_token(),
                    '\t' => return self.next_token(),
                    '\n' => return self.next_token(),
                    _   => if is_valid_variable_char(c) {
                            return self.handle_literal()
                        } else {
                            println!("Wtf do you mean by {:?} ??", c); //TODO: this is debuggign message
                            return Err(LoxError::ParsingError("Unrecognized character".to_string()))
                        }
                },
                None => return Ok(Eof.at(self.reader.get_pos()))
            }
        };
    }

    fn handle_literal(&self) -> LoxResult<Token> {
        let mut buffer = String::new();
        self.reader.back(); // Function is called only after the reader finds the firse letter, so we have to go back
        let start = self.reader.get_pos();
        loop {
            match self.reader.advance() {
                Some(c) => {
                    if is_valid_variable_char(c) {
                        buffer.push(c)
                    } else {
                        self.reader.back();
                        break
                    }
                },
                None => break
            }
        }
        Token::from_string(buffer, start)
    }

    fn handle_bang(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => return Ok(BangEqual.at(self.reader.get_pos())),
                _  => return self.go_back_and_return(Bang)
            }
            None => return Err(failed_to_scan_at(self.reader.get_pos()))
        }
    }

    fn handle_eq(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => return Ok(EqualEqual.at(self.reader.get_pos())),
                _  => return self.go_back_and_return(Equal)
            }
            None => return Err(failed_to_scan_at(self.reader.get_pos()))
        }
    }

    fn handle_le(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => return Ok(LessEqual.at(self.reader.get_pos())),
                _  => return self.go_back_and_return(Less)
            },
            None => return Err(failed_to_scan_at(self.reader.get_pos()))
        }
    }

    fn handle_gr(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '=' => return Ok(GreaterEqual.at(self.reader.get_pos())),
                _  => return self.go_back_and_return(Greater),
            }
            None => return Err(failed_to_scan_at(self.reader.get_pos()))
        }
    }

    fn handle_slash(&self) -> LoxResult<Token> {
        match self.reader.advance() {
            Some(c) => match c {
                '/' => return self.handle_comment(),
                    _  => return self.go_back_and_return(Slash),
            }
            None => return Err(failed_to_scan_at(self.reader.get_pos()))
        }
    }

    fn handle_comment(&self) -> LoxResult<Token> {
        let comment_pos = self.reader.get_pos();
        self.reader.advance_until_newline();
        self.next_token()
    }

    fn go_back_and_return(&self, punct: Punct) -> LoxResult<Token> {
        self.reader.back().expect("Failed to go back");
        return Ok(punct.at(self.reader.get_pos()))
    }
}

fn is_valid_variable_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\'' || c == '_' || c == '"'
}

fn failed_to_scan_at(pos: usize) -> LoxError {
    LoxError::ScanningError(format!("Failed to parse. I'm lost at {}", pos))
}

fn is_eof(t: &Token) -> bool {
    match t {
        PunctToken(punct, _) => punct.eq(&Eof),
        _ => false
    }
}
