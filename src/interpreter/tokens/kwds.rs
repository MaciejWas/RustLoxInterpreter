use crate::interpreter::errors::ErrBuilder;
use crate::interpreter::errors::ErrType::ScanningErr;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::tokens::Position;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kwd {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Comment(String),
}

impl Kwd {
    pub fn is_valid(string: &String) -> bool {
        let pos = (0, 0).into();
        match Self::from(string, pos) {
            Ok(_) => true,
            _ => false,
        }
    }

    pub fn from(string: &String, pos: Position) -> LoxResult<Self> {
        match string.as_str() {
            "and" => Ok(Self::And),
            "class" => Ok(Self::Class),
            "else" => Ok(Self::Else),
            "fun" => Ok(Self::Fun),
            "for" => Ok(Self::For),
            "if" => Ok(Self::If),
            "print" => Ok(Self::Print),
            "return" => Ok(Self::Return),
            "super" => Ok(Self::Super),
            "this" => Ok(Self::This),
            "var" => Ok(Self::Var),
            "while" => Ok(Self::While),
            _ => ErrBuilder::new()
                .at(pos)
                .of_type(ScanningErr)
                .with_message(format!("Could not create keywork token from {:?}", string))
                .to_result(),
        }
    }
}
