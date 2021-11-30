use crate::game::{score::Score, Game};
use core::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Suggestion<G: Game>(pub G::Move, pub Score, pub Box<Vec<Suggestion<G>>>);

impl<G: Game> Suggestion<G> {
    pub fn get_move(&self) -> &G::Move {
        &self.0
    }
    pub fn get_score(&self) -> &Score {
        &self.1
    }
    pub fn get_suggestions(&self) -> &Vec<Suggestion<G>> {
        &self.2
    }
    pub fn get_mut_suggestions(&mut self) -> &mut Vec<Suggestion<G>> {
        &mut self.2
    }
    pub fn add_suggestions(&mut self, add: Vec<Suggestion<G>>) {
        self.2.extend(add);
    }
}

impl<G: Game> Eq for Suggestion<G> {}
impl<G: Game> Ord for Suggestion<G> {
    fn cmp(&self, other: &Suggestion<G>) -> Ordering {
        Ord::cmp(self.get_score(), other.get_score())
    }
}
impl<G: Game> PartialEq for Suggestion<G> {
    fn eq(&self, other: &Suggestion<G>) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}
impl<G: Game> PartialOrd for Suggestion<G> {
    fn partial_cmp(&self, other: &Suggestion<G>) -> Option<Ordering> {
        Some(Ord::cmp(&self, &other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::five_in_row::{mv::FiveInRowMove, FiveInRow};
    #[test]
    fn it_is_comparable() {
        let winning_suggestion =
            Suggestion::<FiveInRow>(FiveInRowMove::Mine(0, 0), Score::Win, Box::new(Vec::new()));
        let progress_suggestion = Suggestion::<FiveInRow>(
            FiveInRowMove::Mine(0, 0),
            Score::Numeric(1.0),
            Box::new(Vec::new()),
        );
        let losing_suggestion =
            Suggestion::<FiveInRow>(FiveInRowMove::Mine(0, 0), Score::Loss, Box::new(Vec::new()));

        assert!(winning_suggestion > progress_suggestion);
        assert!(winning_suggestion > losing_suggestion);
        assert!(progress_suggestion > losing_suggestion);
    }
}
