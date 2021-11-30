#[path = "error.rs"]
pub mod error;
#[path = "score.rs"]
pub mod score;
use crate::game::score::Score;
use std::fmt::Debug;

pub trait Game: Clone + Debug {
    type Move: Eq + Ord + PartialEq + Copy + Debug;

    fn get_score(&self) -> Score;
    fn do_move(&mut self, mv: Self::Move) -> Result<(), error::Error<Self>>;
    fn get_possible_moves(&self, myself: bool) -> Vec<Self::Move>;
}
