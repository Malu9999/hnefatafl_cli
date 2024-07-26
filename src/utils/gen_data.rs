use std::{thread, vec};

use tch::{Device, Tensor};

use crate::{
    agent::{
        mcts::{self, policy::Mcts},
        Bot, BotInit,
    },
    eval::{neural_net::NeuralNet, EvalInit},
    game::{
        board::{self, Board, GameState},
        piece::PieceColor,
    },
};

pub struct Generator {
    num_games: usize,
}

impl Generator {
    pub fn new(num_games: usize) -> Generator {
        Generator { num_games }
    }

    pub fn generate(
        &self,
        fixed: bool,
        attacker_nn: String,
        defender_nn: String,
    ) -> (Tensor, Tensor, usize, usize) {
        let thread_count: usize = std::thread::available_parallelism()
            .unwrap()
            .to_owned()
            .into();

        let mut handles = vec![];

        for i in 0..thread_count {
            let attacker_mcts = Mcts::new(None, 1.4, NeuralNet::new(attacker_nn.clone()));
            let defender_mcts = Mcts::new(None, 1.4, NeuralNet::new(defender_nn.clone()));
            let num_games = self.num_games / thread_count;
            let fix = fixed;

            handles.push(thread::spawn(move || {
                rollout_with_observations(num_games, fix, attacker_mcts, defender_mcts)
            }))
        }

        let mut observations = Vec::<Tensor>::new();
        let mut targets = Vec::new();
        let mut total_black_wins = 0;
        let mut total_white_wins = 0;

        for handle in handles {
            let (mut observation, mut target, black_wins, white_wins) = handle.join().unwrap();
            observations.append(&mut observation);
            targets.append(&mut target);
            total_black_wins += black_wins;
            total_white_wins += white_wins;
        }

        println!("{}", observations.len());

        let observations_tensor =
            Tensor::stack(&observations, 0).to_device(Device::cuda_if_available());
        let targets_tensor = Tensor::from_slice(&targets)
            .unsqueeze(1)
            .to_device(Device::cuda_if_available());

        (
            observations_tensor,
            targets_tensor,
            total_black_wins,
            total_white_wins,
        )
    }
}

fn rollout_with_observations(
    num_rollouts: usize,
    fixed: bool,
    mut mcts_defender: Mcts<NeuralNet>,
    mut mcts_attacker: Mcts<NeuralNet>,
) -> (Vec<Tensor>, Vec<f32>, usize, usize) {
    let mut observations = Vec::<Tensor>::new();
    let mut targets: Vec<f32> = Vec::new();

    let mut white_wins = 0;
    let mut black_wins = 0;

    loop {
        let mut rollout_board = Board::new();

        let mut num_moves = 0;

        let mut turn = PieceColor::Attacker;

        let mut current_obs = Vec::<Tensor>::new();

        while !rollout_board.is_game_over() {
            let mov = match rollout_board.get_player() {
                PieceColor::Attacker => mcts_attacker.get_next_move(&rollout_board, 100),
                PieceColor::Defender => mcts_defender.get_next_move(&rollout_board, 100),
            }
            .unwrap();

            let _ = rollout_board.make_move_captured_positions(&mov);

            let obs = rollout_board.get_observation();

            current_obs.push(obs);

            turn.flip();

            num_moves += 1;
        }

        match rollout_board.who_won() {
            GameState::WinAttacker => {
                if black_wins < num_rollouts / 2 {
                    observations.extend(current_obs);
                    targets.extend(&vec![1.0; num_moves]);
                }
                println!("new_black");
                black_wins += 1;
            }
            GameState::WinDefender => {
                if white_wins < num_rollouts / 2 {
                    observations.extend(current_obs);
                    targets.extend(&vec![-1.0; num_moves]);
                }
                println!("new_white");
                white_wins += 1;
            }
            _ => (),
        };

        if black_wins.min(num_rollouts / 2) + white_wins.min(num_rollouts / 2) == num_rollouts {
            break;
        }

        if fixed && black_wins + white_wins >= num_rollouts {
            break;
        }
    }

    (observations, targets, black_wins, white_wins)
}
