use crate::interpreter::errors::{LoxResult, LoxError};
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kwd {
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,  
    Comment(String)
} 

impl From<&String> for Kwd {
    pub fn from(string: &String) -> LoxResult<Self> {
        match string.as_str() {
            "and"    => Ok(Self::And),
            "class"  => Ok(Self::Class),
            "else"   => Ok(Self::Else),
            "false"  => Ok(Self::False),
            "fun"    => Ok(Self::Fun),
            "for"    => Ok(Self::For),
            "if"     => Ok(Self::If),
            "print"  => Ok(Self::Print),
            "return" => Ok(Self::Return),
            "super"  => Ok(Self::Super),
            "this"   => Ok(Self::This),
            "true"   => Ok(Self::True),
            "var"    => Ok(Self::Var),
            "while"  => Ok(Self::While),
            _        => LoxError::tokenizing_err(String::from("Failed to build Kwd from string"))
        }
    }
}
