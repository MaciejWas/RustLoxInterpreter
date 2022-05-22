use super::reader::{Reader, ReaderBase};
use std::cell::Cell;

pub struct TextReader {
    source: Vec<char>,
    pos: Cell<usize>,
}

impl ReaderBase<char> for TextReader {
    fn from_vec(v: Vec<char>) -> Self {
        TextReader {
            source: v,
            pos: Cell::new(0),
        }
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    fn advance(&self) -> Option<&char> {
        let pos = self.pos.get();
        let output = self.source.get(pos);
        if output.is_some() {
            self.pos.set(pos + 1);
        }
        output
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.pos.get())
    }
}
