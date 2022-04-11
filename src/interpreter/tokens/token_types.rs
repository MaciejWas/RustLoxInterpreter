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

impl Punct {
    pub fn is_eq_or_neq(&self) -> bool {
        match self {
            Self::EqualEqual => true, Self::BangEqual => true,
            _ => false
        }
    }

    pub fn is_comparison(&self) -> bool {
        match self {
            Self::Greater => true, Self::GreaterEqual => true, Self::Less => true, Self::LessEqual => true,
            _ => false
        }
    }

    pub fn is_plus_minus(&self) -> bool {
        match self {
            Self::Plus => true, Self::Minus => true,
            _ => false
        }
    }

    pub fn is_mul_div(&self) -> bool {
        match self {
            Self::Dot => true, Self::Slash => true,
            _ => false
        }
    }

    pub fn is_neg(&self) -> bool {
        match self {
            Self::Minus => true,
            _ => false
        }
    }
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
