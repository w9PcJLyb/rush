use std::collections::HashMap;
use std::collections::VecDeque;

use crate::board::Board;


pub fn bfs(board: &mut Board) -> (bool, Vec<(usize, i32)>) {
    let mut map: HashMap<Vec<usize>, (usize, i32)> = HashMap::new();
    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();

    let init_positions = board.positions();
    queue.push_back(init_positions.clone());
    map.insert(init_positions.clone(), (0, 0));

    while let Some(positions) = queue.pop_front() {
        board.update_positions(&positions);

        if board.solved() {
            board.update_positions(&init_positions);
            return (true, restore_solution(positions, &map));
        }

        for (piece_id, d) in board.movements() {
            let mut next_positions = positions.clone();
            if d > 0 {
                next_positions[piece_id] += d as usize;
            }
            else {
                next_positions[piece_id] -= -d as usize;
            }
            if !map.contains_key(&next_positions) {
                map.insert(next_positions.clone(), (piece_id, d));
                queue.push_back(next_positions);
            }
        }
    }

    return (false, Vec::new());
}

fn restore_solution(mut positions: Vec<usize>, map: &HashMap<Vec<usize>, (usize, i32)>) -> Vec<(usize, i32)> {
    let mut solution: Vec<(usize, i32)> = Vec::new();
    loop {
        let (piece_id, d) = map[&positions];
        if d > 0 {
            positions[piece_id] -= d as usize;
        }
        else if d < 0 {
            positions[piece_id] += -d as usize;
        } else {
            break;
        }
        solution.push((piece_id, d));
    }

    solution.reverse();

    return solution;
}
