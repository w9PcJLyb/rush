use std::fmt;


#[derive(PartialEq, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}


impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::Vertical => write!(f, "Vertical"),
            Orientation::Horizontal => write!(f, "Horizontal"),
        }
    }
}


#[derive(Clone)]
pub struct Piece {
    pub val: char,
    pub p: usize,
    pub row: usize,
    pub size: usize,
    pub orientation: Orientation
}


impl Piece {
    pub fn is_horizontal(&self) -> bool {
        self.orientation == Orientation::Horizontal
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        if self.is_horizontal() {
            y == self.row && x >= self.p && x < self.p + self.size
        } else {
            x == self.row && y >= self.p && y < self.p + self.size
        }
    }
}
