#[path = "suggestion.rs"]
pub mod suggestion;

use crate::game::{error::Error, score::Score, Game};
use crate::gameplay::suggestion::Suggestion;
use std::collections::VecDeque;

pub struct GamePlay<G: Game> {
    pub game: G,
    pub suggestions: Vec<Suggestion<G>>,
}

impl<G: Game> GamePlay<G> {
    pub fn new(game: G) -> Self {
        Self {
            game: game,
            suggestions: Vec::new(),
        }
    }

    fn get_suggestions(
        &self,
        myself: bool,
        parent_moves: &VecDeque<G::Move>,
    ) -> Result<Vec<Suggestion<G>>, Error<G>> {
        let mut game = self.game.clone();
        for parent_move in parent_moves.iter() {
            game.do_move(*parent_move)?;
        }

        let possibilities: Vec<Suggestion<G>> = game
            .get_possible_moves(myself)
            .iter()
            .filter_map(|mv| {
                let mut game_test = self.game.clone();
                let r = game_test.do_move(*mv);
                match r {
                    Ok(_) => {
                        let score = game_test.get_score();
                        Some(Suggestion::new(mv.to_owned(), score))
                    }
                    Err(_) => None,
                }
            })
            .collect::<Vec<_>>();

        let (min_score, max_score) = possibilities.iter().fold(
            if myself {
                (Score::Win, Score::Loss)
            } else {
                (Score::Loss, Score::Win)
            },
            |(min, max), pos| {
                let score = pos.get_deep_score();
                if myself {
                    (Score::min(min, *score), Score::max(max, *score))
                } else {
                    (Score::max(min, *score), Score::min(max, *score))
                }
            },
        );
        let threshold = max_score;

        let mut suggestions = possibilities
            .iter()
            .filter(|p| {
                if myself {
                    *p.get_deep_score() >= threshold - ((min_score - max_score).abs() * 0.3)
                } else {
                    *p.get_deep_score() <= threshold + ((min_score - max_score).abs() * 0.3)
                }
            })
            .map(|p| p.to_owned())
            .collect::<Vec<_>>();

        suggestions.sort_by(|a, b| b.get_deep_score().cmp(a.get_deep_score()));
        if suggestions.len() > 12 {
            suggestions = suggestions[0..11].to_vec()
        }
        Ok(suggestions)
    }

    pub fn compute_suggestions(
        &mut self,
        myself: bool,
        parents: VecDeque<G::Move>,
        depth: u8,
    ) -> Result<(), Error<G>> {
        let mut parents = parents.clone();
        let suggestions = self.get_suggestions(myself, &parents)?;
        if parents.len() == 0 {
            self.suggestions = suggestions.clone();
        } else {
            let maybe_parent = parents.pop_front();
            if let Some(parent) = maybe_parent {
                let maybe_suggestion = self
                    .suggestions
                    .iter_mut()
                    .find(|s| *s.get_move() == parent);
                if let Some(suggestion) = maybe_suggestion {
                    suggestion.add_suggestions(&parents, suggestions.clone());
                }
            }
        }

        if depth > 0 {
            for s in suggestions.iter() {
                if s.get_deep_score().is_finished() {
                    continue;
                }
                let mut parents = parents.clone();
                parents.push_back(s.get_move().clone());
                self.compute_suggestions(!myself, parents, depth - 1)?;
            }
        }

        Ok(())
    }

    pub fn suggest_move(&mut self, myself: bool) -> Result<Suggestion<G>, Error<G>> {
        if self.suggestions.len() == 0 {
            self.compute_suggestions(myself, VecDeque::new(), 0)?;
        }
        self.suggestions
            .get(0)
            .map_or(Err(Error::NoSuggestionAvailable), |s| Ok(s.clone()))
    }

