use crate::interpreter::parser::Token;

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

pub type ExprRule = Single<EqltyRule>;
pub type EqltyRule = Many<CompRule>;
pub type CompRule = Many<TermRule>;
pub type TermRule = Many<FactorRule>;
pub type FactorRule = Many<UnaryRule>;
pub type UnaryRule = Unary;
