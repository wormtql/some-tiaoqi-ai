use crate::{board::board::Board, constants::{RED, BLUE}};
use super::evaluation::Evaluation;

pub struct NaiveBonusEvaluation {
    pub score0: usize,
    pub score1: usize,
    pub score2: usize,
    pub score3: usize,
}

impl Default for NaiveBonusEvaluation {
    fn default() -> Self {
        NaiveBonusEvaluation { score0: 20, score1: 5, score2: 2, score3: 1 }
    }
}

impl NaiveBonusEvaluation {
    pub fn new(scores: (usize, usize, usize, usize)) -> NaiveBonusEvaluation {
        NaiveBonusEvaluation { score0: scores.0, score1: scores.1, score2: scores.2, score3: scores.3 }
    }

    pub fn total_score(&self) -> usize {
        self.score0 + 3 * self.score1 + 5 * self.score2 + 7 * self.score3
    }

    pub fn get_blue_score(&self, board: &Board) -> usize {
        let size = board.size;
        let mut score = 0;
        if board.data[0][0] == BLUE {
            score += self.score0;
        }
    
        if board.data[1][1] == RED {
            score += self.score1;
        }
        if board.data[0][1] == RED {
            score += self.score1;
        }
        if board.data[1][0] == RED {
            score += self.score1;
        }
        
        for i in 0..2 {
            if board.data[2 - i - 1][2] == RED {
                score += self.score2;
            }
            if board.data[2][2 - i - 1] == RED {
                score += self.score2;
            }
        }
        if board.data[2][2] == RED {
            score += self.score2;
        }

        for i in 0..3 {
            if board.data[3 - i - 1][3] == RED {
                score += self.score3;
            }
            if board.data[3][3 - i - 1] == RED {
                score += self.score3;
            }
        }
        if board.data[3][3] == RED {
            score += self.score3;
        }

        score
    }

    pub fn get_red_score(&self, board: &Board) -> usize {
        let size = board.size;
        let mut score = 0;
        if board.data[size - 1][size - 1] == RED {
            score += self.score0;
        }
    
        if board.data[size - 2][size - 2] == RED {
            score += self.score1;
        }
        if board.data[size - 2][size - 1] == RED {
            score += self.score1;
        }
        if board.data[size - 1][size - 2] == RED {
            score += self.score1;
        }
        
        for i in 0..2 {
            if board.data[size - 3 + i + 1][size - 3] == RED {
                score += self.score2;
            }
            if board.data[size - 3][size - 3 + i + 1] == RED {
                score += self.score2;
            }
        }
        if board.data[size - 3][size - 3] == RED {
            score += self.score2;
        }

        for i in 0..3 {
            if board.data[size - 4 + i + 1][size - 4] == RED {
                score += self.score3;
            }
            if board.data[size - 4][size - 4 + i + 1] == RED {
                score += self.score3;
            }
        }
        if board.data[size - 4][size - 4] == RED {
            score += self.score3;
        }

        score
    }
}

impl Evaluation for NaiveBonusEvaluation {
    fn evaluate(&self, board: &Board, next_player: usize) -> f64 {
        let red_score = self.get_red_score(board);
        let blue_score = self.get_blue_score(board);

        let value = if next_player == RED {
            red_score - blue_score
        } else {
            blue_score - red_score
        };

        value as f64
    }
}
