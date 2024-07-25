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
}

impl<T: Eval> AlphaBetaBot<T> {
    pub fn alpha_beta(
        &mut self,
        board: Board,
        depth: usize,
        mut alpha: f64,
        mut beta: f64,
        max_player: bool,
    ) -> f64 {
        if depth == 0 || board.is_game_over() {
            return self.eval_fn.get_eval(&board);
        }

        if max_player {
            let mut max_eval = f64::NEG_INFINITY;

            let mut legal_moves = board.get_legal_moves();
            legal_moves.shuffle(&mut thread_rng());

            for mov in legal_moves {
                let mut child = board.clone();
                child.make_move_captured_positions(&mov);

                let eval = self.alpha_beta(child, depth - 1, alpha, beta, !max_player);

                max_eval = max_eval.max(eval);
                alpha = alpha.max(max_eval);

                if beta <= max_eval {
                    break;
                }
            }

            max_eval
        } else {
            let mut min_eval = f64::INFINITY;

            let mut legal_moves = board.get_legal_moves();
            legal_moves.shuffle(&mut thread_rng());

            for mov in legal_moves {
                let mut child = board.clone();
                child.make_move_captured_positions(&mov);

                let eval = self.alpha_beta(child, depth - 1, alpha, beta, !max_player);

                min_eval = min_eval.min(eval);
                beta = beta.min(min_eval);

                if alpha >= min_eval {
                    break;
                }
            }

            min_eval
        }
    }
}

impl<T: Eval> BotInit for AlphaBetaBot<T> {
    type Ev = T;
    type Params = usize;

    fn new(board: Option<&Board>, _bot_params: Self::Params, eval_fn: Self::Ev) -> Self {
        AlphaBetaBot {
            board: board.unwrap_or(&Board::new()).clone(),
            eval_fn,
        }
    }
}

impl<T: Eval> Bot for AlphaBetaBot<T> {
    fn get_next_move(&mut self, board: &Board, _time: u128) -> Option<Move> {
        self.reset(board);

        let black_gaming =
            Piece::Pawn(self.board.get_player().clone()).is_color(&PieceColor::Attacker);
        let mut best_val = if black_gaming {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };

        let mut legal_moves = self.board.get_legal_moves();
        legal_moves.shuffle(&mut thread_rng());

        let mut best_move = None;

        for mov in legal_moves {
            let mut current_board = board.clone();

            current_board.make_move_captured_positions(&mov);
            let val = self.alpha_beta(
                current_board,
                0,
                f64::NEG_INFINITY,
                f64::INFINITY,
                !black_gaming,
            );

            if black_gaming {
                if val > best_val {
                    best_val = val;
                    best_move = Some(mov);
                }
            } else if val < best_val {
                best_val = val;
                best_move = Some(mov);
            }
        }

        best_move
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
