use super::tokens::{TokenType, TokenType::*, Token};
use super::errors::LoxError;
use super::text_reader::TextReader; 

type ScannerResult = Result<Token, LoxError>;

pub fn scan(source: String) -> Result<Box<Vec<Token>>, LoxError> {
    let reader = TextReader::new(source);
    let mut tokens = Box::new(Vec::new());

    loop {
        match next_token(&reader) {
            Ok(token) => tokens.push(token),
            Err(err) => return Err(err)
        }

        let last_token = tokens.last().expect("Failed to get last token.");
        if last_token.is_of_type(Eof) {
            break
        }
    }
    
    Ok(tokens)
}

fn next_token(
    reader: &TextReader,
) -> Result<Token, LoxError> {
    loop {
        match reader.advance() {
            Some(c) => match c {
                '('  => return Ok(LeftParen.at(reader.get_pos())),
                ')'  => return Ok(RightParen.at(reader.get_pos())),
                '{'  => return Ok(LeftBrace.at(reader.get_pos())),
                '}'  => return Ok(RightBrace.at(reader.get_pos())),
                ','  => return Ok(Comme.at(reader.get_pos())),
                '.'  => return Ok(Dot.at(reader.get_pos())),
                '-'  => return Ok(Minus.at(reader.get_pos())),
                '+'  => return Ok(Plus.at(reader.get_pos())),
                ';'  => return Ok(Semicolon.at(reader.get_pos())),
                '*'  => return Ok(Star.at(reader.get_pos())),
                '!'  => return handle_bang(&reader),
                '='  => return handle_eq(&reader),
                '>'  => return handle_gr(&reader),
                '<'  => return handle_le(&reader),
                '/'  => return handle_slash(&reader),
                ' '  => return next_token(&reader),
                '\t' => return next_token(&reader),
                '\n' => return next_token(&reader),
                 _   => return handle_literal(&reader) // Err(failed_to_parse_at(reader.get_pos())) 
            },
            None => return Ok(Eof.at(reader.get_pos()))
        }
    };
}

fn handle_literal(reader: &TextReader) -> ScannerResult {
    let mut buffer = String::new();

    loop {
        match reader.advance() {
            Some(c) => {
                if c.is_alphanumeric() || c == '\'' || c == '_' {
                    buffer.push(c)
                } else {
                    break
                }
            },
            None => break
        }
    }

    Token::from_string(buffer)
}

fn handle_bang(reader: &TextReader) -> Result<Token, LoxError> {
    match reader.advance() {
        Some(c) => match c {
            '=' => return Ok(BangEqual.at(reader.get_pos())),
             _  => return go_back_and_return(&reader, Bang)
        }
        None => return Err(failed_to_parse_at(reader.get_pos()))
    }
}

fn handle_eq(reader: &TextReader) -> Result<Token, LoxError> {
    match reader.advance() {
        Some(c) => match c {
            '=' => return Ok(EqualEqual.at(reader.get_pos())),
             _  => return go_back_and_return(reader, Equal)
        }
        None => return Err(failed_to_parse_at(reader.get_pos()))
    }
}

fn handle_le(reader: &TextReader) -> Result<Token, LoxError> {
    match reader.advance() {
        Some(c) => match c {
            '=' => return Ok(LessEqual.at(reader.get_pos())),
             _  => return go_back_and_return(&reader, Less)
        },
        None => return Err(failed_to_parse_at(reader.get_pos()))
    }
}

fn handle_gr(reader: &TextReader) -> Result<Token, LoxError> {
    match reader.advance() {
        Some(c) => match c {
            '=' => return Ok(GreaterEqual.at(reader.get_pos())),
             _  => return go_back_and_return(reader, Greater),
        }
        None => return Err(failed_to_parse_at(reader.get_pos()))
    }
}

fn handle_slash(reader: &TextReader) -> Result<Token, LoxError> {
    match reader.advance() {
        Some(c) => match c {
            '/' => return handle_comment(&reader),
                _  => return go_back_and_return(reader, Slash),
        }
        None => return Err(failed_to_parse_at(reader.get_pos()))
    }
}

fn handle_comment(reader: &TextReader) -> Result<Token, LoxError> {
    let comment_pos = reader.get_pos();
    reader.advance_until_newline();
    Ok(Comment.at(comment_pos))
}

fn go_back_and_return(reader: &TextReader, t: TokenType) -> Result<Token, LoxError> {
    reader.back().expect("Failed to go back");
    return Ok(t.at(reader.get_pos()))
}

fn failed_to_parse_at(pos: usize) -> LoxError {
    LoxError::ParsingError(format!("Failed to parse. I'm lost at {}", pos))
}