use chess_ai::board::board::Board;
use chess_ai::solver::solver::{MaybeSolver, Solver};
use chess_ai::constants::RED;
use chess_ai::board::action::Action;
use std::io::stdin;
use chess_ai::solver::brute_force_search_solver::BruteForceSearchSolver;
use chess_ai::solver::mix_solver::MixSolver;

fn main() {
    let mut board = Board::new(9);
    println!("{}", board);
    let solver = MixSolver::default();
    
    while board.is_game_over().is_none() {
        let action = solver.solve(&board, RED);
        board.perform_action(action);
        println!("AI:");
        println!("{}", board);
    
        let mut input = String::new();
        stdin().read_line(&mut input);
        let numbers = input.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    
        let action_player = Action::from_usize(numbers[0], numbers[1], numbers[2], numbers[3]);
        board.perform_action(action_player);
        println!("player:");
        println!("{}", board);
    }

    // let mut board = Board::new(9);
    // board.set_row_by_string(0, "0000rrrr0");
    // board.set_row_by_string(1, "0000rr0rr");
    // board.set_row_by_string(2, "0000000rr");
    // board.set_row_by_string(3, "00000rrrr");
    // board.set_row_by_string(4, "00000r000");
    // board.set_row_by_string(5, "000000r00");
    // board.set_row_by_string(6, "000000000");
    // board.set_row_by_string(7, "000000000");
    // board.set_row_by_string(8, "000000000");
    // let solver = MixSolver::default();
    // println!("{:?}", solver.solve(&board, RED));
}