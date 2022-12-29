use crate::board::action::Action;
use crate::board::board::Board;
use crate::evaluation::manhattan::ManhattanEvaluation;
use crate::evaluation::naive_bonus::NaiveBonusEvaluation;
use crate::solver::alpha_beta_solver::AlphaBetaSolver;
use crate::solver::brute_force_search_solver::BruteForceSearchSolver;
use crate::solver::solver::{MaybeSolver, Solver};

pub struct MixSolver;

impl Default for MixSolver {
    fn default() -> Self {
        MixSolver
    }
}

impl Solver for MixSolver {
    fn solve(&self, board: &Board, next_player: usize) -> Action {
        if board.is_separable() {
            let bf_solver = BruteForceSearchSolver::new(4);
            let result = bf_solver.solve(board, next_player);
            if let Some(x) = result {
                return x;
            }

            // let ab_solver1 = AlphaBetaSolver::new(Box::new(NaiveBonusEvaluation::default()));
            // return ab_solver1.solve(board, next_player);
        }

        let ab_solver = AlphaBetaSolver::new(Box::new(ManhattanEvaluation), 5);
        ab_solver.solve(board, next_player)
    }
}
