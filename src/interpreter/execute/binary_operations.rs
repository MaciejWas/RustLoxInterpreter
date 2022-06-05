use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::*;
// use crate::interpreter::tokens::From
use crate::interpreter::execute::binary_operations::position::Position;
use crate::interpreter::tokens::*;

pub fn eval_err() -> ErrBuilder {
    ErrBuilder::new().of_type(LogicError)
}

pub fn cant_perform_binary_on<O: std::fmt::Debug, A: std::fmt::Debug, B: std::fmt::Debug>(
    op: O,
    a: A,
    b: B,
) -> ErrBuilder {
    eval_err().with_message(format!("Can't perform {:?} on {:?} and {:?}", op, a, b))
}

pub fn handle(op: &Token, acc: LoxValue, val: LoxValue, curr_pos: Position) -> LoxResult<LoxValue> {
    match op.as_punct()? {
        Punct::Star => star(acc, val, curr_pos),
        Punct::Plus => plus(acc, val, curr_pos),
        Punct::Minus => minus(acc, val, curr_pos),
        Punct::EqualEqual => eq(acc, val, curr_pos),
        Punct::BangEqual => neq(acc, val, curr_pos),
        _ => Err(eval_err()
            .with_pos(curr_pos)
            .is_not(op, "a valid lox operation")
            .build()),
    }
}

fn plus(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x + y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x || *y)),
        _ => Err(cant_perform_binary_on("plus", acc, val)
            .with_pos(pos)
            .build()),
    }
}

fn star(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x * y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && *y)),
        _ => Err(cant_perform_binary_on("mul", acc, val)
            .with_pos(pos)
            .build()),
    }
}

fn minus(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x - y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && !*y)),
        _ => Err(cant_perform_binary_on("subtraction", acc, val)
            .with_pos(pos)
            .build()),
    }
}

fn eq(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Boolean(x == y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(x == y)),
        (LoxValue::String(x), LoxValue::String(y)) => Ok(LoxValue::Boolean(x == y)),
        _ => Err(cant_perform_binary_on("equality check", acc, val)
            .with_pos(pos)
            .build()),
    }
}

fn neq(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Boolean(x != y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(x != y)),
        (LoxValue::String(x), LoxValue::String(y)) => Ok(LoxValue::Boolean(x != y)),
        _ => Err(cant_perform_binary_on("not-equality check", acc, val)
            .with_pos(pos)
            .build()),
    }
}