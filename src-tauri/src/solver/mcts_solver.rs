use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::board::action::Action;
use crate::board::board::Board;
use crate::constants::{BLUE, RED};
use crate::simulator::naive_simulator::NaiveSimulator;
use crate::solver::solver::Solver;

pub struct MCTSNode {
    pub visit: usize,
    pub win: usize,
    pub score: f64,

    // will-not-change attributes
    pub board: Board,
    pub next_player: usize,
    pub player: usize,
    pub action: Action,
    pub depth: usize,
    pub game_over: Option<usize>,

    pub parent: Option<Weak<RefCell<MCTSNode>>>,
    pub children: Vec<Rc<RefCell<MCTSNode>>>,
}

impl MCTSNode {
    pub fn new(board: &Board, next_player: usize) -> MCTSNode {
        MCTSNode {
            visit: 0,
            win: 0,
            score: 0.0,
            board: board.clone(),
            next_player,
            player: 3 - next_player,
            action: Action::from_usize(0, 0, 0, 0),
            depth: 0,
            game_over: board.is_game_over(),
            parent: None,
            children: Vec::with_capacity(100),
        }
    }

    pub fn expand_from_node(node: Rc<RefCell<MCTSNode>>, action: Action) -> MCTSNode {
        let mut new_board = node.borrow().board.clone();
        new_board.perform_action(action);
        let new_is_game_over = new_board.is_game_over();

        MCTSNode {
            visit: 0,
            win: 0,
            score: 0.0,
            board: new_board,
            next_player: node.borrow().player,
            player: node.borrow().next_player,
            action,
            depth: node.borrow().depth + 1,
            game_over: new_is_game_over,
            parent: Some(Rc::downgrade(&node)),
            children: Vec::with_capacity(100)
        }
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

#[derive(Clone)]
pub struct MCTSSolverConfig {
    pub min_sim: usize,
    pub max_sim: usize,
    pub simulation_count: usize,
    pub times_per_sim: usize,

    pub ucb_constant: f64,
}

impl Default for MCTSSolverConfig {
    fn default() -> Self {
        MCTSSolverConfig {
            min_sim: 5,
            max_sim: 10,
            // simulation_count: 1000,
            simulation_count: 30000,
            times_per_sim: 5,
            ucb_constant: 1.414,
        }
    }
}

pub struct MCTSSolverLogic {
    pub config: MCTSSolverConfig,

    pub total_expand: usize,
    pub total_nodes: usize,
}

impl MCTSSolverLogic {
    pub fn new(config: MCTSSolverConfig) -> Self {
        Self {
            config,
            total_nodes: 0,
            total_expand: 0,
        }
    }

    fn get_ucb(&self, node: Rc<RefCell<MCTSNode>>) -> f64 {
        let k = self.config.ucb_constant;
        let parent_visit = node.borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().visit as f64;
        let visit = node.borrow().visit as f64;

        k * (parent_visit.ln() / visit).sqrt()
    }

    pub fn select(&self, node: Rc<RefCell<MCTSNode>>) -> Rc<RefCell<MCTSNode>> {
        let mut max_score = -1.0;
        let mut max_index = 0;

        for i in 0..node.borrow().children.len() {
            let n = node.borrow().children[i].clone();
            if n.borrow().visit < self.config.min_sim {
                return n;
            }

            let ucb = self.get_ucb(n.clone());
            let visit = n.borrow().visit as f64;
            let win_count = n.borrow().win as f64;

            let score = win_count / visit + ucb;
            n.borrow_mut().score = score;

            if score > max_score {
                max_score = score;
                max_index = i;
            }
        }

        node.borrow().children[max_index].clone()
    }

    pub fn expand(&mut self, node: Rc<RefCell<MCTSNode>>) {
        // println!("expand");
        let next_player = node.borrow().next_player;
        let actions = node.borrow().board.generate_actions_all(next_player);

        self.total_expand += 1;
        for &action in actions.iter() {
            let new_node = MCTSNode::expand_from_node(node.clone(), action);
            let new_node = Rc::new(RefCell::new(new_node));
            node.borrow_mut().children.push(new_node);
            self.total_nodes += 1;
        }
    }

    pub fn update_mcts(&self, node: Rc<RefCell<MCTSNode>>, red_win: usize, total: usize) {
        // println!("update mcts");
        let mut n: Rc<RefCell<MCTSNode>> = node;

        loop {
            if n.borrow().player == RED {
                n.borrow_mut().win += red_win;
            } else {
                n.borrow_mut().win += total - red_win;
            }
            n.borrow_mut().visit += total;

            let temp = match n.borrow().parent {
                None => {
                    break;
                },
                Some(ref x) => {
                    x.upgrade().unwrap().clone()
                }
            };
            n = temp;
        }
    }

    pub fn uct(&mut self, node: Rc<RefCell<MCTSNode>>) {
        while node.borrow().visit < self.config.simulation_count {
            let mut n = node.clone();
            while n.borrow().has_children() {
                n = self.select(n.clone());
            }
            // println!("node selected");

            let visit = n.borrow().visit;
            if visit >= self.config.max_sim {
                let is_game_over = n.borrow().game_over.is_some();
                if !is_game_over {
                    self.expand(n.clone());
                    n = self.select(n.clone());
                }
            }

            let mut red_win = 0;
            match n.borrow().game_over {
                Some(player) => {
                    if player == RED {
                        red_win = self.config.times_per_sim;
                    } else if player == BLUE {
                        red_win = 0
                    } else {
                        panic!("this cannot happen");
                    }
                },
                None => {
                    for _ in 0..self.config.times_per_sim {
                        if NaiveSimulator::default().simulate(&n.borrow().board, n.borrow().next_player) {
                            red_win += 1;
                        }
                    }
                }
            }

            self.update_mcts(n.clone(), red_win, self.config.times_per_sim);
        }
    }
}

fn get_most_visited_children(node: Rc<RefCell<MCTSNode>>) -> Rc<RefCell<MCTSNode>> {
    let mut max_visit = 0;
    let mut max_index = 0_usize;
    for (index, item) in node.borrow().children.iter().enumerate() {
        // let x = serde_json::to_string_pretty(&*item.borrow()).unwrap();
        // println!("{}", x);
        println!("visit of {}: {}", item.borrow().action, item.borrow().visit);
        if item.borrow().visit > max_visit {
            max_visit = item.borrow().visit;
            max_index = index;
        }
    }

    node.borrow().children[max_index].clone()
}

pub struct MCTSSolver {
    pub config: MCTSSolverConfig,
}

impl MCTSSolver {
    pub fn new(config: MCTSSolverConfig) -> Self {
        MCTSSolver {
            config
        }
    }
}

impl Solver for MCTSSolver {
    fn solve(&self, board: &Board, next_player: usize) -> Action {
        let mut root = MCTSNode::new(board, next_player);
        let root = Rc::new(RefCell::new(root));

        let mut logic = MCTSSolverLogic::new(self.config.clone());
        // println!("logic created");
        logic.uct(root.clone());
        println!("done");

        let most_visited_node = get_most_visited_children(root.clone());
        let action = most_visited_node.borrow().action;

        action
    }
}
