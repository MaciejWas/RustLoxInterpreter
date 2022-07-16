//! Handling of bindings and scopes during runtime.

use crate::interpreter::errors::position::Position;
use crate::interpreter::errors::ErrBuilder;
use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::execute::definitions::{LoxObj, RawLoxObject};
use std::collections::HashMap;
use std::vec::Vec;

/// Simply a scope. Used inside loops, functions, classes, etc.
pub struct Scope {
    bindings: HashMap<String, RawLoxObject>,
    name: String,
}

impl Scope {
    fn new(name: String) -> Self {
        Scope {
            bindings: HashMap::new(),
            name,
        }
    }
}

pub struct State {
    scope_stack: Vec<Scope>,
    namespaces: HashMap<usize, Scope>,
}

impl State {
    pub fn new() -> Self {
        let global_scope = Scope {
            bindings: HashMap::new(),
            name: "Global scope".to_string(),
        };
        let mut scope_stack = Vec::new();
        scope_stack.push(global_scope);
        State {
            scope_stack,
            namespaces: HashMap::new(),
        }
    }

    pub fn assign_namespace(&mut self, obj: &mut LoxObj) -> LoxResult<()> {
        let scope = Scope::new("class scope".to_string());
        let id = match self.namespaces.keys().max() {
            Some(x) => x + 1,
            None => 0,
        };
        self.namespaces.insert(id, scope);
        obj.set_namespace(id);
        Ok(())
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

    pub fn bind(&mut self, identifier: String, obj: LoxObj, pos: Position) -> LoxResult<()> {
        let could_not_insert_into_scope_err = self
            .err()
            .at(pos)
            .with_message("Could not insert into scope".to_string())
            .build();

        let relevant_scope = self
            .scope_stack
            .iter_mut()
            .filter(|scope| scope.bindings.contains_key(&identifier))
            .last();

        return match relevant_scope {
            Some(scope) => {
                let result = scope.bindings.insert(identifier, RawLoxObject::from(obj));
                result.ok_or_else(|| could_not_insert_into_scope_err)?;
                Ok(())
            }
            None => {
                self.scope_stack
                    .last_mut()
                    .ok_or_else(|| {
                        panic!(
                            "Global stack is not present! (While binding {:?} to {:?})",
                            obj.to_string(),
                            identifier
                        )
                    })?
                    .bindings
                    .insert(identifier, RawLoxObject::from(obj));
                Ok(())
            }
        };
    }

    pub fn get(&self, identifier: &String, pos: Position) -> LoxResult<LoxObj> {
        for scope in self.scope_stack.iter().rev() {
            let last_scope_search = scope.bindings.get(identifier);
            if last_scope_search.is_some() {
                return Ok(last_scope_search.map(LoxObj::from).unwrap());
            }
        }
        self.err()
            .with_pos(pos)
            .with_message(format!("Variable {:?} is not in scope", identifier))
            .to_result()
    }

    fn err(&self) -> ErrBuilder {
        ErrBuilder::new().of_type(LogicError)
    }
}
