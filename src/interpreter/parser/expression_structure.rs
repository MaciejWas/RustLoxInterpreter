use crate::interpreter::parser::Token;

pub enum Node<A> {
    Single(Single<A>);
    Bi(BiNode<A>),
    NestedBi(NestedBiNode<A>),
}

pub struct Single<A> {
    value: Box<A>
}

pub struct NestedBiNode<A> {
    left: Box<A>,
    op: Token,
    right: Box<Node<A>>
}

pub struct BiNode<A> {
    left: Box<A>,
    op: Token,
    right: Box<A>
}

pub struct UnaryNode<A> {
   op: Token,
   right: Box<A>
}

pub struct LeafNode {
    value: Token
}

type ExprRule    = Single<EqltyRule>;
type EqltyRule   = Node<TermRule>;
type TermRule    = Node<FactorRule>;
type FactorRule  = Node<UnaryRule>;
type UnaryRule   = UnaryNode<PrimaryRule>;
type PrimaryRule = LeafNode;


pub struct NodeBuilder<A> {
    left: Option<A>,
    op: Option<Token>,
    right: Option<A>,
    right_nested: Option<Node<A>>,

    first: A,
    others: Vec<(Token, A)>
}

impl <A> NodeBuilder<A> {
    pub fn new() -> Self {
        NodeBuilder {
            left: None,
            op: None,
            right: None,
            right_nested: None,

            first: first_val,
            others: Vec::new()
    }

    pub fn left()

    pub fn nest(next_val: A, next_op: Token) -> Self {
        
    }
}
