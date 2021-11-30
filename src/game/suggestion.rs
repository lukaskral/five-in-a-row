use crate::game::{error::Error, score::Score, Game, GameMove};
use core::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Suggestion<G: Game>(pub G::Move, pub Score, pub Box<Vec<Suggestion<G>>>);

impl<G: Game> Suggestion<G> {
    pub fn get_move(&self) -> &G::Move {
        &self.0
    }
    pub fn get_score(&self) -> &Score {
        return &self.1;
    }
    pub fn get_deep_score(&self, depth: u8) -> &Score {
        if depth > 0 {
            let scores: Vec<&Score> = self
                .get_suggestions()
                .iter()
                .map(|s| s.get_deep_score(depth - 1))
                .collect::<Vec<_>>();

            println!("Scores: {:?}", scores);
            let score_result: Result<&Score, Error<G>> = if GameMove::is_mine(self.get_move()) {
                scores
                    .iter()
                    .max()
                    .map_or(Err(Error::NoSuggestionAvailable), |s| Ok(s))
            } else {
                scores
                    .iter()
                    .min()
                    .map_or(Err(Error::NoSuggestionAvailable), |s| Ok(s))
            };
            return score_result.unwrap();
        }
        return &self.get_score();
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
