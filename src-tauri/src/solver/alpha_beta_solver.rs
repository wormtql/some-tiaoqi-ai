use crate::board::action::Action;
use crate::board::board::Board;
use crate::evaluation::evaluation::Evaluation;
use crate::evaluation::manhattan::ManhattanEvaluation;
use crate::solver::solver::Solver;

pub struct AlphaBetaSolver {
    pub evaluator: Box<dyn Evaluation>,
    pub max_depth: usize,
}

impl Default for AlphaBetaSolver {
    fn default() -> Self {
        AlphaBetaSolver {
            evaluator: Box::new(ManhattanEvaluation),
            max_depth: 6,
        }
    }
}

impl AlphaBetaSolver {
    pub fn new(eval: Box<dyn Evaluation>, max_depth: usize) -> Self {
        Self {
            evaluator: eval,
            max_depth,
        }
    }

    pub fn ab_search(&self, board: &mut Board, next_player: usize, depth: usize, alpha: f64, beta: f64, best_move: &mut Action) -> f64 {
        if depth == 0 {
            let v = self.evaluator.evaluate(board, next_player);
            // println!("{}", v);
            return v;
        }

        let mut alpha = alpha;

        let actions = board.generate_actions_all(next_player);
        for &action in actions.iter() {
            board.perform_action(action);
            let mut ph = Action::from_usize(0, 0, 0, 0);
            let value = -self.ab_search(board, 3 - next_player, depth - 1, -beta, -alpha, &mut ph);
            board.undo_action(action);
            if value >= beta {
                *best_move = action;
                return beta;
            }
            if value > alpha {
                *best_move = action;
                alpha = value;
            }
        }

        alpha
    }
}

impl Solver for AlphaBetaSolver {
    fn solve(&self, board: &Board, next_player: usize) -> Action {
        let all_actions = board.generate_actions_all(next_player);
        let mut action = all_actions[0];
        let mut b = board.clone();

        self.ab_search(&mut b, next_player, self.max_depth, -f64::INFINITY, f64::INFINITY, &mut action);
        action
    }
}