    pub fn add_move(&mut self, mv: G::Move) -> Result<(), Error<G>> {
        let maybe_suggestion: Option<&Suggestion<G>> =
            self.suggestions.iter().find(|s| *s.get_move() == mv);

        // TODO don't clone
        self.suggestions = maybe_suggestion.map_or(Vec::new(), |s| (*s).get_suggestions().clone());
        let res = self.game.do_move(mv);

        Game::visualize(&self.game);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::five_in_row::{mv::FiveInRowMove, FiveInRow};

    #[test]
    fn it_suggests_correct_move_1() {
        let moves = Vec::from([
            FiveInRowMove::Rivals(0, 0),
            FiveInRowMove::Mine(0, -1),
            FiveInRowMove::Rivals(0, 1),
            FiveInRowMove::Mine(0, -2),
            FiveInRowMove::Rivals(0, 2),
            FiveInRowMove::Mine(0, -3),
            FiveInRowMove::Rivals(0, 3),
        ]);
        let game = FiveInRow::from_moves(moves);
        game.visualize();
        let mut game_play = GamePlay::new(game);
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(*suggested.get_move(), FiveInRowMove::Mine(0, 4));
    }

    #[test]
    fn it_suggests_correct_move_2() {
        let moves = Vec::from([
            FiveInRowMove::Mine(0, 0),
            FiveInRowMove::Rivals(0, 1),
            FiveInRowMove::Mine(1, 0),
            FiveInRowMove::Rivals(0, 2),
            FiveInRowMove::Mine(2, 0),
            FiveInRowMove::Rivals(0, 3),
        ]);
        let game = FiveInRow::from_moves(moves);
        game.visualize();
        let mut game_play = GamePlay::new(game);
        game_play
            .compute_suggestions(true, VecDeque::new(), 0)
            .unwrap();
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(*suggested.get_move(), FiveInRowMove::Mine(-1, 0));
    }

    #[test]
    fn it_suggests_correct_move_3() {
        let moves = Vec::from([
            FiveInRowMove::Mine(0, 0),
            FiveInRowMove::Rivals(-1, -1),
            FiveInRowMove::Mine(-1, 1),
            FiveInRowMove::Rivals(1, -1),
            FiveInRowMove::Mine(0, -1),
            FiveInRowMove::Rivals(0, -2),
            FiveInRowMove::Mine(-1, -2),
            FiveInRowMove::Rivals(-1, -3),
            FiveInRowMove::Mine(-2, 0),
            FiveInRowMove::Rivals(2, 0),
        ]);
        let game = FiveInRow::from_moves(moves);
        game.visualize();
        let mut game_play = GamePlay::new(game);
        game_play
            .compute_suggestions(true, VecDeque::new(), 1)
            .unwrap();
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(*suggested.get_move(), FiveInRowMove::Mine(-2, -4));
    }

    #[test]
    fn it_suggests_correct_move_4() {
        let moves = Vec::from([
            FiveInRowMove::Rivals(0, 0),
            FiveInRowMove::Mine(-1, -1),
            FiveInRowMove::Rivals(-1, 0),
            FiveInRowMove::Mine(-2, 0),
            FiveInRowMove::Rivals(-3, 1),
            FiveInRowMove::Mine(-2, -1),
            FiveInRowMove::Rivals(-3, -1),
            FiveInRowMove::Mine(-2, 1),
            FiveInRowMove::Rivals(-2, -2),
            FiveInRowMove::Mine(-2, 2),
            FiveInRowMove::Rivals(-2, 3),
            FiveInRowMove::Mine(-1, 2),
            FiveInRowMove::Rivals(-3, 0),
            FiveInRowMove::Mine(-3, 2),
            FiveInRowMove::Rivals(0, 2),
            FiveInRowMove::Mine(-4, 2),
            FiveInRowMove::Rivals(-5, 2),
            FiveInRowMove::Mine(-3, -3),
            FiveInRowMove::Rivals(-4, 0),
            FiveInRowMove::Mine(-1, -3),
            FiveInRowMove::Rivals(-5, 1),
            FiveInRowMove::Mine(-6, 2),
            FiveInRowMove::Rivals(-5, 3),
            FiveInRowMove::Mine(-5, 4),
            FiveInRowMove::Rivals(-6, 5),
            FiveInRowMove::Mine(-5, 0),
            FiveInRowMove::Rivals(0, 1),
        ]);
        let game = FiveInRow::from_moves(moves);
        game.visualize();
        let mut game_play = GamePlay::new(game);
        game_play
            .compute_suggestions(true, VecDeque::new(), 2)
            .unwrap();
        let suggested = game_play.suggest_move(true).unwrap();
        let suggested_move = *suggested.get_move();
        assert!(
            suggested_move == FiveInRowMove::Mine(0, -1)
                || suggested_move == FiveInRowMove::Mine(0, 3),
            "Expected (0, -1) or (0, 3), got {:?}",
            suggested_move
        );
    }

    #[test]
    fn it_suggests_correct_move_5() {
        let moves = Vec::from([
            FiveInRowMove::Mine(0, 0),
            FiveInRowMove::Rivals(0, 1),
            FiveInRowMove::Mine(-1, -1),
            FiveInRowMove::Rivals(0, 2),
            FiveInRowMove::Mine(0, 3),
            FiveInRowMove::Rivals(-1, 2),
            FiveInRowMove::Mine(-1, 3),
            FiveInRowMove::Rivals(1, 2),
            FiveInRowMove::Mine(2, 2),
            FiveInRowMove::Rivals(1, 1),
            FiveInRowMove::Mine(1, 0),
            FiveInRowMove::Rivals(-2, 2),
            FiveInRowMove::Mine(-3, 2),
            FiveInRowMove::Rivals(-2, 1),
        ]);
        let game = FiveInRow::from_moves(moves);
        game.visualize();
        let mut game_play = GamePlay::new(game);
        game_play
            .compute_suggestions(true, VecDeque::new(), 3)
            .unwrap();
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(*suggested.get_move(), FiveInRowMove::Mine(-1, 1));
    }
}
