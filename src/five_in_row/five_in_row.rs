#[path = "direction.rs"]
mod dir;

#[path = "move.rs"]
pub mod mv;

use crate::api::status::Coordinate;
use crate::five_in_row::dir::Direction;
use crate::five_in_row::mv::FiveInRowMove;
use crate::game::{error::Error, score::Score, Game, GameMove};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct FiveInRow {
    pub moves: Vec<FiveInRowMove>,
}

impl FiveInRow {
    pub fn create_empty() -> Self {
        Self { moves: Vec::new() }
    }
    pub fn from_api_coordinates(resp: Vec<Coordinate>, player_id: &str) -> Self {
        let moves: Vec<FiveInRowMove> = resp
            .iter()
            .map(|c| {
                if player_id.eq(&c.playerId) {
                    return FiveInRowMove::Mine(c.x, c.y);
                } else {
                    return FiveInRowMove::Rivals(c.x, c.y);
                }
            })
            .collect();
        Self { moves: moves }
    }
    pub fn from_moves(moves: Vec<FiveInRowMove>) -> Self {
        Self { moves: moves }
    }

    fn score_from_row(mv: &FiveInRowMove, vec: &Vec<&FiveInRowMove>) -> Score {
        let mut moves: Vec<&FiveInRowMove> = vec.clone();
        moves.sort();

        let pos = moves.iter().position(|m| *m == mv).unwrap();
        let mut r_item = mv;
        let mut r_closing: Option<&FiveInRowMove> = None;
        let mut total_iter_cnt = 1;

        let mut i = pos;
        loop {
            let maybe_current = moves.get(i);
            if let Some(current) = maybe_current {
                if mv.is_same_type(Some(current)) {
                    total_iter_cnt = total_iter_cnt + 1;
                    r_item = current;
                } else {
                    r_closing = Some(*current);
                    break;
                }
            } else {
                break;
            }
            if i >= moves.len() - 1 {
                break;
            }
            i = i + 1;
        }

        let mut l_item = mv;
        let mut l_closing: Option<&FiveInRowMove> = None;
        let mut i = pos;
        loop {
            let maybe_current = moves.get(i);
            if let Some(current) = maybe_current {
                if mv.is_same_type(Some(current)) {
                    total_iter_cnt = total_iter_cnt + 1;
                    l_item = current;
                } else {
                    l_closing = Some(*current);
                    break;
                }
            } else {
                break;
            }
            if i == 0 {
                break;
            }
            i = i - 1;
        }
        total_iter_cnt = total_iter_cnt - 2;
        let total_iter_dist = l_item.get_distance(r_item).abs() + 1;

        if let (Some(l_cl), Some(r_cl)) = (l_closing, r_closing) {
            let gap = l_cl.get_distance(r_cl).abs();
            if gap <= 5 {
                return Score::Numeric(0.0);
            }
        }
        let mut score: Score;
        if total_iter_cnt >= 5 {
            if total_iter_cnt == total_iter_dist {
                return if GameMove::is_mine(mv) {
                    Score::Win
                } else {
                    Score::Loss
                };
            }
            score = Score::Numeric(1000.0 / f64::from(total_iter_dist));
        } else if total_iter_cnt >= 4 {
            score = Score::Numeric(100.0 / f64::from(total_iter_dist));
        } else if total_iter_cnt >= 3 {
            score = Score::Numeric(10.0 / f64::from(total_iter_dist));
        } else if total_iter_cnt >= 2 {
            score = Score::Numeric(4.0 / f64::from(total_iter_dist));
        } else {
            score = Score::Numeric(f64::from(total_iter_cnt) / f64::from(total_iter_dist));
        }

        if let Some(l_cl) = l_closing {
            let l_gap = l_item.get_distance(l_cl).abs();
            if l_gap <= 2 {
                score = score * (1.0 - (1.0 / (1.0 + f64::from(l_gap))));
            }
        }

        if let Some(r_cl) = r_closing {
            let r_gap = r_item.get_distance(r_cl).abs();
            if r_gap <= 2 {
                score = score * (1.0 - (1.0 / (1.0 + f64::from(r_gap))));
            }
        }

        if mv.is_mine() {
            score
        } else {
            score * -2.5
        }
    }
}

impl Game for FiveInRow {
    type Move = FiveInRowMove;

    fn get_score(&self) -> Score {
        let score: Score = self.moves.iter().fold(Score::Numeric(0.0), |res, mv| {
            res + Direction::create_list_from_move(mv).iter().fold(
                Score::Numeric(0.0),
                |res, direction| {
                    let items = self
                        .moves
                        .iter()
                        .filter(|i| direction.is_in_direction(i.get_x(), i.get_y()))
                        .collect::<Vec<_>>();
                    res + FiveInRow::score_from_row(&mv, &items)
                },
            )
        });
        score
    }

    fn do_move(&mut self, new_move: Self::Move) -> Result<(), Error<FiveInRow>> {
        let existing_move = self
            .moves
            .iter()
            .find(|mv| mv.get_x() == new_move.get_x() && mv.get_y() == new_move.get_y());
        if let Some(_) = existing_move {
            return Err(Error::IncorrectMove(new_move));
        }
        self.moves.push(new_move);
        Ok(())
    }

