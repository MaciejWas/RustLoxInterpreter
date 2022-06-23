use crate::interpreter::errors::{position::Position, ErrBuilder, ErrType::LogicError, LoxResult};
use crate::interpreter::execute::definitions::LoxObj;
use crate::interpreter::execute::definitions::RawLoxObject;
use crate::interpreter::tokens::{position_of, LoxValue::Boolean, LoxValue::Integer, Punct, Token};

fn unary_op_err() -> ErrBuilder {
    ErrBuilder::new()
        .of_type(LogicError)
        .while_("evaluating unary expression")
}

pub fn negate(raw: RawLoxObject, at: Position) -> LoxResult<RawLoxObject> {
    match raw {
        RawLoxObject::Plain(Boolean(b)) => Ok(RawLoxObject::Plain(Boolean(!b))),
        RawLoxObject::Plain(Integer(b)) => Ok(RawLoxObject::Plain(Integer(-b))),
        _ => unary_op_err()
            .with_pos(at)
            .with_message(format!("Cannot negate {:?}", raw.to_string()))
            .to_result(),
    }
}

/// Applies `op` to `right`
pub fn unary_op(op: &Token, right: RawLoxObject) -> LoxResult<RawLoxObject> {
    let pos = position_of(&op);
    let op = op.as_punct()?;
    match op {
        Punct::Minus => negate(right, pos),
        _ => unary_op_err()
            .with_pos(pos)
            .with_message(format!(
                "applying {:?} on {:?} as an unary operator is not supported.",
                op,
                right.to_string()
            ))
            .to_result(),
    }
}
