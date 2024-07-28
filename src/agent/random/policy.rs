use crate::{
    agent::{Bot, BotInit},
    eval::Eval,
    game::{board::Board, r#move::Move},
};

pub struct RandomBot<T: Eval> {
    _eval: T,
}

impl<T: Eval> BotInit for RandomBot<T> {
    type Ev = T;
    type Params = usize;

    fn new(_bot_params: Self::Params, eval_fn: Self::Ev) -> Self {
        RandomBot { _eval: eval_fn }
    }
}

impl<T: Eval> Bot for RandomBot<T> {
    /// returns a random move
    fn get_next_move(&mut self, board: &Board, _time: u128) -> Option<Move> {
        board.get_random_move()
    }

    fn reset(&mut self, _board: &Board) {}

    fn num_nodes(&self) -> usize {
        0
    }

    fn get_name(&self) -> String {
        "Random".to_string()
    }
}
