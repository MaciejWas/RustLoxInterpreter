use crate::interpreter::errors::LoxResult;
use crate::interpreter::parser::structure::FunctionDefinition;
use crate::interpreter::tokens::LoxValue;

pub enum RawLoxObject {
    Plain(LoxValue),
    Fun(FunctionDefinition),
    Class(()),
}

impl RawLoxObject {
    pub fn to_string(&self) -> String {
        match self {
            RawLoxObject::Plain(lox_val) => format!("{:?}", lox_val),
            RawLoxObject::Fun(function_def) => format!("Function({:?})", function_def.name),
            _ => panic!("not implemented:("),
        }
    }
}

pub struct LoxObj {
    ptr: Box<RawLoxObject>,
}

impl LoxObj {
    pub fn new(raw: RawLoxObject) -> Self {
        LoxObj { ptr: Box::new(raw) }
    }

    pub fn apply<F>(self, f: F) -> LoxResult<LoxObj>
    where
        F: Fn(RawLoxObject) -> LoxResult<RawLoxObject>,
    {
        f(*self.ptr).map(LoxObj::new)
    }

    pub fn transform<F, A>(self, f: F) -> A
    where
        F: Fn(RawLoxObject) -> A,
    {
        f(*self.ptr)
    }

    pub fn to_value(self) -> Option<LoxValue> {
        match *self.ptr {
            RawLoxObject::Plain(x) => Some(x),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self.ptr.as_ref() {
            RawLoxObject::Plain(val) => format!("{:?}", val),
            RawLoxObject::Fun(function_def) => format!("Function {}", function_def.name),
            RawLoxObject::Class(_) => "fadsf fasdf".to_string(),
        }
    }
}

impl From<LoxValue> for LoxObj {
    fn from(lox_val: LoxValue) -> LoxObj {
        LoxObj::new(RawLoxObject::Plain(lox_val))
    }
}

impl From<FunctionDefinition> for LoxObj {
    fn from(def: FunctionDefinition) -> LoxObj {
        LoxObj::new(RawLoxObject::Fun(def))
    }
}

impl From<LoxObj> for Option<LoxValue> {
    fn from(lox_obj: LoxObj) -> Option<LoxValue> {
        match *lox_obj.ptr {
            RawLoxObject::Plain(lox_val) => Some(lox_val),
            _ => None,
        }
    }
}

impl From<LoxObj> for Option<FunctionDefinition> {
    fn from(lox_obj: LoxObj) -> Option<FunctionDefinition> {
        match *lox_obj.ptr {
            RawLoxObject::Fun(fun_definition) => Some(fun_definition),
            _ => None,
        }
    }
}

impl From<LoxObj> for Option<bool> {
    fn from(lox_obj: LoxObj) -> Option<bool> {
        let bool_mapper = |raw_lox_obj: RawLoxObject| match raw_lox_obj {
            RawLoxObject::Plain(lox_val) => Some(bool::from(lox_val)),
            _ => None,
        };
        lox_obj.transform(bool_mapper)
    }
}

impl From<&RawLoxObject> for LoxObj {
    fn from(raw: &RawLoxObject) -> Self {
        match raw {
            RawLoxObject::Fun(fun_definition) => LoxObj::from(fun_definition.clone()),
            RawLoxObject::Plain(lox_value) => LoxObj::from(lox_value.clone()),
            _ => panic!("Not implemented!"),
        }
    }
}

impl From<LoxObj> for RawLoxObject {
    fn from(lox_obj: LoxObj) -> Self {
        *lox_obj.ptr
    }
}
