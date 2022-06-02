pub trait Visitor<A, B> {
    fn visit(&mut self, a: &A) -> B;
}

pub trait Acceptor {
    fn accept<B>(&self, visitor: Box<dyn Visitor<Self, B>>) -> B;
}
