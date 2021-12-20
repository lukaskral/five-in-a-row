#[path = "suggestion.rs"]
pub mod suggestion;

use crate::api::game_connection::GameConnection;
use crate::game::{error::Error, score::Score, Game};
use crate::gameplay::suggestion::Suggestion;
use std::collections::VecDeque;

pub struct GamePlay<G: Game, C: GameConnection<G>> {
    pub game: G,
    pub suggestions: Vec<Suggestion<G>>,
    pub connection: Option<C>,
}

impl<G: Game, C: GameConnection<G>> GamePlay<G, C> {
    #[allow(dead_code)]
    pub fn from_game(game: G) -> Self {
        Self {
            game,
            suggestions: Vec::new(),
            connection: None,
        }
    }

    pub async fn from_api(mut api: C) -> Result<Self, Error<G>> {
        let game = api.start_game().await?;
        Ok(Self {
            game,
            suggestions: Vec::new(),
            connection: Some(api),
        })
    }

    fn get_single_level_suggestions(
        &self,
        myself: bool,
        parent_moves: &VecDeque<G::Move>,
        count: usize,
    ) -> Result<Vec<Suggestion<G>>, Error<G>> {
        let mut game = self.game.clone();
        for parent_move in parent_moves.iter() {
            game.do_move(*parent_move)?;
        }

        let mut possibilities: Vec<Suggestion<G>> = game
            .get_possible_moves(myself)
            .iter()
            .filter_map(|mv| {
                let mut game_test = game.clone();
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
                let score = pos.get_score();
                if myself {
                    (Score::min(min, *score), Score::max(max, *score))
                } else {
                    (Score::max(min, *score), Score::min(max, *score))
                }
            },
        );
        let threshold = max_score;

        possibilities.sort_by(|a, b| {
            if myself {
                b.get_score().cmp(a.get_score())
            } else {
                a.get_score().cmp(b.get_score())
            }
        });
        let mut suggestions = possibilities
            .iter()
            .filter(|p| {
                if myself {
                    *p.get_score() >= threshold - ((min_score - max_score).abs() * 0.5)
                } else {
                    *p.get_score() <= threshold + ((min_score - max_score).abs() * 0.5)
                }
            })
            .map(|p| p.to_owned())
            .collect::<Vec<_>>();

        if suggestions.len() > count {
            suggestions = suggestions[0..count].to_vec()
        }
        Ok(suggestions)
    }

    fn get_suggestions(
        &mut self,
        myself: bool,
        parents: &VecDeque<G::Move>,
        depth: u8,
    ) -> Result<Vec<Suggestion<G>>, Error<G>> {
        let suggestion_count = usize::from(u8::max(2 * depth, 6) - 4);
        let mut suggestions =
            self.get_single_level_suggestions(myself, parents, suggestion_count)?;
        if depth > 0 {
            for s in suggestions.iter_mut() {
                if s.get_deep_score().is_finished() {
                    continue;
                }
                let mut parents = parents.clone();
                parents.push_back(s.get_move().clone());
                s.add_suggestions(
                    &VecDeque::<G::Move>::new(),
                    self.get_suggestions(!myself, &parents, depth - 1)?,
                )?;
            }
        }
        suggestions.sort_by(|a, b| {
            let sc_a = a.get_deep_score();
            let sc_b = b.get_deep_score();
            if myself {
                sc_b.cmp(&sc_a)
            } else {
                sc_a.cmp(&sc_b)
            }
        });
        Ok(suggestions)
    }

    pub fn compute_suggestions(
        &mut self,
        myself: bool,
        parents: VecDeque<G::Move>,
        depth: u8,
    ) -> Result<(), Error<G>> {
        let suggestions = self.get_suggestions(myself, &parents, depth)?;

        if parents.is_empty() {
            self.suggestions = suggestions.clone();
        } else {
            Suggestion::extend_suggestions(&mut self.suggestions, &parents, suggestions)?;
        }

        Ok(())
    }

    pub fn suggest_move(&mut self, myself: bool) -> Result<Suggestion<G>, Error<G>> {
        if self.suggestions.is_empty() {
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

    pub async fn play(&mut self) -> Result<String, Error<G>> {
        let result = loop {
            let (maybe_rivals_move, maybe_winner) = {
                let connection = self.connection.as_mut().ok_or(Error::Invalid)?;
                connection.await_move().await?
            };
            if let Some(winner) = maybe_winner {
                break Ok(winner);
            }
            if let Some(rivals_move) = maybe_rivals_move {
                self.add_move(rivals_move)?;
                println!("Rival's move: {:?}", rivals_move,);
            }
            self.compute_suggestions(true, VecDeque::new(), 6)?;
            let maybe_suggestion = self.suggest_move(true);
            if let Ok(suggestion) = maybe_suggestion {
                println!("My move: {:?}", suggestion.get_move(),);
                let mv = suggestion.get_move();
                {
                    let connection = self.connection.as_mut().ok_or(Error::Invalid)?;
                    connection.put_move(mv).await?;
                }
                self.add_move(*mv)?;
            }
        };
        result
    }
}
