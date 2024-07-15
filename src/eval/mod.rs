use crate::game::{board::Board, r#move::Move};

pub mod human_score;
pub mod random_rollout;
pub mod random_rollout_parallel;

pub trait Eval {
    type Param;

    fn init(param: Self::Param) -> Self;

    fn get_eval(&self, board: &Board) -> f64;

    fn update(&mut self, board: Board);
}
