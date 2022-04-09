#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoxValue {
    Integer(i32),
    Boolean(bool),
    String(String)
}

impl LoxValue {
    pub fn from_int(x: i32) -> Self {
        Self::Integer(x)
    }

    pub fn from_bool(x: bool) -> Self {
        Self::Boolean(x)
    }

    pub fn from_string(x: String) -> Self {
        Self::String(x)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Punct {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comme, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kwd {
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
  
    Eof, Comment
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Identifier {
    Identifier(String)
}

impl Identifier {
    pub fn from(text: String) -> Self {
        Self::Identifier(text)
    }
}
