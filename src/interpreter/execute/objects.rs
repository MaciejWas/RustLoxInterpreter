use crate::interpreter::execute::state::Scope;
use std::collections::HashMap;
use crate::interpreter::parser::structure::Program;
use crate::interpreter::tokens::Token;
use crate::interpreter::tokens::LoxValue;

pub enum LoxObject {
    Plain(LoxValue),
    Fun(Fun),
    Class(Class)
}

struct Fun {
    args: Vec<Token>,
    body: Program,
    ret: Token,
}

struct Class {
    fields: Scope,
    methods: HashMap<String, Fun>
}