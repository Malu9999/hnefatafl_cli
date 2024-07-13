use crate::game::{board::Board, r#move::Move, piece::PieceColor};

mod mcts;

pub trait Bot {
    fn init(exploration_param: f64, color: PieceColor, board: Option<&Board>) -> Self;

    fn get_next_move(&mut self, board: &Board, color: PieceColor, time: u128) -> Option<Move>;

    fn num_nodes(&self) -> usize;

    fn get_name(&self) -> String;
}
