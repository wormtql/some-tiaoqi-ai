use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::board::action::Action;
use crate::constants::{BLUE, DIR4, DIR4_JUMP2, EMPTY, RED};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub size: usize,
    pub data: Vec<Vec<usize>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                let cell = self.data[j][self.size - i - 1];
                if cell == RED {
                    write!(f, "{} ", "o".red())?;
                } else if cell == BLUE {
                    write!(f, "{} ", "o".blue())?;
                } else {
                    write!(f, ". ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut b = Board {
            size,
            data: vec![vec![0; size]; size]
        };

        for i in 0..4 {
            for j in 0..4 {
                b.data[i][j] = RED;
                b.data[size - i - 1][size - j - 1] = BLUE;
            }
        }

        b
    }

    pub fn is_red_winning(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.data[self.size - i - 1][self.size - j - 1] != RED {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_blue_winning(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.data[i][j] != BLUE {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_game_over(&self) -> Option<usize> {
        if self.is_red_winning() {
            Some(RED)
        } else if self.is_blue_winning() {
            Some(BLUE)
        } else {
            None
        }
    }

    pub fn generate_actions_from_point_jumps(&self, x: usize, y: usize, result: &mut HashSet<Action>) {
        let mut queue: Vec<(usize, usize)> = Vec::new();
        let mut vis = [[false; 10]; 10];
        vis[x][y] = true;
        queue.push((x, y));

        while !queue.is_empty() {
            let (px, py) = queue.pop().unwrap();
            for k in 0..4 {
                let nx2 = px as i32 + DIR4_JUMP2[k][0];
                let ny2 = py as i32 + DIR4_JUMP2[k][1];
                if nx2 >= 0 && nx2 < self.size as i32 && ny2 >= 0 && ny2 < self.size as i32 {
                    let nx2 = nx2 as usize;
                    let ny2 = ny2 as usize;

                    let nx = (px as i32 + DIR4[k][0]) as usize;
                    let ny = (py as i32 + DIR4[k][1]) as usize;

                    if !vis[nx2][ny2] && self.data[nx2][ny2] == EMPTY && self.data[nx][ny] != EMPTY {
                        vis[nx2][ny2] = true;

                        result.insert(Action::from_usize(x, y, nx2, ny2));
                        queue.push((nx2, ny2));
                    }
                }
            }
        }
    }

    pub fn generate_actions_from_point_jumps_forward_only(&self, x: usize, y: usize, result: &mut HashSet<Action>) {
        let mut queue: Vec<(usize, usize)> = Vec::new();
        let mut vis = [[false; 10]; 10];
        vis[x][y] = true;
        queue.push((x, y));

        while !queue.is_empty() {
            let (px, py) = queue.pop().unwrap();
            for k in 0..4 {
                let nx2 = px as i32 + DIR4_JUMP2[k][0];
                let ny2 = py as i32 + DIR4_JUMP2[k][1];
                if nx2 >= 0 && nx2 < self.size as i32 && ny2 >= 0 && ny2 < self.size as i32 {
                    let nx2 = nx2 as usize;
                    let ny2 = ny2 as usize;

                    let nx = (px as i32 + DIR4[k][0]) as usize;
                    let ny = (py as i32 + DIR4[k][1]) as usize;

                    if !vis[nx2][ny2] && self.data[nx2][ny2] == EMPTY && self.data[nx][ny] != EMPTY {
                        vis[nx2][ny2] = true;

                        let action = Action::from_usize(x, y, nx2, ny2);
                        if action.is_forward(self.data[x][y]) {
                            result.insert(action);
                        }

                        queue.push((nx2, ny2));
                    }
                }
            }
        }
    }

    pub fn generate_actions_from_point_single_step(&self, x: usize, y: usize, result: &mut HashSet<Action>) {
        // single step
        for k in 0..4 {
            let nx: i32 = x as i32 + DIR4[k][0];
            let ny: i32 = y as i32 + DIR4[k][1];
            if nx >= 0 && nx < self.size as i32 && ny >= 0 && ny < self.size as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if self.data[nx][ny] == EMPTY {
                    result.insert(Action::from_usize(x, y, nx, ny));
                }
            }
        }
    }

    pub fn generate_actions_from_point_single_step_forward_only(&self, x: usize, y: usize, result: &mut HashSet<Action>) {
        for k in 0..4 {
            let nx: i32 = x as i32 + DIR4[k][0];
            let ny: i32 = y as i32 + DIR4[k][1];
            if nx >= 0 && nx < self.size as i32 && ny >= 0 && ny < self.size as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if self.data[nx][ny] == EMPTY {
                    let action = Action::from_usize(x, y, nx, ny);
                    if action.is_forward(self.data[x][y]) {
                        result.insert(action);
                    }
                }
            }
        }
    }

    pub fn generate_actions_all(&self, player: usize) -> Vec<Action> {
        let mut temp = HashSet::new();

        for i in 0..self.size {
            for j in 0..self.size {
                if self.data[i][j] == player {
                    // actions starting from (i, j)
                    self.generate_actions_from_point_single_step(i, j, &mut temp);
                    self.generate_actions_from_point_jumps(i, j, &mut temp);
                }
            }
        }

        temp.iter().cloned().collect()
    }

    pub fn generate_actions_forward_only(&self, player: usize) -> Vec<Action> {
        let mut temp = HashSet::new();

        for i in 0..self.size {
            for j in 0..self.size {
                if self.data[i][j] == player {
                    // actions starting from (i, j)
                    self.generate_actions_from_point_single_step_forward_only(i, j, &mut temp);
                    self.generate_actions_from_point_jumps_forward_only(i, j, &mut temp);
                }
            }
        }

        temp.iter().cloned().collect()
    }

    pub fn perform_action(&mut self, action: Action) {
        self.data[action.to_x as usize][action.to_y as usize] = self.data[action.from_x as usize][action.from_y as usize];
        self.data[action.from_x as usize][action.from_y as usize] = EMPTY;
    }

    pub fn undo_action(&mut self, action: Action) {
        self.data[action.from_x as usize][action.from_y as usize] = self.data[action.to_x as usize][action.to_y as usize];
        self.data[action.to_x as usize][action.to_y as usize] = EMPTY;
    }

    pub fn is_separable(&self) -> bool {
        let mut region: [[bool; 10]; 10] = [[false; 10]; 10];
        for i in 0..self.size {
            for j in 0..self.size {
                let cell = self.data[i][j];
                if cell == RED {
                    for k in 2..4 {
                        let nx1 = i as i32 + DIR4[k][0];
                        let ny1 = j as i32 + DIR4[k][0];
                        if nx1 >= 0 && nx1 < self.size as i32 && ny1 >= 0 && ny1 < self.size as i32 {
                            let nx1 = nx1 as usize;
                            let ny1 = ny1 as usize;
                            let cell1 = self.data[nx1][ny1];
                            if cell1 == BLUE {
                                return false;
                            }
                        }
                    }

                    for u in i..self.size {
                        for v in j..self.size {
                            region[u][v] = true;
                        }
                    }
                }
            }
        }

        for i in 0..self.size {
            for j in 0..self.size {
                if region[i][j] && self.data[i][j] == BLUE {
                    return false;
                }
            }
        }

        true
    }

    pub fn set_row_by_string(&mut self, row: usize, value: &str) {
        for (index, c) in value.chars().enumerate() {
            if c == 'b' {
                self.data[index][self.size - 1 - row] = BLUE;
            } else if c == 'r' {
                self.data[index][self.size - 1 - row] = RED;
            } else {
                self.data[index][self.size - 1 - row] = EMPTY;
            }
        }
    }
}
