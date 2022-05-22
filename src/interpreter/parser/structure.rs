use crate::interpreter::parser::Token;

#[derive(Debug)]
pub enum Or2<A, B> {
    Opt1(A),
    Opt2(B),
}

#[derive(Debug)]
pub struct Single<A> {
    pub value: A,
}

#[derive(Debug)]
pub struct Many<A> {
    pub first: A,
    pub rest: Vec<(Token, A)>,
}

#[derive(Debug)]
pub struct Unary {
    pub op: Option<Token>,
    pub right: Token,
}

pub type Program = Vec<Statement>;
pub type Statement = Or2<ExprRule, PrintStmt>;
pub type PrintStmt = Single<ExprRule>;
pub type ExprRule = Single<EqltyRule>;
pub type EqltyRule = Many<CompRule>;
pub type CompRule = Many<TermRule>;
pub type TermRule = Many<FactorRule>;
pub type FactorRule = Many<UnaryRule>;
pub type UnaryRule = Unary;


pub trait NamedType {
    fn type_name() -> String;
}

impl NamedType for Statement { fn type_name() -> String { "Statement".to_string() } }
impl NamedType for PrintStmt { fn type_name() -> String { "PrintStmt".to_string() } }
impl NamedType for ExprRule { fn type_name() -> String { "Expression".to_string() } }
impl NamedType for EqltyRule { fn type_name() -> String { "Equality".to_string() } }
impl NamedType for CompRule { fn type_name() -> String { "Comparison".to_string() } }
impl NamedType for TermRule { fn type_name() -> String { "Term".to_string() } }
impl NamedType for FactorRule { fn type_name() -> String { "Factor".to_string() } }
impl NamedType for UnaryRule { fn type_name() -> String { "Unary".to_string() } }