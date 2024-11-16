use rush::board::Board;
use rush::piece::{Piece, Orientation};
use rush::solve::bfs;


#[test]
fn test_with_one_piece() {
    let primary_piece = Piece {
        val: 'A', p: 0, row: 1, size: 2, orientation: Orientation::Horizontal
    }; 
    let board = Board::new(4, 4, primary_piece);

    let (solved, solution) = bfs(&board);
    assert_eq!(solved, true);
    assert_eq!(solution, vec![(0, 2)]);
}

#[test]
fn test_with_two_pieces() {
    let primary_piece = Piece {
        val: 'A', p: 0, row: 0, size: 2, orientation: Orientation::Horizontal
    }; 
    let mut board = Board::new(4, 4, primary_piece);

    let piece: Piece = Piece {
        val: 'B', p: 0, row: 3, size: 3, orientation: Orientation::Vertical
    }; 
    board.add_piece(&piece);

    let (solved, solution) = bfs(&board);
    assert_eq!(solved, true);
    assert_eq!(solution, vec![(1, 1), (0, 2)]);
}

#[test]
fn test_solved() {
    let primary_piece = Piece {
        val: 'A', p: 2, row: 1, size: 2, orientation: Orientation::Horizontal
    }; 
    let board = Board::new(4, 4, primary_piece);

    let (solved, solution) = bfs(&board);
    assert_eq!(solved, true);
    assert_eq!(solution, vec![]);
}

#[test]
fn test_unsolvable_puzzle() {
    let primary_piece = Piece {
        val: 'A', p: 0, row: 1, size: 2, orientation: Orientation::Horizontal
    }; 
    let mut board = Board::new(4, 4, primary_piece);
    board.add_wall(3, 1);

    let (solved, solution) = bfs(&board);
    assert_eq!(solved, false);
    assert_eq!(solution, vec![]);
}

#[test]
fn test_unsolvable_puzzle_2() {
    let primary_piece = Piece {
        val: 'A', p: 0, row: 1, size: 2, orientation: Orientation::Horizontal
    }; 
    let mut board = Board::new(4, 4, primary_piece);

    let piece: Piece = Piece {
        val: 'B', p: 2, row: 1, size: 2, orientation: Orientation::Horizontal
    }; 
    board.add_piece(&piece);

    let (solved, solution) = bfs(&board);
    assert_eq!(solved, false);
    assert_eq!(solution, vec![]);
}