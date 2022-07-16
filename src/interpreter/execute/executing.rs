//! A Visitor-style executor for `Vec<Statement>`.

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
use std::iter::zip;

use super::state::State;

pub struct Evaluated {
    pub returned: Option<LoxObj>,
}

impl Evaluated {
    fn nil() -> Self {
        Evaluated { returned: None }
    }
    fn returned(returned: LoxObj) -> Self {
        Evaluated {
            returned: Some(returned),
        }
    }
}

pub struct Executor {
    pub state: State,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            state: State::new(),
        }
    }

    pub fn from(state: State) -> Self {
        Executor { state }
    }

    /// Creates new scope, does F, pops last scope. Generally used every time the executor goes into curly brackets.
    pub fn scoped<F, A>(&mut self, f: F) -> A
    where
        F: Fn(&mut Self) -> A,
    {
        self.state.push_new_scope();
        let result = f(self);
        self.state
            .pop_last_scope()
            .unwrap_or_else(|| panic!("Could not pop last scope!"));
        result
    }
}

impl Visitor<Program, LoxResult<Evaluated>> for Executor {
    fn visit(&mut self, p: &Program) -> LoxResult<Evaluated> {
        for stmt in p.iter() {
            let evaluated_stmt = self.visit(stmt)?;
            if evaluated_stmt.returned.is_some() {
                return Ok(Evaluated::from(evaluated_stmt));
            }
        }
        Ok(Evaluated { returned: None })
    }
}

impl Visitor<Statement, LoxResult<Evaluated>> for Executor {
    fn visit(&mut self, stmt: &Statement) -> LoxResult<Evaluated> {
        match stmt {
            Statement::Expr(expr) => {
                self.visit(expr)?;
            }
            Statement::Print(expr) => {
                let evaluated = self.visit(expr)?;
                println!("{}", evaluated.to_string())
            }
            Statement::If(cond, program) => {
                let cond_evaluated = self.visit(cond)?;
                match Option::<bool>::from(cond_evaluated) {
                    Some(true) => return self.scoped(|v| v.visit(program)),
                    Some(false) => return Ok(Evaluated::nil()),
                    None => {
                        return eval_err()
                            .at(locate(&cond))
                            .with_message("could not evaluate if stmt condition".to_string())
                            .to_result()
                    }
                }
            }
            Statement::WhileLoop(cond, program) => loop {
                let cond_evaluated = self.visit(cond)?;
                let can_continue_loop = Option::<bool>::from(cond_evaluated);
                match can_continue_loop {
                    Some(true) => {
                        let evaluated_program = self.scoped(|v| v.visit(program))?;
                        if evaluated_program.returned.is_some() {
                            return Ok(evaluated_program);
                        }
                    }
                    Some(false) => return Ok(Evaluated { returned: None }),
                    None => {
                        return eval_err()
                            .without_pos()
                            .with_message("could not evaluate while stmt condition".to_string())
                            .to_result()
                    }
                }
            },
            Statement::Let(lval, rval) => {
                let pos = locate(&rval.expr);
                let right_evaluated = self.visit(&rval.expr)?;
                self.state
                    .bind(lval.identifier.clone(), right_evaluated, pos)?;
            }
            Statement::Fun(pos, function_definition) => {
                self.state.bind(
                    function_definition.name.clone(),
                    LoxObj::from(function_definition.clone()),
                    *pos,
                )?;
            }
            Statement::Class(defn) => {
                let ClassDefinition {
                    name,
                    fields,
                    methods,
                } = defn;
                let Token { val, pos } = name;

                let identifier = match val {
                    TokenValue::Id(id) => id.clone(),
                    _ => panic!("Class name is not an identifier. This should have been caught by the parser but wasn't.")
                };

                self.state.bind(identifier, LoxObj::from(defn.clone()), *pos)?;

                for field in fields {
                    self.state.bind_to_obj(identifier, field);
                }
                for method in methods {
                    self.state.bind_to_obj(identifier, method);
                }
                
            }
            Statement::Return(expr) => {
                let evaluated_expr = self.visit(expr)?;
                return Ok(Evaluated {
                    returned: Some(evaluated_expr),
                });
            }
        }
        Ok(Evaluated { returned: None })
    }
}

