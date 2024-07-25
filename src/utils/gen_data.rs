use std::{thread, vec};

use tch::{Device, Tensor};

use crate::game::{
    board::{self, Board, GameState},
    piece::PieceColor,
};

pub struct Generator {
    num_games: usize,
}

impl Generator {
    pub fn new(num_games: usize) -> Generator {
        Generator { num_games }
    }

    pub fn generate(&self) -> (Tensor, Tensor) {
        let thread_count = std::thread::available_parallelism().unwrap().to_owned();

        let mut handles = vec![];

        for _ in 0..thread_count.into() {
            let num_games = self.num_games;
            handles.push(thread::spawn(move || rollout_with_observations(num_games)))
        }

        let mut observations = Vec::<Tensor>::new();
        let mut targets = Vec::new();

        for handle in handles {
            let (mut observation, mut target) = handle.join().unwrap();
            observations.append(&mut observation);
            targets.append(&mut target);
        }

        let observations_tensor =
            Tensor::stack(&observations, 0).to_device(Device::cuda_if_available());
        let targets_tensor = Tensor::from_slice(&targets)
            .unsqueeze(1)
            .to_device(Device::cuda_if_available());

        (observations_tensor, targets_tensor)
    }
}

fn rollout_with_observations(num_rollouts: usize) -> (Vec<Tensor>, Vec<f32>) {
    let mut observations = Vec::<Tensor>::new();
    let mut targets: Vec<f32> = Vec::new();

    let mut white_wins = 0;
    let mut black_wins = 0;

    loop {
        let mut board = Board::new();
        let mut num_moves = 0;

        let mut turn = PieceColor::Attacker;

        let mut current_obs = Vec::<Tensor>::new();

        while !board.is_game_over() {
            let mov = board.get_random_move_color(&turn).unwrap();

            let _ = board.make_move_captured_positions(&mov);

            let obs = board.get_observation();

            current_obs.push(obs);

            turn.flip();

            num_moves += 1;
        }

        match board.who_won() {
            GameState::WinAttacker => {
                if black_wins < num_rollouts / 2 {
                    observations.extend(current_obs);
                    targets.extend(&vec![1.0; num_moves]);
                    black_wins += 1;
                }
            }
            GameState::WinDefender => {
                if white_wins < num_rollouts / 2 {
                    observations.extend(current_obs);
                    targets.extend(&vec![-1.0; num_moves]);
                    white_wins += 1;
                }
            }
            _ => (),
        };

        if black_wins + white_wins == num_rollouts {
            break;
        }
    }

    (observations, targets)
}
