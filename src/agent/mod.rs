use crate::{
    eval::Eval,
    game::{board::Board, piece::PieceColor, r#move::Move},
};

pub mod alpha_beta;
pub mod mcts;
pub mod random;

pub trait BotInit {
    type Ev: Eval;
    type Params;

    fn new(board: Option<&Board>, bot_params: Self::Params, eval_fn: Self::Ev) -> Self;
}

pub trait Bot {
    fn get_next_move(&mut self, board: &Board, time: u128) -> Option<Move>;

    fn reset(&mut self, board: &Board);

    fn num_nodes(&self) -> usize;

    fn get_name(&self) -> String;
}
