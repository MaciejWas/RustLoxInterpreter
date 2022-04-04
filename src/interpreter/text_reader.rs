use std::cell::Cell;

pub struct TextReader {
    source: String,
    pos: Cell<usize>,
}

impl TextReader {
    pub fn new(source: String) -> Self {
        TextReader {source: source, pos: Cell::new(0)}
    }

    pub fn back(&self) -> Option<()> {
        let new_pos = self.pos.get() - 1;
        match self.source.chars().nth(new_pos) {
            Some(c) => {
                self.pos.set(new_pos);
                Some(())
            }
            None => None
        }
    }

    pub fn advance(&self) -> Option<char> {
        let new_pos = self.pos.get() + 1;
        match self.source.chars().nth(new_pos) {
            Some(c) => {
                self.pos.set(new_pos);
                Some(c)
            }
            None => None
        }
    }

    pub fn advance_until_newline(&self) {
        loop {
            match self.advance() {
                Some(c) => if c == '\n' {break}
                None    => {break},
            }
        }
    }

    pub fn get_pos(&self) -> usize {
        self.pos.get()
    }
}