#[path = "score.rs"]
pub mod score;
use crate::game::score::Score;
use std::error::Error;
use std::fmt::Debug;

pub trait Game: Clone {
    type Move: Eq + Ord + PartialEq + Copy + Debug;
    type Error;

    fn get_score(&self) -> Score;
    fn do_move(&mut self, mv: Self::Move) -> Result<(), Self::Error>;
    fn get_possible_moves(&self, myself: bool) -> Vec<Self::Move>;
    fn get_error(&self, source: Option<Box<dyn Error>>) -> Self::Error;
}
