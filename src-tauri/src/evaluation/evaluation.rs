use crate::board::board::Board;

pub trait Evaluation {
    fn evaluate(&self, board: &Board, next_player: usize) -> f64;
}