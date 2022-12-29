use crate::board::board::Board;
use rand::Rng;
use crate::constants::RED;

pub struct NaiveSimulator;

impl Default for NaiveSimulator {
    fn default() -> Self {
        NaiveSimulator
    }
}

impl NaiveSimulator {
    // true if red wins, false if blue wins
    pub fn simulate(&self, board: &Board, next_player: usize) -> bool {
        // println!("simulate start");
        let mut b = board.clone();
        let mut next_player = next_player;
        let mut game_over = board.is_game_over();

        while game_over.is_none() {
            let mut actions = b.generate_actions_forward_only(next_player);
            if actions.is_empty() {
                // actions = b.generate_actions_all(next_player);
                return next_player == RED;
            }

            let random_index = rand::thread_rng().gen::<usize>() % actions.len();

            let action = actions[random_index];
            b.perform_action(action);
            // println!("{}", b);
            next_player = 3 - next_player;
            game_over = b.is_game_over();
        }

        game_over.unwrap() == RED
    }
}
