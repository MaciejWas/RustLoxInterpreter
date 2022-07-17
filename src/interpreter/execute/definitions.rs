use crate::interpreter::tokens::TokenValue::Id;
use crate::interpreter::tokens::Token;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::interpreter::parser::structure::ClassDefinition;
use crate::interpreter::parser::structure::FunctionDefinition;
use crate::interpreter::tokens::LoxValue;

/// Value which is held in the bindings
#[derive(Debug, Clone)]
pub enum LoxObj {
    Object(HashMap<String, LoxObj>),
    Plain(LoxValue),
    Fun(FunctionDefinition),
    Inbuilt(String),
    Class(ClassDefinition),
}

pub type LoxObjRef = Rc<RefCell<LoxObj>>;

impl LoxObj {
    pub fn set(&mut self, id: Token, obj: LoxObj) {
        let name = match id.val {
            Id(name) => name,
            _ => panic!("Attempted to assign value to non-identifier.")
        };
        let mut map = match self {
            LoxObj::Object(map) => map,
            _ => panic!("cannot assign to this object")
        };

        map.insert(name, obj);
    }

    pub fn to_string(&self) -> String {
        match self {
            LoxObj::Plain(val) => format!("{:?}", val),
            LoxObj::Fun(function_def) => format!("Function {:?}", function_def.name),
            LoxObj::Class(defn) => format!("{:?}", defn.name),
            LoxObj::Object(_) => format!("Instance"),
            LoxObj::Inbuilt(name) => format!("Inbuilt Function {}", name)
        }
    }

    pub fn to_value(self) -> Option<LoxValue> {
        match self {
            LoxObj::Plain(val) => Some(val),
            _ => None
        }
    }

    pub fn to_ref(self) -> LoxObjRef {
        Rc::new(RefCell::from(self))
    }
}