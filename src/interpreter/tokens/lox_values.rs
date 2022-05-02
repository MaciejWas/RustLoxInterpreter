#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoxValue {
    Integer(i32),
    Boolean(bool),
    String(String)
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