impl Visitor<Expr, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, expr: &Expr) -> LoxResult<LoxObj> {
        match expr {
            Expr::Eqlty(eqlty) => self.visit(eqlty),
            _ => panic!("call expr is not supported anymore"),
        }
    }
}

impl Visitor<Eqlty, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, eqlty: &Eqlty) -> LoxResult<LoxObj> {
        let first_evaluated: LoxResult<LoxObj> = self.visit(&eqlty.first);
        eqlty
            .rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Comp, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, comp: &Comp) -> LoxResult<LoxObj> {
        let first_evaluated = self.visit(&comp.first);
        comp.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Term, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, term: &Term) -> LoxResult<LoxObj> {
        let first_evaluated = self.visit(&term.first);
        term.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Factor, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, fac: &Factor) -> LoxResult<LoxObj> {
        let first_evaluated = self.visit(&fac.first);
        fac.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Unary, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, unary: &Unary) -> LoxResult<LoxObj> {
        match unary {
            Unary::Final(None, token) => self.as_lox_obj(token),
            Unary::Final(Some(op), token) => {
                let lox_obj = self.as_lox_obj(token)?;
                return lox_obj.apply(|raw| unary_op(op, raw));
            }
            Unary::Recursive(None, expr) => self.visit(expr.as_ref()),
            Unary::Recursive(Some(op), expr) => {
                let evaluated_expression = self.visit(expr.as_ref())?;
                return evaluated_expression.apply(|raw| unary_op(op, raw));
            }
            Unary::Call(operator, fn_name, args) => {
                let func = self.as_lox_obj(fn_name)?;
                let pos = fn_name.pos;
                let args_evaluated: LoxResult<Vec<LoxObj>> = args
                    .into_iter()
                    .map(|arg_expr| self.visit(arg_expr.as_ref()))
                    .collect();

                let fn_output = self.call(func, args_evaluated?, pos)?;
                return match operator {
                    Some(op) => fn_output.apply(|raw| unary_op(op, raw)),
                    None => Ok(fn_output),
                };
            }
        }
    }
}

fn eval_fold(acc: LoxResult<LoxObj>, next: (&Token, LoxResult<LoxObj>)) -> LoxResult<LoxObj> {
    let acc: LoxObj = acc?;
    let (op, val) = next;
    let val: LoxObj = val?;

    if let Some(acc_val) = acc.to_value() {
        if let Some(val_val) = val.to_value() {
            let result = binary_operations::handle(op, acc_val, val_val, op.pos)?;
            return Ok(LoxObj::from(result));
        }
    }

    panic!("TODO: not panic")
}

impl Executor {
    /// Evaluates token to `LoxObj` if token is an identifier or value
    fn as_lox_obj(&self, token: &Token) -> LoxResult<LoxObj> {
        match &token.val {
            TokenValue::Id(id) => self.state.get(&id, token.pos),
            TokenValue::Val(lox_val) => Ok(LoxObj::from(lox_val.clone())),
            _ => Err(eval_err()
                .at(token.pos)
                .is_not(token, "a lox object")
                .build()),
        }
    }

    fn call(&mut self, func: LoxObj, args: Vec<LoxObj>, pos: Position) -> LoxResult<LoxObj> {
        let function = func.transform(|raw| match raw {
            RawLoxObject::Fun(function_def) => Ok(function_def),
            _ => eval_err()
                .at(pos)
                .is_not(raw.to_string(), "callable")
                .to_result(),
        })?;

        self.state.push_new_scope();

        for (arg_name, arg_evaluated) in zip(function.args, args) {
            self.state.bind(arg_name, arg_evaluated, pos)?;
        }
        let program_result = self.visit(&function.body)?;

        self.state.pop_last_scope();

        match program_result.returned {
            Some(obj) => Ok(obj),
            None => Ok(LoxObj::from(LoxValue::from(0))),
        }
    }
}
