use clap::{Parser, ArgAction};

use rush::board::Board;
use rush::solve::bfs;


#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    puzzle: String,

    #[clap(short, long, action = ArgAction::SetTrue, default_value_t = false)]
    verbose: bool,
}


fn main() {
    let args = Args::parse();

    let mut board = Board::from_string(args.puzzle).unwrap();
    if args.verbose {
        println!("Puzzle:");
        board.display();
    }

    let (solved, solution) = bfs(&board);
    if !solved {
        println!("Solution not found");
        return;
    }

    let mut solution_str = format!("Solution ({} moves):", solution.len());
    for (piece_id, d) in solution.iter() {
        let sign = if *d >= 0 { "+" } else { "-" };
        solution_str.push_str(&format!(" {}{}{}", board.get_piece(*piece_id).val, sign, d.abs()));
    }
    println!("{}", solution_str);

    if args.verbose {
        let mut step = 0;
        for (piece_id, d) in solution.iter() {
            step += 1;
            let sign = if *d >= 0 { "+" } else { "-" };
            println!("Step {}: {}{}{}", step, board.get_piece(*piece_id).val, sign, d.abs());
            board.move_piece(*piece_id, *d);
            board.display();
        }
    }
}
