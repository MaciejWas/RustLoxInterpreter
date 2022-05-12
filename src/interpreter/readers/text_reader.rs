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

    fn advance(&self) -> Option<()> {
        if self.peek().is_some() {
            self.pos.set(self.pos.get() + 1);
            return Some(())
        }
        None
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.pos.get())
    }

    fn curr(&self) -> &char {
        self.source.get(self.pos.get())
            .unwrap_or_else(|| panic!("Current pos: {:?} is out of bounds of source.", self.pos.get()))
    }
}