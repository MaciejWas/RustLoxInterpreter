use crate::interpreter::parser::Token;

pub enum Node<A> {
    Bi(BiNode<A>),
    NestedBi(NestedBiNode<A>),
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

type ExprRule    = EqltyRule;
type EqltyRule   = Node<TermRule>;
type TermRule    = Node<FactorRule>;
type FactorRule  = Node<UnaryRule>;
type UnaryRule   = UnaryNode<PrimaryRule>;
type PrimaryRule = LeafNode;

