use crate::interpreter::execute::definitions::LoxObj;
use crate::interpreter::errors::position::Position;
use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::*;
use crate::interpreter::tokens::*;

pub fn eval_err() -> ErrBuilder {
    ErrBuilder::new().of_type(LogicError)
}

pub fn handle(op: &Token, acc: LoxObj, val: LoxObj) -> LoxResult<LoxObj> {
    let to_value = |x: LoxObj| x.to_value().ok_or(eval_err().is_not(acc, "value").build());
    let acc = to_value(acc)?;
    let val = to_value(val)?;

    let result = match op.as_punct()? {
        Punct::Star => star(acc, val, op.pos),
        Punct::Plus => plus(acc, val, op.pos),
        Punct::Minus => minus(acc, val, op.pos),
        Punct::EqualEqual => eq(acc, val, op.pos),
        Punct::BangEqual => neq(acc, val, op.pos),
        _ => Err(eval_err()
            .with_pos(op.pos)
            .is_not(op, "a valid lox operation")
            .build()),
    }?;

    return Ok(LoxObj::Plain(result));
}

fn plus(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x + y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x || *y)),
        _ => eval_err()
            .cant_perform_a_on_b_and_c("plus", acc, val)
            .with_pos(pos)
            .to_result(),
    }
}

fn star(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x * y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && *y)),
        _ => eval_err()
            .cant_perform_a_on_b_and_c("star", acc, val)
            .with_pos(pos)
            .to_result(),
    }
}

fn minus(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x - y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && !*y)),
        _ => eval_err()
            .cant_perform_a_on_b_and_c("minus", acc, val)
            .with_pos(pos)
            .to_result(),
    }
}

fn eq(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Boolean(x == y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(x == y)),
        (LoxValue::String(x), LoxValue::String(y)) => Ok(LoxValue::Boolean(x == y)),
        _ => eval_err()
            .cant_perform_a_on_b_and_c("equality check", acc, val)
            .with_pos(pos)
            .to_result(),
    }
}

fn neq(acc: LoxValue, val: LoxValue, pos: Position) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Boolean(x != y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(x != y)),
        (LoxValue::String(x), LoxValue::String(y)) => Ok(LoxValue::Boolean(x != y)),
        _ => eval_err()
            .cant_perform_a_on_b_and_c("inequality check", acc, val)
            .with_pos(pos)
            .to_result(),
    }
}
