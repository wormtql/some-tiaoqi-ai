use std::fmt::{Display, Formatter};
use crate::constants::RED;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Action {
    pub from_x: u8,
    pub from_y: u8,
    pub to_x: u8,
    pub to_y: u8,
}

impl Action {
    pub fn from_usize(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Action {
        Action {
            from_x: from_x as u8,
            from_y: from_y as u8,
            to_x: to_x as u8,
            to_y: to_y as u8,
        }
    }

    pub fn is_forward(&self, player: usize) -> bool {
        if player == RED {
            self.to_x >= self.from_x && self.to_y >= self.from_y
        } else {
            self.to_x <= self.from_x && self.to_y <= self.from_y
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) -> ({}, {})", self.from_x, self.from_y, self.to_x, self.to_y)
    }
}
