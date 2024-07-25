mod agent;
mod eval;
mod game;
mod gym;
mod synthesis;
mod utils;

use core::time;
use std::thread;

use agent::{alpha_beta::policy::AlphaBetaBot, mcts::policy::Mcts, random::policy::RandomBot};
use agent::{Bot, BotInit};
use eval::human_score::{HumanScore, HumanScoreParam};
use eval::neural_net::NeuralNet;
use eval::random_rollout::RandomRollout;
use eval::EvalInit;
use game::{
    board::{Board, GameState},
    piece::PieceColor,
};
use synthesis::network::Network;
use tch::{nn::VarStore, Device, Tensor};
use utils::gen_data::{self, Generator};

fn main() {
    help_me();
    /*let mut board = Board::new();

    let mut turn = PieceColor::Attacker;

    //let mut vs = VarStore::new(Device::cuda_if_available());

    let mut net = Network::new();

    net.load("model.ot");
    //let (observations, targets) = Generator::new(100).generate();

    println!("done");

    let eval = HumanScore::new(HumanScoreParam {
        w_ring_1: 0.0,
        w_ring_2: 0.0,
        w_ring_3: 0.0,
        w_ring_4: 0.0,
        w_corner: -1.0,
        w_edge: 0.0,
        w_king_dst: -1.0,
    });

    let mut alphabeta = AlphaBetaBot::new(Some(&board), 3, eval);

    let mut mcts = Mcts::new(Some(&board), 2.0, RandomRollout::new(1));
    //let mut random = RandomBot::new(Some(&board), 2, RandomRollout::new(1));
    //let mut mcts = Mcts::new(Some(&board), 2.0, RandomRollout::new(1));

    //net.train(observations, targets, 1000, &vs);

    //let _ = vs.save("test.net");

    while !board.is_game_over() {
        //mcts.reset(&board);

        let mov = match turn {
            PieceColor::Attacker => mcts.get_next_move(&board, 100).unwrap(),
            PieceColor::Defender => mcts.get_next_move(&board, 100).unwrap(),
        };

        //let mov = board.get_random_move().unwrap();
        println!("{}", mov);
        //mcts.print_root();

        let captured = board.make_move_captured_positions(&mov);

        let obs = board
            .get_observation()
            .unsqueeze(0)
            .to_device(Device::cuda_if_available());
        println!("{:?}", f32::try_from(net.forward(&obs)));

        //println!("{}", board.get_king_pos().unwrap().min_dist_to_corner());

        turn.flip();
        println!("{}", board);

        thread::sleep(time::Duration::from_millis(500));
    }*/
}

fn help_me() {
    let mut net = Network::new();

    for i in 0..100 {
        net.save("old.ot");

        // gen training data
        let gen = Generator::new(112);
        let (observations, targets, _, _) =
            gen.generate(false, "old.ot".to_string(), "old.ot".to_string());

        net.train(observations, targets, 10);

        // test agains new network
        net.save("new.ot");

        let (_, _, black_wins_old, white_wins_new) =
            gen.generate(true, "old.ot".to_string(), "new.ot".to_string());

        let (_, _, black_wins_new, white_wins_old) =
            gen.generate(true, "new.ot".to_string(), "old.ot".to_string());

        if black_wins_new > black_wins_old && white_wins_new > white_wins_old {
            println!("I'm better {}", i);
            net.load("old.ot");
        } else {
            println!("I'm worse {}", i);
        }
    }
}
