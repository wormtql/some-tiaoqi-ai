use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::Hasher;
use smallvec::SmallVec;
use crate::board::action::Action;
use crate::board::board::Board;
use crate::constants::{EMPTY, RED, BLUE};
use crate::solver::solver::{MaybeSolver, Solver};

pub struct BruteForceSearchSolver {
    pub max_depth: usize,
}

impl Default for BruteForceSearchSolver {
    fn default() -> Self {
        Self {
            max_depth: 5,
        }
    }
}

struct Node {
    data: Board,
    depth: usize,
    actions: SmallVec<[Action; 32]>,
}

fn get_board_hash(board: &Board) -> u64 {
    let mut hasher = DefaultHasher::new();
    for i in 0..board.size {
        for j in 0..board.size {
            hasher.write_usize(board.data[i][j]);
        }
    }
    hasher.finish()
}

impl BruteForceSearchSolver {
    pub fn new(max_depth: usize) -> BruteForceSearchSolver {
        BruteForceSearchSolver {
            max_depth,
        }
    }

    pub fn extract_board(&self, board: &Board, player: usize) -> Board {
        let mut ret = board.clone();
        let size = board.size;
        for i in 0..size {
            for j in 0..size {
                if ret.data[i][j] != player {
                    ret.data[i][j] = EMPTY;
                }
            }
        }
        ret
    }

    pub fn dfs(&self, board: &mut Board, actions: &mut SmallVec<[Action; 32]>, depth: usize, player: usize) -> bool {
        if player == RED && board.is_red_winning() {
            return true;
        } else if player == BLUE && board.is_blue_winning() {
            return true;
        }

        if depth == self.max_depth {
            return false;
        }

        let candidate_actions = board.generate_actions_all(player);
        for &action in candidate_actions.iter() {
            board.perform_action(action);
            actions.push(action);
            if self.dfs(board, actions, depth + 1, player) {
                return true;
            }
            board.undo_action(action);
            actions.pop();
        }

        false
    }

    pub fn search(&self, board: &Board, max_depth: usize, player: usize) -> Option<SmallVec<[Action; 32]>> {
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut vis: HashSet<u64> = HashSet::new();

        queue.push_back(Node {
            data: board.clone(),
            depth: 0,
            actions: SmallVec::new(),
        });
        vis.insert(get_board_hash(board));

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            // println!("{}", max_depth);

            if player == RED && p.data.is_red_winning() {
                return Some(p.actions.clone());
            } else if player == BLUE && p.data.is_blue_winning() {
                return Some(p.actions.clone());
            }
            if p.depth == max_depth {
                // println!("{}", p.depth);
                continue;
            }

            let actions = p.data.generate_actions_all(player);
            // println!("{}", actions.len());

            for &action in actions.iter() {
                let mut new_board = p.data.clone();
                new_board.perform_action(action);
                let new_hash = get_board_hash(&new_board);
                let mut new_actions = p.actions.clone();
                new_actions.push(action);
                if !vis.contains(&new_hash) {
                    queue.push_back(Node {
                        data: new_board,
                        depth: p.depth + 1,
                        actions: new_actions
                    })
                }
            }
        }

        None
    }
}

impl MaybeSolver for BruteForceSearchSolver {
    fn solve(&self, board: &Board, next_player: usize) -> Option<Action> {
        let mut b = self.extract_board(board, next_player);
        // let result = self.search(&b, self.max_depth, next_player);
        // match result {
        //     Some(x) => Some(x[0]),
        //     None => None
        // }

        let mut result_actions: SmallVec<[Action; 32]> = SmallVec::new();
        let result = self.dfs(&mut b, &mut result_actions, 0, next_player);
        if result {
            println!("{:?}", result_actions);
            Some(result_actions[0])
        } else {
            None
        }
    }
}
