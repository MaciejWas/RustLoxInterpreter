use crate::interpreter::execute::inbuilt::LoxObj::Plain;
use crate::interpreter::tokens::LoxValue::Integer;
use crate::interpreter::errors::ErrBuilder;
use crate::interpreter::errors::ErrType::RuntimeError;
use crate::interpreter::parser::locator::locate;
use crate::interpreter::tokens::LoxValue;
use crate::interpreter::{
    errors::position::Position,
    errors::LoxResult,
    execute::{
        definitions::{LoxObj},
        operations::{binary_operations, eval_err, unary_op},
    },
    parser::structure::*,
    parser::visitor::*,
    tokens::Token,
    tokens::TokenValue,
};

fn runtime_err_at(pos: Position) -> ErrBuilder {
    ErrBuilder::new().of_type(RuntimeError).at(pos)
} 


fn modulo(a: LoxObj, b: LoxObj, pos: Position) -> LoxResult<LoxObj> {
    let cast_to_int = |obj: LoxObj| match obj {
        Plain(Integer(x)) => Ok(x),
        _ => runtime_err_at(pos).is_not(obj.to_string(), "integer").to_result()
    };

    let val_a = cast_to_int(a)?;
    let val_b = cast_to_int(b)?;

    Ok(Plain(Integer(val_a % val_b)))
}


