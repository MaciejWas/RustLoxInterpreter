use std::collections::HashMap;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::parser::structure::ClassDefinition;
use crate::interpreter::parser::structure::FunctionDefinition;
use crate::interpreter::tokens::LoxValue;

/// Value which is held in the bindings
#[derive(Debug)]
pub enum LoxObj {
    Object(HashMap<String, LoxObj>),
    Plain(LoxValue),
    Fun(FunctionDefinition),
    Class(ClassDefinition),
}

type LoxObjRef = Box<LoxObj>;

impl LoxObj {
    pub fn to_string(&self) -> String {
        match self {
            LoxObj::Plain(val) => format!("{:?}", val),
            LoxObj::Fun(function_def) => format!("Function {}", function_def.name),
            LoxObj::Class(defn) => format!("{:?}", defn.name),
            LoxObj::Object(_) => format!("Instance"),
        }
    }

    pub fn to_value(self) -> Option<LoxValue> {
        match self {
            LoxObj::Plain(val) => Some(val),
            _ => None
        }
    }
}