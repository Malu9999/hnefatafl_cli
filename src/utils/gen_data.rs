use std::{thread, vec};

use tch::{Device, Tensor};

use crate::{
    agent::{alpha_beta::policy::AlphaBetaBot, random::policy::RandomBot, Bot, BotInit},
    eval::{neural_net::NeuralNet, random_rollout::RandomRollout, EvalInit},
    game::{
        board::{Board, GameState},
        piece::PieceColor,
    },
};

pub struct Generator {
    num_games: usize,
}

impl Generator {
    /// Create a new generator
    pub fn new(num_games: usize) -> Generator {
        Generator { num_games }
    }

    /// Generate training data
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

        for _ in 0..thread_count {
            let _attacker_mcts =
                Box::new(AlphaBetaBot::new(3, NeuralNet::new(attacker_nn.clone())));
            let _defender_mcts =
                Box::new(AlphaBetaBot::new(3, NeuralNet::new(defender_nn.clone())));

            let attacker_mcts = Box::new(RandomBot::new(1, RandomRollout::new(1)));
            let defender_mcts = Box::new(RandomBot::new(1, RandomRollout::new(1)));
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

/// Rollout a fixed number of games
fn rollout_with_observations(
    num_rollouts: usize,
    fixed: bool,
    mut mcts_defender: Box<dyn Bot>,
    mut mcts_attacker: Box<dyn Bot>,
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
                //println!("new_black");
                black_wins += 1;
            }
            GameState::WinDefender => {
                if white_wins < num_rollouts / 2 {
                    observations.extend(current_obs);
                    targets.extend(&vec![-1.0; num_moves]);
                }
                //println!("new_white");
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
