use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Clone, Eq, PartialEq)]
pub enum LoxValue {
    Integer(i32),
    Boolean(bool),
    String(String),
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Integer(x) => write!(f, "Int({})", x),
            Self::Boolean(x) => write!(f, "Bool({})", x),
            Self::String(x) => write!(f, "String({})", x),
        }
    }
}

impl From<i32> for LoxValue {
    fn from(x: i32) -> Self {
        LoxValue::Integer(x)
    }
}

impl From<bool> for LoxValue {
    fn from(x: bool) -> Self {
        LoxValue::Boolean(x)
    }
}

impl From<String> for LoxValue {
    fn from(x: String) -> Self {
        LoxValue::String(x)
    }
}

impl From<LoxValue> for bool {
    fn from(lox_val: LoxValue) -> bool {
        match lox_val {
            LoxValue::Integer(x) => x != 0,
            LoxValue::Boolean(x) => x,
            LoxValue::String(x) => x.len() > 0,
        }
    }
}
