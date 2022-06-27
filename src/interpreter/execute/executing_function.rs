use crate::interpreter::parser::structure::Statement::Return;
use crate::interpreter::parser::structure::Statement;
use crate::interpreter::tokens::LoxValue;
use crate::interpreter::Visitor;
use crate::interpreter::tokens::Token;
use crate::interpreter::parser::structure::Program;
use crate::interpreter::execute::definitions::LoxObj;
use crate::interpreter::errors::LoxResult;
use crate::interpreter::execute::state::State;
use crate::interpreter::Executor;

pub struct FuncExecutor<'a> {
    executor: &'a Executor,
}

impl<'a>  FuncExecutor<'a>  {
    pub fn from(executor: &'a Executor) -> Self {
        FuncExecutor { executor }
    }
}

impl<'a> Visitor<Program, LoxResult<LoxObj>> for FuncExecutor<'a>  {
    fn visit(&mut self, program: &Program) -> LoxResult<LoxObj> {
        for stmt in program {
            let result = self.visit(stmt)?;
            if let Some(returned) = result {
                return Ok(returned)
            }
        };
        Ok(LoxObj::from(LoxValue::from(0)))
    }
}

impl Visitor<Statement, LoxResult<Option<LoxObj>>> for FuncExecutor {
    fn visit(&mut self, stmt: &Statement) -> LoxResult<Option<LoxObj>> {
        match stmt {
            Return(expr) => return Ok(Some(self.executor.visit(expr)?)),
            _ => {}
        };
        self.executor.visit(stmt)?;
        Ok(None)
    }
}
