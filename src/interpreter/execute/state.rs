use crate::interpreter::errors::LoxResult;
use crate::interpreter::tokens::LoxValue;
use std::collections::HashMap;
use std::vec::Vec;

struct Scope {
    bindings: HashMap<String, LoxValue>,
    name: String,
}

pub struct State {
    scope_stack: Vec<Scope>,
}

impl State {
    pub fn new() -> Self {
        let global_scope = Scope {
            bindings: HashMap::new(),
            name: "Global scope".to_string(),
        };
        let mut scope_stack = Vec::new();
        scope_stack.push(global_scope);
        State { scope_stack }
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

    pub fn bind(&mut self, identifier: String, value: LoxValue) -> LoxResult<()> {
        self.scope_stack
            .last_mut()
            .ok_or_else(|| {
                panic!(
                    "Global stack is not present! (While binding {:?} to {:?})",
                    value, identifier
                )
            })?
            .bindings
            .insert(identifier, value);

        Ok(())
    }

    pub fn get(&self, identifier: String) -> Option<&LoxValue> {
        for scope in self.scope_stack.iter().rev() {
            let last_scope_search = scope.bindings.get(&identifier);
            if last_scope_search.is_some() {
                return last_scope_search;
            }
        }
        None
    }
}
