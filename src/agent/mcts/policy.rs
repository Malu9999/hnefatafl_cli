use std::cell::RefCell;
use std::rc::Rc;
use std::time;

use crate::agent::Bot;
use crate::eval::{self, Eval};
use crate::game::piece::PieceColor;
use crate::game::{board::Board, r#move::Move};

use super::node::{MctsTreenode, TreenodeRef};

pub const EPIC_VICTORY_REWARD: f64 = 1000.0;

pub struct Mcts<T: Eval> {
    exploration_param: f64,
    tree_root: TreenodeRef,
    num_nodes: usize,
    eval_fn: T,
}

pub struct MctsParams {
    exploration_param: f64,
}

impl<T: Eval> Mcts<T> {
    /// resets the MctsBot to a certain state.
    /// Here, the exploration parameter can be reset as well as the board and color.
    pub fn reset_to(&mut self, exploration_param: f64, board: &Board) {
        self.exploration_param = exploration_param;
        self.tree_root = Rc::new(RefCell::new(MctsTreenode::new_root(board.clone())));
        self.num_nodes = 0;
    }

    /// returns the best possible move from the root node after
    /// children have been calculated.
    /// If this is executed before the first child extention, it will simply return None.
    pub fn get_best_move(&self) -> Option<Move> {
        let mut max_eval = f64::MIN;
        let mut incumbent_mov: Option<Move> = None;

        let root_borrowed = RefCell::borrow(&self.tree_root);
        println!("{}", root_borrowed);
        let children: &Vec<TreenodeRef> = root_borrowed.get_children();

        for child in children.iter() {
            let child_borrowed = RefCell::borrow(child);

            let child_q_val = child_borrowed.get_q_val();
            let child_n_val = child_borrowed.get_n_val() as f64;

            let eval = child_q_val / child_n_val;

            if eval > max_eval {
                incumbent_mov = Some(child_borrowed.get_mov().as_ref().unwrap().clone());
                max_eval = eval;
            }
        }

        incumbent_mov
    }

    pub fn print_root(&self) {
        println!("{}", RefCell::borrow(&self.tree_root));
    }

    /// performs as many MCTS iterations as possible within the given time horizon.
    pub fn grow_with_time_limit(&mut self, time_limit: u128)
    where
        T: Eval,
    {
        let start_time = time::Instant::now();

        while start_time.elapsed().as_millis() < time_limit {
            // get next node to expand and move to be expanded
            let node_to_expand = self
                .tree_policy(self.exploration_param)
                .expect("Tree policy failed.");

            let term = RefCell::borrow(&node_to_expand).is_terminal();

            // if the node is terminal, we find out who won and propagate backwards.
            if term {
                let mut node_to_expand_borrowed = RefCell::borrow_mut(&node_to_expand);

                let eval = self.eval_fn.get_eval(node_to_expand_borrowed.get_board());

                let outcome = match node_to_expand_borrowed.get_board().get_player() {
                    PieceColor::Attacker => -eval,
                    PieceColor::Defender => eval,
                };
                node_to_expand_borrowed.back_propagation(outcome);
                continue;
            }

            // choose the next move todo
            let next_move = RefCell::borrow_mut(&node_to_expand)
                .choose_move()
                .expect("No move found.");

            // create a new child node
            let mut new_child = MctsTreenode::new_child_node(&node_to_expand, next_move);

            // find out who won and generate outcome
            let eval = self.eval_fn.get_eval(new_child.get_board());

            let outcome = match new_child.get_board().get_player() {
                PieceColor::Attacker => -eval,
                PieceColor::Defender => eval,
            };

            // popagate the reward through the tree
            new_child.back_propagation(outcome);

            // add child to the parent node
            RefCell::borrow_mut(&node_to_expand).add_child(new_child);

            self.num_nodes += 1;
        }
    }

    /// performs the tree policy on the MCTS tree yielding the next node to expand.
    /// If all nodes have been expanded, it will return None.
    fn tree_policy(&self, expl_param: f64) -> Option<TreenodeRef> {
        let mut current_node = Rc::clone(&self.tree_root);

        loop {
            // get next index of move
            let i = RefCell::borrow(&current_node).get_unexplored_moves_idx();
            let term = RefCell::borrow(&current_node).is_terminal();

            // if the current node is terminal or there are still some moves not expanded -> return
            if i < RefCell::borrow(&current_node).num_movs() || term {
                return Some(current_node);
            }

            // get the next child node by using ucb
            let next_node = RefCell::borrow(&current_node).get_next_child_ucb(expl_param)?;
            current_node = next_node;
        }
    }

    #[allow(unused)]
    pub fn compute_depth(&self) -> usize {
        let mut current = Rc::clone(&self.tree_root);
        let mut counter = 0;

        loop {
            let next_child = RefCell::borrow(&current).get_children().first().cloned();

            match next_child {
                Some(child) => {
                    counter += 1;
                    current = Rc::clone(&child);
                }
                None => return counter,
            }
        }
    }
}

impl<T: Eval> Bot for Mcts<T> {
    type Ev = T;

    /// creates a new MctsBot.
    /// For this, an exporation_parameter must be provided which will be used in
    /// the UCB_1 formula for decision-making in the tree-policy of MCTS.
    /// Moreover, the color of the root node must be provided.
    fn init(exploration_param: f64, board: Option<&Board>, eval_fn: T) -> Self {
        Mcts {
            exploration_param,
            tree_root: Rc::new(RefCell::new(MctsTreenode::new_root(
                board.unwrap_or(&Board::init()).clone(),
            ))),
            num_nodes: 0,
            eval_fn,
        }
    }

    fn get_next_move(&mut self, board: &Board, time_limit: u128) -> Option<Move> {
        self.reset_to(self.exploration_param, board);
        self.grow_with_time_limit(time_limit);
        self.get_best_move()
    }

    fn reset(&mut self, board: &Board) {
        self.reset_to(self.exploration_param, board);
    }

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn get_name(&self) -> String {
        "MctsBot".to_owned()
    }
}
