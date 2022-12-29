use crate::board::action::Action;
use crate::board::board::Board;
use crate::solver::alpha_beta_solver::AlphaBetaSolver;
use crate::solver::mcts_solver::MCTSSolver;
use crate::solver::mix_solver::MixSolver;
use crate::solver::solver::Solver;

#[tauri::command]
pub async fn chess1_solve(board: Board, next_player: usize) -> Action {
    // let solver = MCTSSolver::new(Default::default());
    // let solver = AlphaBetaSolver::default();
    let solver = MixSolver::default();
    let action = solver.solve(&board, next_player);
    action
}
