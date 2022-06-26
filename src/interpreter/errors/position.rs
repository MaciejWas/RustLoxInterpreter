#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Position {
    pub line: usize,
    pub line_pos: usize,
}

impl Into<Position> for (usize, usize) {
    fn into(self) -> Position {
        Position {
            line: self.0,
            line_pos: self.1,
        }
    }
}