    fn get_possible_moves(&self, myself: bool) -> Vec<FiveInRowMove> {
        let mut vec = Vec::new();
        if self.moves.len() == 0 {
            vec.push(FiveInRowMove::Mine(0, 0));
            return vec;
        }
        for x in -20..20 {
            for y in -20..20 {
                let m = self.moves.iter().find(|m| m.get_x() == x && m.get_y() == y);
                let maybe_move = match m {
                    None => {
                        if myself {
                            Some(FiveInRowMove::Mine(x, y))
                        } else {
                            Some(FiveInRowMove::Rivals(x, y))
                        }
                    }
                    Some(_) => None,
                };
                if let Some(mv) = maybe_move {
                    if mv.get_distance_from_moves(&self.moves) <= 5 {
                        vec.push(mv);
                    }
                }
            }
        }
        vec.iter()
            .to_owned()
            .filter_map(|mv| {
                if mv.get_distance_from_moves(&self.moves) <= 5 {
                    Some(mv.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_creates_empty_game() {
        let game = FiveInRow::create_empty();
        assert_eq!(game.moves.len(), 0);
        assert_eq!(game.get_score(), Score::Numeric(0.0));
    }

    #[test]
    fn it_does_new_move() {
        let mut game = FiveInRow::create_empty();
        assert_eq!(game.moves.len(), 0);
        assert!(game.do_move(FiveInRowMove::Mine(0, 0)).is_ok());
        assert_eq!(game.moves.len(), 1);
        assert!(game.do_move(FiveInRowMove::Rivals(0, 0)).is_err());
        assert_eq!(game.moves.len(), 1);
        assert!(game.do_move(FiveInRowMove::Rivals(0, 1)).is_ok());
        assert_eq!(game.moves.len(), 2);
    }

    #[test]
    fn it_creates_game_from_coordinates() {
        let mut coords = Vec::<Coordinate>::new();
        coords.push(Coordinate {
            playerId: String::from("pl1"),
            x: 0,
            y: 0,
        });
        coords.push(Coordinate {
            playerId: String::from("pl2"),
            x: 0,
            y: 1,
        });

        let game = FiveInRow::from_api_coordinates(coords, &String::from("pl1"));
        assert_eq!(game.moves.len(), 2);
        assert_eq!(*game.moves.get(0).unwrap(), FiveInRowMove::Mine(0, 0));
        assert_eq!(*game.moves.get(1).unwrap(), FiveInRowMove::Rivals(0, 1));
        //assert_eq!(game.get_score(), 0.0);
    }

    #[test]
    fn it_computes_score_for_row() {
        let mv = FiveInRowMove::Mine(0, 0);
        let mvr = FiveInRowMove::Rivals(0, 0);

        let score_x = FiveInRow::score_from_row(&mv, &Vec::from([&mv]));
        let score_xx =
            FiveInRow::score_from_row(&mv, &Vec::from([&mv, &FiveInRowMove::Mine(0, 1)]));
        let score_xxx = FiveInRow::score_from_row(
            &mv,
            &Vec::from([&mv, &FiveInRowMove::Mine(0, 1), &FiveInRowMove::Mine(0, 2)]),
        );
        let score_xxxx = FiveInRow::score_from_row(
            &mv,
            &Vec::from([
                &mv,
                &FiveInRowMove::Mine(0, 1),
                &FiveInRowMove::Mine(0, 2),
                &FiveInRowMove::Mine(0, 3),
            ]),
        );
        let score_xxxxx = FiveInRow::score_from_row(
            &mv,
            &Vec::from([
                &mv,
                &FiveInRowMove::Mine(0, 1),
                &FiveInRowMove::Mine(0, 2),
                &FiveInRowMove::Mine(0, 3),
                &FiveInRowMove::Mine(0, 4),
            ]),
        );
        let score_xxxox = FiveInRow::score_from_row(
            &mv,
            &Vec::from([
                &mv,
                &FiveInRowMove::Mine(0, 1),
                &FiveInRowMove::Mine(0, 2),
                &FiveInRowMove::Rivals(0, 3),
                &FiveInRowMove::Mine(0, 4),
            ]),
        );

        let score_xxxxex = FiveInRow::score_from_row(
            &mv,
            &Vec::from([
                &mv,
                &FiveInRowMove::Mine(0, 1),
                &FiveInRowMove::Mine(0, 2),
                &FiveInRowMove::Mine(0, 3),
                &FiveInRowMove::Mine(0, 6),
            ]),
        );

        let score_ooooo = FiveInRow::score_from_row(
            &mvr,
            &Vec::from([
                &mvr,
                &FiveInRowMove::Rivals(0, 1),
                &FiveInRowMove::Rivals(0, 2),
                &FiveInRowMove::Rivals(0, 3),
                &FiveInRowMove::Rivals(0, 4),
            ]),
        );

        assert!(score_x < score_xx);
        assert!(score_xx < score_xxx);
        assert!(score_xxx < score_xxxx);
        assert!(score_xxxx < score_xxxxx);
        assert!(score_xxxx < score_xxxxex);
        assert_ne!(score_xxxxex, Score::Win);

        assert_eq!(score_xxxxx, Score::Win);
        assert_eq!(score_ooooo, Score::Loss);

        assert!(score_xxx > score_xxxox);
    }

    #[test]
    fn it_replays_last_game() {
        let moves = Vec::from([
            FiveInRowMove::Mine(0, 0),
            FiveInRowMove::Rivals(0, 1),
            FiveInRowMove::Mine(0, -1),
            FiveInRowMove::Rivals(0, 2),
            FiveInRowMove::Mine(0, 4),
            FiveInRowMove::Rivals(-1, 2),
            FiveInRowMove::Mine(0, -2),
            FiveInRowMove::Rivals(0, -3),
            FiveInRowMove::Mine(0, 5),
            FiveInRowMove::Rivals(1, 2),
            FiveInRowMove::Mine(0, 6),
            FiveInRowMove::Rivals(2, 2),
            FiveInRowMove::Mine(0, 7),
        ]);
        let game = FiveInRow::from_moves(moves);
        let score = game.get_score();
        assert!(score < Score::Win);
        assert!(score > Score::Loss);
        assert!(score < Score::Numeric(0.0));
    }
}
