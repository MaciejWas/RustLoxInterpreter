pub trait ReaderBase<A> {
    fn from_vec(v: Vec<A>) -> Self;
    fn advance(&self) -> Option<()>;
    fn peek(&self) -> Option<&A>;
    fn pos(&self) -> usize;
    fn curr(&self) -> &A;
}

pub trait Reader<A>: ReaderBase<A> {
    fn advance_if<F>(&self, pred: F) -> Option<()>
    where
        F: Fn(&A) -> bool;
    fn advance_until<F>(&self, pred: F)
    where
        F: Fn(&A) -> bool;
}

impl<T, A> Reader<A> for T
where
    T: ReaderBase<A>,
{
    fn advance_if<F>(&self, pred: F) -> Option<()>
    where
        F: Fn(&A) -> bool,
    {
        let token = self.peek()?;
        if pred(token) {
            return self.advance();
        }

        None
    }

    fn advance_until<F>(&self, pred: F)
    where
        F: Fn(&A) -> bool,
    {
        loop {
            self.advance_if(|a: &A| !pred(a));
        }
    }
}
