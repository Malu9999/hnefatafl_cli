use crate::{
    eval::Eval,
    game::{board::Board, piece::PieceColor, r#move::Move},
};

pub mod alpha_beta;
pub mod mcts;

pub trait Bot {
    type Ev: Eval;

    fn init(exploration_param: f64, board: Option<&Board>, eval_fn: Self::Ev) -> Self;

    fn get_next_move(&mut self, board: &Board, time: u128) -> Option<Move>;

    fn reset(&mut self, board: &Board);

    fn num_nodes(&self) -> usize;

    fn get_name(&self) -> String;
}
