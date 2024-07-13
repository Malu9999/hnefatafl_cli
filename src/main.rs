mod game;
mod synthesis;

use game::{
    board::{Board, GameState},
    piece::PieceColor,
};
use synthesis::network::Network;
use tch::{nn::VarStore, Device, Tensor};

fn main() {
    let mut board = Board::init();

    let mut turn = PieceColor::Attacker;

    let vs = VarStore::new(Device::cuda_if_available());

    let net = Network::new(&vs);

    let (observations, targets) = rollout_with_observations(1000);

    net.train(observations, targets, 100, &vs);

    while !board.is_game_over() {
        let mov = board.get_random_move_color(&turn).unwrap();
        println!("{}", mov);

        let captured = board.make_move_captured_positions(&mov);

        let obs = board.get_observation().unsqueeze(0);

        println!("{:?}", f32::try_from(net.forward(&obs)));

        turn.flip();
        println!("{}", board);
    }
}

fn rollout_with_observations(num_rollouts: usize) -> (Tensor, Tensor) {
    let mut observations = Vec::new();
    let mut targets: Vec<f32> = Vec::new();

    for i in 0..num_rollouts {
        let mut board = Board::init();
        let mut num_moves = 0;

        let mut turn = PieceColor::Attacker;

        while !board.is_game_over() {
            let mov = board.get_random_move_color(&turn).unwrap();

            let captured = board.make_move_captured_positions(&mov);

            let obs = board.get_observation();

            observations.push(obs);

            turn.flip();

            num_moves += 1;
        }

        let who_won = match board.who_won() {
            GameState::Draw => 0.0,
            GameState::WinBlack => 1.0,
            GameState::WinWhite => -1.0,
            GameState::Undecided => 0.0,
        };

        targets.extend(&vec![who_won; num_moves]);
    }

    let observations_tensor = Tensor::stack(&observations, 0);
    let targets_tensor = Tensor::from_slice(&targets).unsqueeze(1);

    (observations_tensor, targets_tensor)
}
