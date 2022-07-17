use crate::interpreter::execute::operations::unary_operations::LoxObj::Plain;
use crate::interpreter::errors::{position::Position, ErrBuilder, ErrType::LogicError, LoxResult};
use crate::interpreter::execute::definitions::LoxObj;
use crate::interpreter::tokens::{LoxValue::Boolean, LoxValue::Integer, Punct, Token};

fn unary_op_err() -> ErrBuilder {
    ErrBuilder::new()
        .of_type(LogicError)
        .while_("evaluating unary expression")
}

pub fn negate(raw: &LoxObj, at: Position) -> LoxResult<LoxObj> {
    match raw {
        Plain(Boolean(b)) => Ok(Plain(Boolean(!b))),
        Plain(Integer(b)) => Ok(Plain(Integer(-b))),
        _ => unary_op_err()
            .with_pos(at)
            .with_message(format!("Cannot negate {:?}", raw.to_string()))
            .to_result(),
    }
}

/// Applies `op` to `right`
pub fn unary_op(op: &Token, right: &LoxObj) -> LoxResult<LoxObj> {
    let pos = op.pos;
    let op = op.as_punct()?;
    match op {
        Minus => negate(right, pos),
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
