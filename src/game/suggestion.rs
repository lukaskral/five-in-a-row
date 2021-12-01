use crate::game::{error::Error, score::Score, Game, GameMove};
use core::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Suggestion<G: Game> {
    mv: G::Move,
    score: Score,
    suggestions: Box<Vec<Suggestion<G>>>,
    depth: u8,
}

impl<G: Game> Suggestion<G> {
    pub fn new(mv: G::Move, score: Score) -> Self {
        Self {
            mv: mv,
            score: score,
            suggestions: Box::new(Vec::new()),
            depth: 0,
        }
    }
    pub fn get_move(&self) -> &G::Move {
        &self.mv
    }
    pub fn get_score(&self) -> &Score {
        return &self.score;
    }
    pub fn get_deep_score(&self) -> &Score {
        if self.depth == 0 {
            return self.get_score();
        }
        let scores: Vec<&Score> = self
            .suggestions
            .iter()
            .map(|s| s.get_deep_score())
            .collect::<Vec<_>>();

        let score_result: Result<&Score, Error<G>> = if GameMove::is_mine(self.get_move()) {
            scores
                .iter()
                .min()
                .map_or(Err(Error::NoSuggestionAvailable), |s| Ok(s))
        } else {
            scores
                .iter()
                .max()
                .map_or(Err(Error::NoSuggestionAvailable), |s| Ok(s))
        };
        return score_result.unwrap();
    }

    pub fn get_suggestions(&self) -> &Vec<Suggestion<G>> {
        &self.suggestions
    }
    pub fn get_mut_suggestions(&mut self) -> &mut Vec<Suggestion<G>> {
        &mut self.suggestions
    }
    pub fn add_suggestions(&mut self, parents: &VecDeque<G::Move>, add: Vec<Suggestion<G>>) {
        let mut parents = parents.clone();
        if parents.len() > 0 {
            let maybe_parent = parents.pop_front();
            if let Some(parent) = maybe_parent {
                let maybe_suggestion = self
                    .suggestions
                    .iter_mut()
                    .find(|s| *s.get_move() == parent);
                if let Some(suggestion) = maybe_suggestion {
                    suggestion.add_suggestions(&parents, add.clone());
                    self.depth = suggestion.depth + 1;
                }
            }
        }
        self.depth = u8::max(1, self.depth);
        self.suggestions.extend(add);
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
            Suggestion::<FiveInRow>::new(FiveInRowMove::Mine(0, 0), Score::Win);
        let progress_suggestion =
            Suggestion::<FiveInRow>::new(FiveInRowMove::Mine(0, 0), Score::Numeric(1.0));
        let losing_suggestion =
            Suggestion::<FiveInRow>::new(FiveInRowMove::Mine(0, 0), Score::Loss);

        assert!(winning_suggestion > progress_suggestion);
        assert!(winning_suggestion > losing_suggestion);
        assert!(progress_suggestion > losing_suggestion);
    }

    #[test]
    fn it_computes_deep_score() {
        let mut suggestion =
            Suggestion::<FiveInRow>::new(FiveInRowMove::Mine(0, 0), Score::Numeric(1.0));
        let parents = VecDeque::new();
        suggestion.add_suggestions(
            &parents,
            vec![
                Suggestion::new(FiveInRowMove::Rivals(1, 0), Score::Numeric(0.0)),
                Suggestion::new(FiveInRowMove::Rivals(1, 1), Score::Numeric(-1.0)),
                Suggestion::new(FiveInRowMove::Rivals(1, -1), Score::Numeric(2.0)),
            ],
        );
        assert_eq!(*suggestion.get_score(), Score::Numeric(1.0));
        assert_eq!(*suggestion.get_deep_score(), Score::Numeric(-1.0));
    }
}
