use crate::board::action::Action;
use crate::board::board::Board;

pub trait Solver {
    fn solve(&self, board: &Board, next_player: usize) -> Action;
}

pub trait MaybeSolver {
    fn solve(&self, board: &Board, next_player: usize) -> Option<Action>;
}