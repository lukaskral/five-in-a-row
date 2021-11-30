#[path = "suggestion.rs"]
pub mod suggestion;

use crate::game::{error::Error, score::Score, Game};
use crate::gameplay::suggestion::Suggestion;

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
        parent_moves: &Vec<G::Move>,
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
                        Some(Suggestion(mv.to_owned(), score, Box::new(Vec::new())))
                    }
                    Err(_) => None,
                }
            })
            .collect::<Vec<_>>();

        let max_score = possibilities.iter().fold(Score::Loss, |max, pos| {
            let Suggestion(_, score, _) = pos;
            if myself {
                Score::max(max, *score)
            } else {
                Score::min(max, *score)
            }
        });
        let threshold = max_score;

        let mut suggestions = possibilities
            .iter()
            .filter(|p| {
                if myself {
                    *p.get_score() >= threshold - (threshold.abs() * 0.2)
                } else {
                    *p.get_score() <= threshold + (threshold.abs() * 0.2)
                }
            })
            .map(|p| p.to_owned())
            .collect::<Vec<_>>();
        suggestions.sort();
        Ok(suggestions)
    }

    pub fn compute_suggestions(
        &mut self,
        myself: bool,
        parent_moves: &Vec<G::Move>,
        depth: u8,
    ) -> Result<(), Error<G>> {
        let suggestions = self.get_suggestions(myself, parent_moves)?;
        if parent_moves.len() == 0 {
            self.suggestions = suggestions;
        } else {
            let mut maybe_suggestion: Option<&mut Suggestion<G>> = None;
            for parent_move in parent_moves.iter() {
                maybe_suggestion = if let Some(sug_ref) = maybe_suggestion {
                    sug_ref
                        .get_mut_suggestions()
                        .iter_mut()
                        .find(|s| s.get_move() == parent_move)
                } else {
                    self.suggestions
                        .iter_mut()
                        .find(|s| s.get_move() == parent_move)
                };
            }
            maybe_suggestion.map(|s| s.add_suggestions(suggestions));
        }

        /* TODO
        if depth > 0 {
            for s in suggestions.iter() {
                if s.get_score().is_finished() {
                    continue;
                }
                let mut parents = parent_moves.clone();
                parents.push(s.get_move().clone());
                self.compute_suggestions(myself, &parents, depth - 1)?;
            }
        }
        */

        Ok(())
    }

    pub fn suggest_move(&mut self, myself: bool) -> Result<Suggestion<G>, Error<G>> {
        if self.suggestions.len() == 0 {
            self.compute_suggestions(myself, &Vec::new(), 0)?;
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
        self.game.do_move(mv)
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
            FiveInRowMove::Mine(0, -2),
            FiveInRowMove::Rivals(0, 3),
        ]);
        let game = FiveInRow::from_moves(moves);
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
        ]);
        let game = FiveInRow::from_moves(moves);
        let mut game_play = GamePlay::new(game);
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
        let mut game_play = GamePlay::new(game);
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(*suggested.get_move(), FiveInRowMove::Mine(-2, -4));
    }
}