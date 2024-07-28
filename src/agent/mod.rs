use crate::{
    eval::Eval,
    game::{board::Board, r#move::Move},
};

pub mod alpha_beta;
pub mod mcts;
pub mod random;

pub trait BotInit {
    type Ev: Eval;
    type Params;

    fn new(bot_params: Self::Params, eval_fn: Self::Ev) -> Self;
}

pub trait Bot {
    /// Get the next move for the bot
    fn get_next_move(&mut self, board: &Board, time: u128) -> Option<Move>;

    /// Reset the bot for a new state
    fn reset(&mut self, board: &Board);

    /// Get the number of nodes explored by the bot
    fn num_nodes(&self) -> usize;

    /// Get the name of the bot
    fn get_name(&self) -> String;
}
