use rand::seq::SliceRandom;
use rand::thread_rng;

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::game::board::{Board, GameState};
use crate::game::piece::{Piece, PieceColor};
use crate::game::r#move::Move;

#[derive(Clone)]
pub struct MctsTreenode {
    terminal: bool,
    state: Board,
    mov: Option<Move>, //move used to get to this state :D
    unexplored_moves: Vec<Move>,
    unexplored_moves_index: usize,
    q_val: f64,
    n_val: usize,
    children: Vec<TreenodeRef>,
    parent: Option<TreenodeRef>,
}

pub type TreenodeRef = Rc<RefCell<MctsTreenode>>;

impl Display for MctsTreenode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "color: {}, Qv: {}, nv: {}, #children: {}",
            Piece::Pawn(self.get_board().get_player().clone()),
            self.q_val,
            self.n_val,
            self.children.len(),
            // self.state
        )?;
        for (idx, child) in self.children.iter().enumerate() {
            writeln!(
                f,
                "{idx}: Q_val: {}, # played: {}, move: {}",
                RefCell::borrow(child).q_val,
                RefCell::borrow(child).n_val,
                RefCell::borrow(child).mov.as_ref().unwrap()
            )?;
        }
        Ok(())
    }
}

impl MctsTreenode {
    pub fn new_root(state: Board) -> MctsTreenode {
        let unexplored_moves = state.get_legal_moves();
        MctsTreenode {
            terminal: state.is_game_over(),
            state,
            mov: None,
            unexplored_moves,
            unexplored_moves_index: 0,
            q_val: 0.0,
            n_val: 0,
            children: vec![],
            parent: None,
        }
    }

    pub fn new_child_node(parent: &TreenodeRef, mov: Move) -> MctsTreenode {
        let parent_borrowed = RefCell::borrow(parent);

        let mut child_state = parent_borrowed.state.clone();

        // make move on child state and get captured positions
        child_state.make_move_captured_positions(&mov);

        let mut unexplored_moves = child_state.get_legal_moves();
        unexplored_moves.shuffle(&mut thread_rng());

        // check_move(&child_state, &unexplored_moves, &child_color);

        MctsTreenode {
            terminal: child_state.is_game_over(),
            state: child_state,
            mov: Some(mov),
            unexplored_moves,
            unexplored_moves_index: 0,
            q_val: 0.0,
            n_val: 0,
            children: vec![],
            parent: Some(Rc::clone(parent)),
        }
    }

    /// Returns a reference to the next best child node by the UCB1 formula
    pub fn get_next_child_ucb(&self, expl_param: f64) -> Option<TreenodeRef> {
        let mut maxucbval = f64::NEG_INFINITY;
        let mut maxchild: Option<&TreenodeRef> = None;

        for child in &self.children {
            let child_ucb_val = RefCell::borrow(child).compute_ucb_val(expl_param);
            if child_ucb_val > maxucbval {
                maxucbval = child_ucb_val;
                maxchild = Some(child);
            }
        }

        Some(Rc::clone(maxchild?))
    }

    /// Computes the UCB1 value of itself.
    fn compute_ucb_val(&self, expl_param: f64) -> f64 {
        let nvf = self.n_val as f64;
        let nvf_parent = RefCell::borrow(self.parent.as_ref().unwrap()).n_val as f64;

        self.q_val / nvf + expl_param * (2.0 * nvf_parent.ln() / nvf).sqrt()
    }

    /// performs back propagation from node switching the sign in each layer
    /// to accomodate for a two-player-game.
    pub fn back_propagation(&mut self, outcome: f64) {
        self.n_val += 1;
        self.q_val += outcome;

        if let Some(parent) = &self.parent {
            RefCell::borrow_mut(parent).back_propagation(-1.0 * outcome);
        }
    }

    /// Chooses the next move to be expanded.
    /// Here we just expand the moves in the given order
    /// This could be improved by using some heuristics.
    pub fn choose_move(&mut self) -> Option<Move> {
        let pre_i = self.unexplored_moves_index;
        if pre_i >= self.unexplored_moves.len() {
            return None;
        }
        self.unexplored_moves_index += 1;
        Some(self.unexplored_moves[pre_i].clone())
    }

    /// performs a random rollout on self.
    pub fn rollout_policy(&self) -> (f64, usize) {
        // initialize board and counter
        let mut rollout_board = self.state.clone();
        let mut num_moves = 0;

        // perform actions as long as the game is not over
        while !rollout_board.is_game_over() {
            // get a random move
            if let Some(mov) = rollout_board.get_random_move() {
                // perform random move and imcrement counter
                rollout_board.make_move_captured_positions(&mov);
                num_moves += 1;
            } else {
                // if player is unable to move, other pary wins
                return match rollout_board.get_player() {
                    PieceColor::Attacker => (-1.0, num_moves),
                    PieceColor::Defender => (1.0, num_moves),
                };
            }
        }

        let reward = match rollout_board.who_won() {
            GameState::WinAttacker => 1.0,
            GameState::WinDefender => -1.0,
            _ => 0.0,
        };

        (reward, num_moves)
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub fn get_mov(&self) -> &Option<Move> {
        &self.mov
    }

    pub fn num_movs(&self) -> usize {
        self.unexplored_moves.len()
    }

    pub fn get_unexplored_moves_idx(&self) -> usize {
        self.unexplored_moves_index
    }

    pub fn get_q_val(&self) -> f64 {
        self.q_val
    }

    pub fn get_n_val(&self) -> usize {
        self.n_val
    }

    pub fn get_children(&self) -> &Vec<TreenodeRef> {
        &self.children
    }

    pub fn add_child(&mut self, new_child: MctsTreenode) {
        self.children.push(Rc::new(RefCell::new(new_child)));
    }

    pub fn get_board(&self) -> &Board {
        &self.state
    }
}

/// A simple function that checks if the moves computed by the update move funtion
/// actually coicide with the correct possible moves computation
/// This function meant for debugging purposes only
#[allow(unused)]
fn check_move(board: &Board, computed_moves: &[Move], color: &PieceColor) {
    // println!("{}", board);
    let mut a = computed_moves.to_owned();
    let mut a_cmp = board.possible_moves_color(color);
    a.sort();
    a_cmp.sort();
    assert_eq!(a, a_cmp);
}
