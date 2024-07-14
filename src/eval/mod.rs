use crate::game::{board::Board, r#move::Move};

pub mod human_score;
pub mod random_rollout;
pub mod random_rollout_parallel;

pub trait Eval {
    fn init() -> Self;

    fn get_eval(&self, board: &Board) -> f64;

    fn update(&mut self, board: Board);
}
