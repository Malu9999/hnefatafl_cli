use crate::game::board::Board;

pub mod human_score;
pub mod neural_net;
pub mod random_rollout;
pub mod random_rollout_parallel;

pub trait EvalInit {
    type Param;

    fn new(param: Self::Param) -> Self;
}

pub trait Eval {
    /// Returns the evaluation of the given board.
    fn get_eval(&self, board: &Board) -> f64;

    /// Updates the evaluation with the given board.
    #[allow(unused)]
    fn update(&mut self, board: Board);
}
