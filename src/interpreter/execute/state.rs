//! Handling of bindings and scopes during runtime.

use crate::interpreter::errors::ErrType::RuntimeError;
use std::hash::Hash;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use crate::interpreter::tokens::Token;
use crate::interpreter::tokens::TokenValue;
use crate::interpreter::errors::position::Position;
use crate::interpreter::errors::ErrBuilder;
use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::execute::definitions::{LoxObj};
use std::collections::HashMap;
use std::vec::Vec;

type ObjId = u64;

fn hash_str(string: String) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

/// Simply a scope. Used inside loops, functions, classes, etc.
pub struct Scope {
    bindings: HashMap<String, LoxObj>,
    name: String,
}

impl Scope {
    pub fn new(name: String) -> Self {
        Scope {
            bindings: HashMap::with_capacity(100),
            name,
        }
    }

    pub fn bind(&mut self, identifier: Token, raw_obj: LoxObj) {
        let Token { val, pos } = identifier;
        let TokenValue::Id(name) = val;
        let id = hash_str(name);

        self.bindings.insert(name, raw_obj);
    }

    pub fn get(&self, identifier: Token) -> LoxResult<&mut LoxObj> {
        let Token { val, pos } = identifier;
        let TokenValue::Id(name) = val;

        let obj = self.bindings.get_mut(&name).ok_or(self.err()
            .with_pos(pos)
            .with_message(format!("Variable {:?} is not in scope", identifier))
            .build())?;

        return Ok( obj )
    }

    fn err(&self) -> ErrBuilder {
        ErrBuilder::new().of_type(LogicError)
    }
}

/// Wrapper around a stack of scopes
pub struct State {
    scope_stack: Vec<Scope>,
}

impl State {
    pub fn new() -> Self {
        let global_scope = Scope::new("Global scope".to_string());
        let mut scope_stack = Vec::new();
        scope_stack.push(global_scope);
        State { scope_stack }
    }

    fn get_curr_scope(&mut self) -> &mut Scope {
        self.scope_stack
            .last_mut()
            .unwrap_or_else(|| panic!("Global stack not present"))
    }

    pub fn push_new_scope(&mut self) {
        self.scope_stack.push(Scope {
            bindings: HashMap::new(),
            name: "placeholder".to_string(),
        })
    }

    pub fn pop_last_scope(&mut self) -> Option<()> {
        self.scope_stack.pop().map(|_| ())
    }

    pub fn bind(&mut self, identifier: Token, obj: LoxObj) {
        let Token { val, pos } = identifier;
        let TokenValue::Id(name) = val;

        let relevant_scope = self
            .scope_stack
            .iter_mut()
            .filter(|scope| scope.bindings.contains_key(&name))
            .next(); // first scope which contains this identifier

        return match relevant_scope {
            Some(scope) => {
                scope.bind(identifier, LoxObj::from(obj));
            }
            None => {
                self.get_curr_scope()
                    .bind(identifier, LoxObj::from(obj));
            }
        };
    }

    pub fn get(&self, identifier: &String, pos: &Position) -> LoxResult<&mut LoxObj> {
        for scope in self.scope_stack.iter().rev() {
            let scope_search = scope.bindings.get(identifier);
            if let Some(raw_obj) = scope.bindings.get(identifier) {
                return Ok(&mut raw_obj);
            }
        }
        self.err()
            .at(*pos)
            .with_message(format!("Variable {:?} is not in scope", identifier))
            .to_result()
    }

    fn err(&self) -> ErrBuilder {
        ErrBuilder::new().of_type(RuntimeError)
    }
}
