use std::fmt;


#[derive(Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comme, Dot, Minus, Plus, Semicolon, Slash, Star,
  
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
  
    // Literals.
    Identifier, String, Number,
  
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
  
    Eof
}

impl fmt::Display for TokenType {   
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: u32
}

impl fmt::Display for Token {   
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[Token: {}, {} | at {}]", self.ttype, self.lexeme, self.line)
    }
}