mod agent;
mod eval;
mod game;
mod gym;
mod synthesis;
mod utils;

use core::time;
use std::thread;

use agent::Bot;
use agent::{alpha_beta::policy::AlphaBetaBot, mcts::policy::Mcts};
use eval::human_score::{HumanScore, HumanScoreParam};
use game::{
    board::{Board, GameState},
    piece::PieceColor,
};
use synthesis::network::Network;
use tch::{nn::VarStore, Device, Tensor};
use utils::gen_data::{self, Generator};

fn main() {
    let mut board = Board::new();

    let mut turn = PieceColor::Attacker;

    let mut vs = VarStore::new(Device::cuda_if_available());

    let net = Network::new(&vs);

    let _ = vs.load("test.net");
    let (observations, targets) = Generator::new(100).generate();

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

        let obs = board
            .get_observation()
            .unsqueeze(0)
            .to_device(Device::cuda_if_available());
        println!("{:?}", f32::try_from(net.forward(&obs)));

        println!("{}", board.get_king_pos().unwrap().min_dist_to_corner());

        turn.flip();
        println!("{}", board);

        thread::sleep(time::Duration::from_millis(500));
    }
}
