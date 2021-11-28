#[path = "game/score.rs"]
pub mod score;
use crate::game::score::Score;
use std::error::Error;

pub trait Game: Clone {
    type Move: Copy + std::fmt::Debug;
    type Error;

    fn get_score(&self) -> Score;
    fn do_move(&mut self, mv: Self::Move) -> Result<(), Self::Error>;
    fn get_possible_moves(&self, myself: bool) -> Vec<Self::Move>;
    fn get_error(&self, source: Option<Box<dyn Error>>) -> Self::Error;
}
