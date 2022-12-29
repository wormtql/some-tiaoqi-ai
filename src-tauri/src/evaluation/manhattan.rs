use smallvec::SmallVec;
use crate::board::board::Board;
use crate::constants::{BLUE, EMPTY, RED};
use crate::evaluation::evaluation::Evaluation;

pub struct ManhattanEvaluation;

impl ManhattanEvaluation {
    pub fn dis_red(&self, board: &Board) -> usize {
        // let mut empty_cells: SmallVec<[(usize, usize); 16]> = SmallVec::new();
        // let size = board.size;
        // for i in 0..4 {
        //     for j in 0..4 {
        //         if board[size - i - 1][size - j - 1] == EMPTY {
        //             empty_cells.push((i, j));
        //         }
        //     }
        // }

        let mut result = 0;
        let size = board.size;
        // let mut iter = 0_usize;

        for i in 0..size {
            for j in 0..size {
                if i >= size - 4 && j >= size - 4 {
                    continue;
                }
                if board.data[i][j] == RED {
                    // result += empty_cells[iter].0 as i32 - i as i32 + empty_cells[iter].1 as i32 - j as i32;
                    // iter += 1;
                    if i < size - 4 {
                        result += size - 4 - i;
                    }
                    if j < size - 4 {
                        result += size - 4 - j;
                    }
                }
            }
        }

        result
        // result.abs() as usize
    }

    pub fn dis_blue(&self, board: &Board) -> usize {
        let mut result = 0;
        let size = board.size;

        for i in 0..size {
            for j in 0..size {
                if i < 4 && j < 4 {
                    continue;
                }
                if board.data[i][j] == BLUE {
                    if i >= 4 {
                        result += i - 3;
                    }
                    if j >= 4 {
                        result += j - 3;
                    }
                }
            }
        }

        result
    }

    pub fn dis_red2(&self, board: &Board) -> usize {
        let mut result = 0.0;
        let size = board.size;

        for i in 0..size {
            for j in 0..size {
                if board.data[i][j] == RED {
                    result += (size as f64 - 2.5 - j as f64).abs();
                    result += (size as f64 - 2.5 - i as f64).abs();
                }
            }
        }

        result as usize
    }

    pub fn dis_blue2(&self, board: &Board) -> usize {
        let mut result = 0.0;
        let size = board.size;

        for i in 0..size {
            for j in 0..size {
                if board.data[i][j] == BLUE {
                    result += (i as f64 - 1.5).abs();
                    result += (j as f64 - 1.5).abs();
                }
            }
        }

        result as usize
    }

    pub fn bonus_red(&self, board: &Board) -> usize {
        let mut ans = 0;
        let size = board.size;
        ans += if board.data[size - 1][size - 1] == RED { 3 } else { 0 };
        ans += if board.data[size - 4][size - 1] == RED { 2 } else { 0 };
        ans += if board.data[size - 4][size - 4] == RED { 2 } else { 0 };
        ans += if board.data[size - 1][size - 4] == RED { 2 } else { 0 };
        ans += if board.data[size - 2][size - 1] == RED { 1 } else { 0 };
        ans += if board.data[size - 3][size - 1] == RED { 1 } else { 0 };
        ans += if board.data[size - 1][size - 2] == RED { 1 } else { 0 };
        ans += if board.data[size - 1][size - 3] == RED { 1 } else { 0 };
        ans += if board.data[size - 4][size - 2] == RED { 1 } else { 0 };
        ans += if board.data[size - 4][size - 3] == RED { 1 } else { 0 };
        ans += if board.data[size - 2][size - 4] == RED { 1 } else { 0 };
        ans += if board.data[size - 3][size - 4] == RED { 1 } else { 0 };

        ans
    }

    pub fn bonus_blue(&self, board: &Board) -> usize {
        let mut ans = 0;
        ans += if board.data[0][0] == RED { 3 } else { 0 };
        ans += if board.data[0][3] == RED { 2 } else { 0 };
        ans += if board.data[3][3] == RED { 2 } else { 0 };
        ans += if board.data[3][0] == RED { 2 } else { 0 };
        ans += if board.data[1][0] == RED { 1 } else { 0 };
        ans += if board.data[2][0] == RED { 1 } else { 0 };
        ans += if board.data[0][1] == RED { 1 } else { 0 };
        ans += if board.data[0][2] == RED { 1 } else { 0 };
        ans += if board.data[3][1] == RED { 1 } else { 0 };
        ans += if board.data[3][2] == RED { 1 } else { 0 };
        ans += if board.data[1][3] == RED { 1 } else { 0 };
        ans += if board.data[2][3] == RED { 1 } else { 0 };

        ans
    }
}

impl Evaluation for ManhattanEvaluation {
    fn evaluate(&self, board: &Board, next_player: usize) -> f64 {
        // let size = board.size;
        // let total = (size - 4) * 2 * 16;
        //

        match board.is_game_over() {
            Some(x) => {
                return if (x == RED && next_player == RED) || (x == BLUE && next_player == BLUE) {
                    f64::INFINITY
                } else {
                    -f64::INFINITY
                }
            },
            _ => ()
        }

        let dis_red = self.dis_red2(board);
        let dis_blue = self.dis_blue2(board);
        let v = if next_player == RED {
            dis_blue as i32 - dis_red as i32
        } else {
            dis_red as i32 - dis_blue as i32
        };

        let bonus_red = self.bonus_red(board);
        let bonus_blue = self.bonus_blue(board);
        let bonus = if next_player == RED {
            bonus_red as i32 - bonus_blue as i32
        } else {
            bonus_blue as i32 - bonus_red as i32
        };

        (v + bonus) as f64
    }
}
