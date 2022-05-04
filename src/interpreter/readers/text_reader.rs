use std::cell::Cell;

pub struct TextReader {
    pub source: String,
    pos: Cell<usize>,
}

impl TextReader {
    pub fn new(source: String) -> Self {
        TextReader {source: source, pos: Cell::new(0)}
    }

    pub fn reset(self) -> Self {
        TextReader {source: self.source, pos: Cell::new(0)}
    }

    pub fn back(&self) -> Option<()> {
        let prev_pos = self.pos.get() as i32 - 1;
        if prev_pos < 0 {
            None
        } else {
            self.pos.set(prev_pos as usize);
            Some(())
        }
        }
    
    pub fn advance(&self) -> Option<char> {
        let curr_pos = self.pos.get();
        match self.source.chars().nth(curr_pos) {
            Some(c) => {
                self.pos.set(curr_pos + 1);
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

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;
    use super::TextReader;

    #[test]
    fn step_forward() {
        let text = "sadfasdfdfs";
        let r = TextReader::new(text.to_string());
        assert_eq!(r.get_pos(), 0);
        assert_eq!(r.advance(), Some('s'));
        assert_eq!(r.get_pos(), 1);
    }
}
