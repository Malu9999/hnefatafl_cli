mod game;

use game::{board::Board, piece::PieceColor};

fn main() {
    let mut board = Board::init();

    let mut turn = PieceColor::Attacker;

    while !board.is_game_over() {
        let mov = board.get_random_move_color(&turn).unwrap();
        println!("{}", mov);

        let captured = board.make_move_captured_positions(&mov);

        println!("{:?}", captured);

        turn.flip();
        println!("{}", board);
    }
}
