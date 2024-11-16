use std::fmt;


#[derive(PartialEq)]
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
}
