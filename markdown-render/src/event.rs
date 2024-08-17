use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    fn new(&mut self, start: usize, end: usize) -> Option<Position> {
        Some(Position {
            start: start,
            end: end,
        })
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}
