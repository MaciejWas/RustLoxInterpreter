use crate::interpreter::execute::inbuilt::RawLoxObject::Plain;
use crate::interpreter::parser::locator::locate;
use crate::interpreter::tokens::LoxValue;
use crate::interpreter::{
    errors::position::Position,
    errors::LoxResult,
    execute::{
        definitions::{LoxObj, RawLoxObject},
        operations::{binary_operations, eval_err, unary_op},
    },
    parser::structure::*,
    parser::visitor::*,
    tokens::Token,
    tokens::TokenValue,
};