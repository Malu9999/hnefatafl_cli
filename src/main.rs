mod agent;
mod eval;
mod game;
mod synthesis;

use core::time;
use std::thread;

use agent::Bot;
use agent::{alpha_beta::policy::AlphaBetaBot, mcts::policy::Mcts};
use eval::human_score::{HumanScore, HumanScoreParam};
use eval::{random_rollout::RandomRollout, random_rollout_parallel::RandomRolloutPar, Eval};
use game::{
    board::{Board, GameState},
    piece::PieceColor,
};
use synthesis::network::Network;
use tch::{nn::VarStore, Device, Tensor};

fn main() {
    let mut board = Board::init();

    let mut turn = PieceColor::Attacker;

    let mut vs = VarStore::new(Device::cuda_if_available());

    let net = Network::new(&vs);

    let _ = vs.load("test.net");
    let (observations, targets) = rollout_with_observations(1000);

    println!("done");

    //let eval = <HumanScore as Eval>::init(HumanScoreParam {
    //    w_ring_1: 1.0,
    //    w_ring_2: 1.0,
    //    w_ring_3: 1.0,
    //    w_ring_4: 1.0,
    //    w_corner: 1.0,
    //    w_edge: 1.0,
    //    w_king_dst: 100.0,
    //});

    //let mut mcts = <AlphaBetaBot<HumanScore> as Bot>::init(2.0, Some(&board), eval);

    net.train(observations, targets, 1000, &vs);

    let _ = vs.save("test.net");

    while !board.is_game_over() {
        //mcts.reset(&board);

        //let mov = mcts.get_next_move(&board, 1000).unwrap();

        let mov = board.get_random_move().unwrap();
        println!("{}", mov);
        //mcts.print_root();

        let captured = board.make_move_captured_positions(&mov);

        let obs = board.get_observation().unsqueeze(0).to_device(Device::cuda_if_available());
        println!("{:?}", f32::try_from(net.forward(&obs)));

        println!("{}", board.get_king_pos().unwrap().min_dist_to_corner());

        turn.flip();
        println!("{}", board);

        thread::sleep(time::Duration::from_millis(500));
    }
}

fn rollout_with_observations(num_rollouts: usize) -> (Tensor, Tensor) {
    let mut observations = Vec::<Tensor>::new();
    let mut targets: Vec<f32> = Vec::new();

    let mut white_wins = 0;
    let mut black_wins = 0;

    loop {
        let mut board = Board::init();
        let mut num_moves = 0;

        let mut turn = PieceColor::Attacker;

        let mut current_obs = Vec::<Tensor>::new();

        while !board.is_game_over() {
            let mov = board.get_random_move_color(&turn).unwrap();

            let captured = board.make_move_captured_positions(&mov);

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
                    if (black_wins + white_wins) % 10 == 0{
                        println!("{} {}", black_wins, white_wins);
                    }
                }
            }
            GameState::WinDefender => {
                if white_wins < num_rollouts / 2 {
                    observations.extend(current_obs);
                    targets.extend(&vec![-1.0; num_moves]);
                    white_wins += 1;
                    if (black_wins + white_wins) % 10 == 0{
                        println!("{} {}", black_wins, white_wins);
                    }
                }
            }
            _ => (),
        };

        

        if black_wins + white_wins == num_rollouts {
            break;
        }
    }

    let observations_tensor = Tensor::stack(&observations, 0).to_device(Device::cuda_if_available());
    let targets_tensor = Tensor::from_slice(&targets)
        .unsqueeze(1)
        .to_device(Device::cuda_if_available());

    (observations_tensor, targets_tensor)
}
