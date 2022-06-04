use crate::interpreter::errors::position::Position;
use super::reader::ReaderBase;
use std::cell::Cell;

pub struct TextReader {
    source: Vec<char>,
    pos: Cell<usize>,
    current_line: Cell<usize>,
    line_pos: Cell<usize>,
}

impl TextReader {
    pub fn curr_pos(&self) -> Position {
        Position { line: self.current_line.get(), line_pos: self.line_pos.get() }
    }
}

impl ReaderBase<char> for TextReader {
    fn from_vec(v: Vec<char>) -> Self {
        TextReader {
            source: v,
            pos: Cell::new(0),
            current_line: Cell::new(0),
            line_pos: Cell::new(0),
        }
    }

    fn pos(&self) -> usize {
        self.pos.get()
    }

    fn advance(&self) -> Option<&char> {
        let pos = self.pos.get();
        let output = self.source.get(pos);
        if let Some(c) = output {
            self.pos.set(pos + 1);
            self.line_pos.set(self.line_pos.get() + 1);

            if *c == '\n' {
                self.current_line.set(self.current_line.get() + 1);
                self.line_pos.set(0);
            }
        }

        output
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.pos.get())
    }

    fn previous(&self) -> Option<&char> {
        if self.pos.get() >= 1 {
            return self.source.get(self.pos.get() - 1);
        }
        None
    }
}
