mod game;

use clap::Parser;
use game::{board::Board, piece::PieceColor};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

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

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
