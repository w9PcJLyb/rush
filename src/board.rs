use std::collections::HashSet;

use crate::piece::{Piece, Orientation};


#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    pieces: Vec<Piece>,
    walls: HashSet<(usize, usize)>,
    primary_piece_id: usize,
}


impl Board {
    pub fn new(width: usize, height: usize, primary_piece: Piece) -> Self {
        Board {
            width,
            height,
            pieces: vec![primary_piece],
            walls: HashSet::new(),
            primary_piece_id: 0,
        }
    }

    pub fn from_string(puzzle: String) -> Result<Board, String> {
        let board_size_f = (puzzle.len() as f64).sqrt(); 
        let board_size = board_size_f.floor() as usize;
        if board_size * board_size != puzzle.len() {
            return Err(format!("Invalid puzzle size"));
        }

        let data: Vec<char> = puzzle.chars().collect();

        let mut explored: HashSet<usize> = HashSet::new();
        let mut used_values: HashSet<char> = HashSet::new(); 
        let mut pieces: Vec<Piece> = Vec::new();
        let mut walls: HashSet<(usize, usize)> = HashSet::new();
        let mut primary_piece_id: usize = 0;

        for y in 0..board_size {
            for x in 0..board_size {
                let i = y * board_size + x;

                if explored.contains(&i) {
                    continue;
                }
                explored.insert(i);
                
                let val = data[i];

                if val == 'o' || val == '.' {
                    // empty cell
                    continue;
                }
                
                if val == 'x' {
                    // wall
                    walls.insert((x, y));
                    continue;
                }

                if !val.is_ascii_uppercase() {
                    return Err(format!("Unsupported value: `{}`, Allowed values: '.' or 'o' for empty cells, 'x' for walls, and A-Z for pieces", val));
                }

                // piece
                if used_values.contains(&val) {
                    return Err(
                        format!("There is more than one piece with the value '{}'", val)
                    );
                }

                used_values.insert(val);
                
                let mut size = 1;
                while x + size < board_size && data[y * board_size + x + size] == val {
                    explored.insert(y * board_size + x + size);
                    size += 1;
                }

                if size > 1 {
                    let piece = Piece {
                        val,
                        p: x,
                        row: y,
                        size,
                        orientation: Orientation::Horizontal
                    };
                    pieces.push(piece);
                    if val == 'A' {
                        primary_piece_id = pieces.len() - 1;
                    }
                    continue;
                }
          
                while y + size < board_size && data[(y + size) * board_size + x] == val {
                    explored.insert((y + size) * board_size + x);
                    size += 1;
                }

                if size > 1 {
                    let piece = Piece {
                        val,
                        p: y,
                        row: x,
                        size,
                        orientation: Orientation::Vertical
                    };
                    pieces.push(piece);
                    if val == 'A' {
                        return Err(format!("The primary piece must be horizontal"));
                    }
                    continue;
                }

                return Err(
                    format!(
                        "Piece with the value '{}' has a size of one. The size must be greater than one",
                        val,
                    )
                );
            }
        }

        if !used_values.contains(&'A') {
            return Err(
                format!("There is no primary piece. There must be a piece with the value 'A'")
            );
        }

        Ok(
            Board {
                width: board_size,
                height: board_size,
                pieces,
                walls,
                primary_piece_id,
            }
        )
    }

    pub fn display(&self) {
        let mut map: Vec<Vec<char>> = vec![vec![' '; self.height]; self.width];

        for piece in &self.pieces {
            if piece.is_horizontal() {
                for x in piece.p..(piece.p + piece.size) {
                    map[piece.row][x] = piece.val;
                }
            }
            else {
                for y in piece.p..(piece.p + piece.size) {
                    map[y][piece.row] = piece.val;
                }
            }
        }

        for wall in &self.walls {
            map[wall.1][wall.0] = '#';
        }

        let mut line = String::from("");
        for _ in 0..self.width {
            line.push_str(" -");
        }

        println!("+{} +", line);
        for y in 0..self.height {
            let mut row = String::from("");
            for x in 0..self.width {
                row.push_str(&format!(" {}", map[y][x]));
            }
            println!("|{} |", row);
        }
        println!("+{} +", line);
    }

    pub fn get_piece(&self, piece_id: usize) -> &Piece {
        &self.pieces[piece_id]
    }   

    pub fn get_primary_piece(&self) -> &Piece {
        &self.pieces[self.primary_piece_id]
    }

    pub fn solved(&self) -> bool {
        // returns true if the primary piece is in the rightmost position
        let primary_piece = self.get_primary_piece();
        primary_piece.p == self.width - primary_piece.size
    }

    pub fn movements(&self) -> Vec<(usize, i32)> {
        let mut map: Vec<Vec<bool>> = vec![vec![true; self.height]; self.width];
        for piece in &self.pieces {
            if piece.is_horizontal() {
                for x in piece.p..(piece.p + piece.size) {
                    map[piece.row][x] = false;
                }
            }
            else {
                for y in piece.p..(piece.p + piece.size) {
                    map[y][piece.row] = false;
                }
            }
        }

        for wall in &self.walls {
            map[wall.1][wall.0] = false;
        }

        let mut results: Vec<(usize, i32)> = Vec::new();
 
        // horizontal pieces
        let mut d : usize;
        for (i, piece) in self.pieces.iter().enumerate() {
            if piece.is_horizontal() {
                d = 1;
                while piece.p >= d && map[piece.row][piece.p - d] {
                    results.push((i, -(d as i32)));
                    d += 1;
                }

                d = 1;
                let end = piece.p + piece.size - 1;
                while end + d < self.width && map[piece.row][end + d] {
                    results.push((i, d as i32));
                    d += 1;
                }
            }
            else {
                d = 1;
                while piece.p >= d && map[piece.p - d][piece.row] {
                    results.push((i, -(d as i32)));
                    d += 1;
                }

                d = 1;
                let end = piece.p + piece.size - 1;
                while end + d < self.height && map[end + d][piece.row] {
                    results.push((i, d as i32));
                    d += 1;
                }
            }
        }

        results
    }

    pub fn move_piece(&mut self, piece_id: usize, d: i32) {
        let piece = &mut self.pieces[piece_id];
        if d > 0 {
            piece.p += d as usize;
        }
        else {
            piece.p -= -d as usize;
        }
    }

    pub fn positions(&self) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        for piece in &self.pieces {
            v.push(piece.p);
        }
        v
    }

    pub fn update_positions(&mut self, positions: &Vec<usize>) {
        for (piece_id, new_position) in positions.iter().enumerate() {
            self.pieces[piece_id].p = *new_position;
        }
    }

    pub fn is_free(&self, x: usize, y: usize) -> bool {
        for piece in self.pieces.iter() {
            if piece.contains(x, y) {
                return false;
            }
        }
        for (x_, y_) in self.walls.iter() {
            if x == *x_ && y == *y_ {
                return false;
            }
        }
        return true;
    }

    pub fn add_wall(&mut self, x: usize, y: usize) {
        if !self.is_free(x, y) {
            panic!("The cell ({}, {}) is already occupied", x, y)
        }
        self.walls.insert((x, y));
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        if piece.is_horizontal() {
            let y = piece.row;
            for x in piece.p..(piece.p + piece.size) {
                if !self.is_free(x, y) {
                    panic!("The cell ({}, {}) is already occupied", x, y)
                }
            }
        } else {
            let x = piece.row;
            for y in piece.p..(piece.p + piece.size) {
                if !self.is_free(x, y) {
                    panic!("The cell ({}, {}) is already occupied", x, y)
                }
            }
        }
        self.pieces.push(piece.clone());
    }
}
