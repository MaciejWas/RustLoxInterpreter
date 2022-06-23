use crate::interpreter::errors::ErrBuilder;
use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::execute::definitions::{LoxObj, RawLoxObject};
use crate::interpreter::tokens::position_of;
use crate::interpreter::tokens::Token;
use std::collections::HashMap;
use std::vec::Vec;

pub struct Scope {
    bindings: HashMap<String, RawLoxObject>,
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

    pub fn bind(&mut self, identifier: String, obj: LoxObj) -> LoxResult<()> {
        for scope in self.scope_stack.iter_mut() {
            if scope.bindings.contains_key(&identifier) {
                scope.bindings.insert(identifier, RawLoxObject::from(obj));
                return Ok(());
            }
        }

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

    pub fn get(&self, identifier: &String) -> Option<LoxObj> {
        for scope in self.scope_stack.iter().rev() {
            let last_scope_search = scope.bindings.get(identifier);
            if last_scope_search.is_some() {
                return last_scope_search.map(LoxObj::from);
            }
        }
        None
    }

    pub fn as_object(&self, token: &Token) -> LoxResult<LoxObj> {
        let err = self
            .err()
            .with_pos(position_of(&token))
            .while_(format!("Mapping {:?} to lox object", token));

        match token {
            Token::IdentifierToken(identifier, _) => self.get(identifier).ok_or(
                err.with_message(format!("Variable {:?} is not in scope", identifier))
                    .build(),
            ),
            Token::ValueToken(lox_val, _) => {
                Ok(LoxObj::from(&RawLoxObject::Plain(lox_val.clone())))
            }
            _ => Err(err.is_not(token, "a lox object").build()),
        }
    }

    fn err(&self) -> ErrBuilder {
        ErrBuilder::new().of_type(LogicError)
    }
}
