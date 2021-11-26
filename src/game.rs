// #[path = "game/score.rs"]
// pub mod score;
use std::error::Error;

pub trait Game: Clone {
    type Move: Copy;
    type Error;

    fn get_score(&self) -> f64;
    fn do_move(&mut self, mv: Self::Move) -> Result<(), Self::Error>;
    fn get_possible_moves(&self) -> Vec<Self::Move>;
    fn get_error(&self, source: Option<Box<dyn Error>>) -> Self::Error;
}