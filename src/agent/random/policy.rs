use crate::{
    agent::{Bot, BotInit},
    eval::Eval,
    game::{board::Board, r#move::Move},
};

pub struct RandomBot<T: Eval> {
    eval: T,
}

impl<T: Eval> BotInit for RandomBot<T> {
    type Ev = T;
    type Params = usize;

    fn new(board: Option<&Board>, bot_params: Self::Params, eval_fn: Self::Ev) -> Self {
        RandomBot { eval: eval_fn }
    }
}

impl<T: Eval> Bot for RandomBot<T> {
    fn get_next_move(&mut self, board: &Board, _time: u128) -> Option<Move> {
        board.get_random_move()
    }

    fn reset(&mut self, board: &Board) {
        ()
    }

    fn num_nodes(&self) -> usize {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
