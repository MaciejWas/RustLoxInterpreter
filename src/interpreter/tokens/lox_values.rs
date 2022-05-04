use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;

#[derive(Clone, Eq, PartialEq)]
pub enum LoxValue {
    Integer(i32),
    Boolean(bool),
    String(String)
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

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;
    use super::LoxValue;

    quickcheck! {
        fn quickcheck_lox_value(x: i32, y: bool, z: String) -> bool {
            let _ = LoxValue::from(x);
            let _ = LoxValue::from(y);
            let _ = LoxValue::from(z);
            true
        }
    }
}
