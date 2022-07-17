#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Punct {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comme,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Mod,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Eof,
}
