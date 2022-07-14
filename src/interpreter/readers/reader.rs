/// A base class which provides some utility methods over an iterable object.
pub trait ReaderBase<A> {
    fn from_vec(v: Vec<A>) -> Self;

    /// Will return consecutive elements with each call. Will return None when every element has been returned. Satisfies:
    ///  * `advance() == peek()`,
    ///  * `advance()?; pos() == pos() + 1`,
    ///  * `advance() == None` if and only if `pos() == v.len()` where v is the vector passed to the constructor `from_vec`.
    fn advance(&self) -> Option<&A>;

    /// Returns the element which would be returned in the next call by `advance`
    fn peek_n(&self, n: usize) -> Option<&A>;

    /// Current position in the iterable. Ranges from `[0, v.len())`. Satisfies:
    /// * `self.advance(); self.pos()` == `self.pos() + 1`
    fn pos(&self) -> usize;

    /// Returns the element returned by last call to advance()
    fn previous(&self) -> Option<&A>;
}

pub trait Reader<A>: ReaderBase<A> {
    fn from_vec(v: Vec<A>) -> Self;
    fn advance(&self) -> Option<&A>;
    fn peek(&self) -> Option<&A>;
    fn peek_n(&self, n: usize) -> Option<&A>;
    fn peek_or<E>(&self, err: E) -> Result<&A, E>;
    fn pos(&self) -> usize;
    fn advance_if<F>(&self, pred: F) -> Option<&A>
    where
        F: Fn(&A) -> bool;
    fn advance_or<E>(&self, err: E) -> Result<&A, E>;
    fn advance_until<F>(&self, pred: F)
    where
        F: Fn(&A) -> bool;
    fn previous(&self) -> Option<&A>;
}

impl<T, A> Reader<A> for T
where
    T: ReaderBase<A>,
{
    fn from_vec(v: Vec<A>) -> Self {
        T::from_vec(v)
    }

    fn advance(&self) -> Option<&A> {
        T::advance(self)
    }

    fn peek_n(&self, n: usize) -> Option<&A> {
        T::peek_n(self, n)
    }

    fn peek(&self) -> Option<&A> {
        T::peek_n(self, 0)
    }

    fn pos(&self) -> usize {
        T::pos(self)
    }

    fn previous(&self) -> Option<&A> {
        T::previous(self)
    }

    fn advance_if<F>(&self, pred: F) -> Option<&A>
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
            match self.advance_if(|a: &A| !pred(a)) {
                None => break,
                Some(_) => continue
            };
        }
    }

    fn advance_or<E>(&self, err: E) -> Result<&A, E> {
        self.advance().ok_or(err)
    }

    fn peek_or<E>(&self, err: E) -> Result<&A, E> {
        self.peek().ok_or(err)
    }
}
