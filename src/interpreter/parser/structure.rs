use super::visitor::*;
use crate::interpreter::parser::Token;

#[derive(Debug, Clone)]
pub enum Or2<A, B> {
    Opt1(A),
    Opt2(B),
}

pub type Program = Vec<Statement>;
pub type SubRules<A> = Vec<(Token, A)>;

#[derive(Debug, Clone)]
pub enum Statement {
    ExprStmt(Expr),
    PrintStmt(Expr),
    IfStmt(Expr, Program),
    LetStmt(LVal, RVal),
    WhileLoop(Expr, Program),
    DefStmt(FunctionDefinition),
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub args: Vec<Token>,
    pub body: Program,
}

#[derive(Debug, Clone)]
pub struct LVal {
    pub identifier: String,
}

#[derive(Debug, Clone)]
pub struct RVal {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Eqlty(Eqlty),
}

#[derive(Debug, Clone)]
pub struct Eqlty {
    pub first: Comp,
    pub rest: SubRules<Comp>,
}

#[derive(Debug, Clone)]
pub struct Comp {
    pub first: Term,
    pub rest: SubRules<Term>,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub first: Factor,
    pub rest: SubRules<Factor>,
}

#[derive(Debug, Clone)]
pub struct Factor {
    pub first: Unary,
    pub rest: SubRules<Unary>,
}

#[derive(Debug, Clone)]
pub enum Unary {
    Final(Option<Token>, Token),
    Recursive(Option<Token>, Box<Expr>),
}

// #################################

pub trait FromSubRules<A> {
    fn from_sub(first: A, rest: SubRules<A>) -> Self;
}

impl FromSubRules<Comp> for Eqlty {
    fn from_sub(first: Comp, rest: SubRules<Comp>) -> Self {
        Eqlty {
            first: first,
            rest: rest,
        }
    }
}

impl FromSubRules<Term> for Comp {
    fn from_sub(first: Term, rest: SubRules<Term>) -> Self {
        Comp {
            first: first,
            rest: rest,
        }
    }
}

impl FromSubRules<Factor> for Term {
    fn from_sub(first: Factor, rest: SubRules<Factor>) -> Self {
        Term {
            first: first,
            rest: rest,
        }
    }
}

impl FromSubRules<Unary> for Factor {
    fn from_sub(first: Unary, rest: SubRules<Unary>) -> Self {
        Factor {
            first: first,
            rest: rest,
        }
    }
}

// #################################

impl Acceptor for Statement {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Expr {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Eqlty {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Comp {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Factor {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Term {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Unary {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

// #################################

pub trait NamedType {
    fn type_name(&self) -> String;
}

impl NamedType for Statement {
    fn type_name(&self) -> String {
        match self {
            Self::PrintStmt(_) => "Statement",
            Self::ExprStmt(_) => "Expression",
            Self::IfStmt(_, _) => "IfStatement",
            Self::LetStmt(_, _) => "LetStmt",
            Self::WhileLoop(_, _) => "WhileLoop",
            Self::DefStmt(_) => "DefStmt",
        }
        .to_string()
    }
}
impl NamedType for Expr {
    fn type_name(&self) -> String {
        "Expression".to_string()
    }
}
impl NamedType for Eqlty {
    fn type_name(&self) -> String {
        "Equality".to_string()
    }
}
impl NamedType for Comp {
    fn type_name(&self) -> String {
        "Comparison".to_string()
    }
}
impl NamedType for Term {
    fn type_name(&self) -> String {
        "Term".to_string()
    }
}
impl NamedType for Factor {
    fn type_name(&self) -> String {
        "Factor".to_string()
    }
}
impl NamedType for Unary {
    fn type_name(&self) -> String {
        "Unary".to_string()
    }
}
