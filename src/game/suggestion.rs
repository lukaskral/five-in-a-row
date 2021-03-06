use crate::game::{error::Error, score::Score, Game, GameMove};
use core::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Suggestion<G: Game> {
    mv: G::Move,
    score: Score,
    deep_score: Option<Score>,
    suggestions: Box<Vec<Suggestion<G>>>,
    depth: u8,
}

impl<G: Game> Suggestion<G> {
    pub fn new(mv: G::Move, score: Score) -> Self {
        Self {
            mv,
            score,
            suggestions: Box::new(Vec::new()),
            deep_score: None,
            depth: 0,
        }
    }
    pub fn get_move(&self) -> &G::Move {
        &self.mv
    }
    pub fn get_score(&self) -> &Score {
        &self.score
    }

    pub fn compute_deep_score(&self) -> &Score {
        if self.depth == 0 || self.suggestions.len() == 0 {
            return self.get_score();
        }
        let scores: Vec<&Score> = self
            .suggestions
            .iter()
            .map(|s| s.compute_deep_score())
            .collect::<Vec<_>>();

        let score_result: Result<&Score, Error<G>> = if GameMove::is_mine(self.get_move()) {
            scores
                .iter()
                .min()
                .map_or(Err(Error::DeepScoreComputationError), |s| Ok(s))
        } else {
            scores
                .iter()
                .max()
                .map_or(Err(Error::DeepScoreComputationError), |s| Ok(s))
        };
        score_result.unwrap()
    }

    pub fn get_deep_score(&self) -> Score {
        if self.depth == 0 {
            return self.get_score().clone();
        }
        self.deep_score.unwrap()
    }

    pub fn get_suggestions(&self) -> &Vec<Suggestion<G>> {
        &self.suggestions
    }
    pub fn add_suggestions(
        &mut self,
        parents: &VecDeque<G::Move>,
        add: Vec<Suggestion<G>>,
    ) -> Result<(), Error<G>> {
        if !parents.is_empty() {
            Self::extend_suggestions(&mut self.suggestions, parents, add)?;
            let deep_score = self.compute_deep_score();
            self.deep_score = Some(deep_score.clone());
        } else {
            self.depth = u8::max(1, self.depth);
            self.suggestions.extend(add);
            let deep_score = self.compute_deep_score();
            self.deep_score = Some(deep_score.clone());
        }
        Ok(())
    }

    pub fn extend_suggestions(
        vec: &mut Vec<Suggestion<G>>,
        parents: &VecDeque<G::Move>,
        add: Vec<Suggestion<G>>,
    ) -> Result<(), Error<G>> {
        let mut parents = parents.clone();
        let parent = parents
            .pop_front()
            .ok_or(Error::SuggestionComputationError)?;
        let suggestion = vec
            .iter_mut()
            .find(|s| *s.get_move() == parent)
            .ok_or(Error::SuggestionComputationError)?;
        suggestion.add_suggestions(&parents, add)?;
        Ok(())
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
    use crate::five_in_a_row::{mv::FiveInRowMove, FiveInRow};

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
        suggestion
            .add_suggestions(
                &VecDeque::new(),
                vec![
                    Suggestion::new(FiveInRowMove::Rivals(1, 0), Score::Numeric(0.0)),
                    Suggestion::new(FiveInRowMove::Rivals(1, 1), Score::Numeric(-1.0)),
                    Suggestion::new(FiveInRowMove::Rivals(1, -1), Score::Numeric(2.0)),
                ],
            )
            .unwrap();
        assert_eq!(*suggestion.get_score(), Score::Numeric(1.0));
        assert_eq!(suggestion.get_deep_score(), Score::Numeric(-1.0));
    }
}
