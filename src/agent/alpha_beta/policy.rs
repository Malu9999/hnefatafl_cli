use core::f64;
use std::time;

use crate::{
    agent::{Bot, BotInit},
    eval::Eval,
    game::{
        board::Board,
        piece::{Piece, PieceColor},
        r#move::Move,
    },
};

use rand::{prelude::SliceRandom, thread_rng};

pub struct AlphaBetaBot<T: Eval> {
    board: Board,
    eval_fn: T,
    max_depth: usize,
    best_move: Option<Move>,
}

impl<T: Eval> AlphaBetaBot<T> {
    pub fn alpha_beta(
        &mut self,
        board: Board,
        depth: usize,
        mut alpha: f64,
        beta: f64,
        max_player: bool,
        dist_from_root: usize,
        first_move: Option<Move>,
    ) -> f64 {
        if depth == 0 || board.is_game_over() {
            let factor = if max_player { 1.0 } else { -1.0 };
            if board.is_game_over() {
                let addition: f64 = match board.who_won() {
                    crate::game::board::GameState::Undecided => 0,
                    crate::game::board::GameState::WinAttacker => -(dist_from_root as i32),
                    crate::game::board::GameState::WinDefender => dist_from_root as i32,
                    crate::game::board::GameState::Draw => 0,
                } as f64;
                return factor * self.eval_fn.get_eval(&board) + addition;
            } else {
                return factor * self.eval_fn.get_eval(&board);
            }
        }

        let mut legal_moves = board.get_legal_moves();
        legal_moves.shuffle(&mut thread_rng());
        if let Some(fm) = first_move.clone() {
            legal_moves.insert(0, fm);
        }

        let mut value = f64::NEG_INFINITY;

        for (idx, mov) in legal_moves.iter().enumerate() {
            if idx != 0 && first_move.as_ref().is_some_and(|fm| *fm == *mov) {
                continue;
            }

            let mut child = board.clone();
            child.make_move_captured_positions(&mov);

            let eval = -self.alpha_beta(
                child,
                depth - 1,
                -beta,
                -alpha,
                !max_player,
                dist_from_root + 1,
                None,
            );

            if eval > value && dist_from_root == 0 {
                self.best_move = Some(mov.clone());
            }

            value = value.max(eval);
            alpha = alpha.max(value);

            if alpha >= beta {
                break;
            }
        }

        value
    }
}

impl<T: Eval> BotInit for AlphaBetaBot<T> {
    type Ev = T;
    type Params = usize;

    fn new(board: Option<&Board>, bot_params: Self::Params, eval_fn: Self::Ev) -> Self {
        AlphaBetaBot {
            board: board.unwrap_or(&Board::new()).clone(),
            eval_fn,
            max_depth: bot_params,
            best_move: None,
        }
    }
}

impl<T: Eval> Bot for AlphaBetaBot<T> {
    fn get_next_move(&mut self, board: &Board, time: u128) -> Option<Move> {
        let start_time = time::Instant::now();

        self.reset(board);

        let black_gaming =
            Piece::Pawn(self.board.get_player().clone()).is_color(&PieceColor::Attacker);

        self.best_move = None;

        for i in 1..=self.max_depth {
            self.alpha_beta(
                board.clone(),
                i,
                f64::NEG_INFINITY + 1.0,
                f64::INFINITY - 1.0,
                black_gaming,
                0,
                self.best_move.clone(),
            );

            /*if start_time.elapsed().as_millis() > time {
                break;
            }*/
        }

        /*for mov in legal_moves {
            let mut current_board = board.clone();

            current_board.make_move_captured_positions(&mov);
            let val = self.alpha_beta(
                current_board,
                self.max_depth,
                f64::NEG_INFINITY,
                f64::INFINITY,
                !black_gaming,
                0,
            );

            println!("{}, {}", mov, val);

            if black_gaming {
                if val > best_val {
                    best_val = val;
                    best_move = Some(mov);
                }
            } else if val < best_val {
                println!("{}", mov);
                best_val = val;
                best_move = Some(mov);
            }
        }*/

        self.best_move.clone()
    }

    fn reset(&mut self, board: &Board) {
        self.board = board.clone();
    }

    fn num_nodes(&self) -> usize {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
