mod game;
mod synthesis;

use game::{board::Board, piece::PieceColor};
use synthesis::network::Network;
use tch::{nn::VarStore, Device};

fn main() {
    let mut board = Board::init();

    let mut turn = PieceColor::Attacker;

    let vs = VarStore::new(Device::cuda_if_available());

    let net = Network::new(&vs);

    while !board.is_game_over() {
        let mov = board.get_random_move_color(&turn).unwrap();
        println!("{}", mov);

        let captured = board.make_move_captured_positions(&mov);

        println!("{:?}", captured);

        turn.flip();
        println!("{}", board);
    }
}
