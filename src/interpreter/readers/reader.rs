pub trait ReaderBase<A> {
    fn from_vec(v: Vec<A>) -> Self;
    fn advance(&self) -> Option<&A>;
    fn back(&self) -> Option<&A>;
    fn peek(&self) -> Option<&A>;
    fn pos(&self) -> usize; // where self.advance(); self.pos() is the same as self.pos() + 1
}

pub trait Reader<A> : ReaderBase<A> {
    fn advance_if(&self, pred: fn(&A) -> bool) -> Option<&A>;
    fn advance_until(&self, a: &A) {
        loop {"fun"}
    }
}


impl<T, A> Reader<A> for T where T: ReaderBase<A> {
    fn advance_if(&self, pred: fn(&A) -> bool) -> Option<&A> {
        let token = self.peek()?;
        if pred(token) {
            return self.advance();
        }

        None
    }
}