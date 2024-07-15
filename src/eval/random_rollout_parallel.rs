use std::thread;

use crate::game::{
    board::{Board, GameState},
    piece::PieceColor,
    r#move::Move,
};

use super::Eval;

pub struct RandomRolloutPar;

impl Eval for RandomRolloutPar {
    type Param = usize;

    fn init(param: usize) -> Self {
        RandomRolloutPar
    }

    fn get_eval(&self, board: &Board) -> f64 {
        let num_threads = thread::available_parallelism().unwrap().get() * 4;

        let mut handles = Vec::new();

        for i in 0..num_threads {
            let mut rollout_board = board.clone();
            handles.push(thread::spawn(move || {
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
            }));
        }

        let mut values = Vec::<f64>::with_capacity(num_threads);

        for handle in handles {
            values.push(handle.join().unwrap());
        }

        values.iter().sum::<f64>() / num_threads as f64

        // perform actions as long as the game is not over
    }

    fn update(&mut self, board: Board) {
        ()
    }
}
