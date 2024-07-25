use crate::game::{
    board::{Board, GameState},
    piece::PieceColor,
    r#move::Move,
};

use super::{Eval, EvalInit};

pub struct RandomRollout;

impl EvalInit for RandomRollout {
    type Param = usize;

    fn new(param: Self::Param) -> Self {
        RandomRollout
    }
}

impl Eval for RandomRollout {
    fn get_eval(&self, board: &Board) -> f64 {
        let mut rollout_board = board.clone();

        // perform actions as long as the game is not over
        while !rollout_board.is_game_over() {
            // get a random move
            if let Some(mov) = rollout_board.get_random_move() {
                // perform random move and imcrement counter
                rollout_board.make_move_captured_positions(&mov);
            } else {
                // if player is unable to move, other pary wins
                return match rollout_board.get_player() {
                    PieceColor::Attacker => -1.0,
                    PieceColor::Defender => 1.0,
                };
            }
        }

        match rollout_board.who_won() {
            GameState::WinAttacker => 1.0,
            GameState::WinDefender => -1.0,
            _ => 0.0,
        }
    }

    fn update(&mut self, board: Board) {
        ()
    }
}
