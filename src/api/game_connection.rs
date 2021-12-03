use crate::game::{error::Error, Game};
use async_trait::async_trait;

#[async_trait]
pub trait GameConnection<G: Game> {
    async fn start_game(&mut self) -> Result<G, Error<G>>;
    async fn put_move(&mut self, mv: &G::Move) -> Result<(), Error<G>>;
    async fn await_move(&mut self) -> Result<(Option<G::Move>, Option<String>), Error<G>>;
}
