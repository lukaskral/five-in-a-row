#[path = "five_in_row/direction.rs"]
mod dir;

#[path = "five_in_row/move.rs"]
pub mod mv;

use crate::api::status::Coordinate;
use crate::five_in_row::dir::Direction;
use crate::five_in_row::mv::FiveInRowMove;
use crate::game::Game;
use std::error::Error;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::vec::Vec;

#[derive(Debug)]
pub enum FiveInRowError {
    Error,
}
impl Display for FiveInRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "unhandled error!"),
        }
    }
}
impl StdError for FiveInRowError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

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

    fn score_from_item_cnt(item_cnt: f64, items_in_range: i32, range: Option<i32>) -> f64 {
        if let Some(range_size) = range {
            if range_size < 5 {
                return 0.0;
            }
            return f64::from(item_cnt.powf(2.0))
                + (f64::from(items_in_range) / f64::from(range_size));
        }
        f64::from(item_cnt.powf(2.0))
    }

    fn score_from_row(vec: &Vec<&FiveInRowMove>) -> f64 {
        let mut moves = vec.clone();
        moves.sort();
        let mut score: f64 = 0.0;
        let mut total_iter_cnt = 0;
        let mut iter_cnt = 0.0;
        let mut last_item: Option<&FiveInRowMove> = None;
        let mut last_other_item: Option<&FiveInRowMove> = None;
        let mut mul = 1.0;
        for item in moves {
            if last_item == None || item.is_same_type(last_item) {
                total_iter_cnt = total_iter_cnt + 1;
                iter_cnt = match last_item {
                    None => 1.0,
                    Some(mv) => {
                        let distance = mv.get_distance(item).abs();
                        //println!("\ndistance from last one {}", mv.get_distance(item));
                        if distance <= 1 {
                            iter_cnt + 1.0
                        } else if distance <= 3 {
                            iter_cnt + 0.5
                        } else if distance <= 5 {
                            iter_cnt
                        } else {
                            1.0
                        }
                    }
                };
            } else {
                let range = match last_other_item {
                    None => None,
                    Some(mv) => Some(item.get_distance(mv)),
                };
                score = score + mul * Self::score_from_item_cnt(iter_cnt, total_iter_cnt, range);
                iter_cnt = 1.0;
                total_iter_cnt = 1;
                last_other_item = Some(item);
            }
            /*println!(
                "score: {}, cnt: {}, totl_cnt: {}",
                score, iter_cnt, total_iter_cnt
            );*/
            last_item = Some(item);
            mul = match item {
                FiveInRowMove::Mine(_, _) => 1.0,
                FiveInRowMove::Rivals(_, _) => -1.0,
            };
        }
        score = score + mul * Self::score_from_item_cnt(iter_cnt, total_iter_cnt, None);
        /*println!(
            "score: {}, cnt: {}, totl_cnt: {}",
            score, iter_cnt, total_iter_cnt
        );*/
        score
    }
}

impl Game for FiveInRow {
    type Move = FiveInRowMove;
    type Error = FiveInRowError;

    fn get_score(&self) -> f64 {
        let score: f64 = self.moves.iter().fold(0.0, |res, mv| {
            res + Direction::create_list_from_move(mv)
                .iter()
                .fold(0.0, |res, direction| {
                    let items = self
                        .moves
                        .iter()
                        .filter(|i| direction.is_in_direction(i.get_x(), i.get_y()))
                        .collect::<Vec<_>>();
                    res + FiveInRow::score_from_row(&items)
                })
        });
        score
    }

    fn do_move(&mut self, new_move: Self::Move) -> Result<(), Self::Error> {
        let existing_move = self
            .moves
            .iter()
            .find(|mv| mv.get_x() == new_move.get_x() && mv.get_y() == new_move.get_y());
        if let Some(_) = existing_move {
            return Err(self.get_error(None));
        }
        self.moves.push(new_move);
        Ok(())
    }

    fn get_possible_moves(&self) -> Vec<FiveInRowMove> {
        let mut vec = Vec::new();
        if self.moves.len() == 0 {
            vec.push(FiveInRowMove::Mine(0, 0));
            return vec;
        }
        for x in -20..20 {
            for y in -20..20 {
                let m = self.moves.iter().find(|m| m.get_x() == x && m.get_y() == y);
                match m {
                    None => {
                        vec.push(FiveInRowMove::Mine(x, y));
                    }
                    Some(_) => {}
                }
            }
        }
        vec
    }

    fn get_error(&self, _source: Option<Box<dyn Error>>) -> Self::Error {
        FiveInRowError::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_creates_empty_game() {
        let game = FiveInRow::create_empty();
        assert_eq!(game.moves.len(), 0);
        assert_eq!(game.get_score(), 0.0);
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
        assert_eq!(game.get_score(), 0.0);
    }

    #[test]
    fn it_computes_score_for_row() {
        assert_eq!(FiveInRow::score_from_row(&Vec::from([])), 0.0);
        assert_eq!(
            FiveInRow::score_from_row(&Vec::from([
                &FiveInRowMove::Mine(0, 0),
                &FiveInRowMove::Rivals(0, 1),
            ])),
            0.0
        );
        assert_eq!(
            FiveInRow::score_from_row(&Vec::from([
                &FiveInRowMove::Mine(0, 0),
                &FiveInRowMove::Mine(0, 1),
                &FiveInRowMove::Rivals(0, 2),
                &FiveInRowMove::Rivals(0, 3),
            ])),
            0.0
        );
        assert_eq!(
            FiveInRow::score_from_row(&Vec::from([
                &FiveInRowMove::Mine(0, 0),
                &FiveInRowMove::Rivals(0, 2),
                &FiveInRowMove::Rivals(0, 3),
            ])),
            -3.0
        );
        assert_eq!(
            FiveInRow::score_from_row(&Vec::from([
                &FiveInRowMove::Mine(0, 0),
                &FiveInRowMove::Mine(0, 3),
                &FiveInRowMove::Mine(0, 5),
                &FiveInRowMove::Rivals(0, 7),
                &FiveInRowMove::Rivals(0, 9),
            ])),
            1.75
        );
        assert_eq!(
            FiveInRow::score_from_row(&Vec::from([
                &FiveInRowMove::Mine(0, 0),
                &FiveInRowMove::Rivals(0, 1),
                &FiveInRowMove::Rivals(0, 2),
                &FiveInRowMove::Rivals(0, 3),
                &FiveInRowMove::Rivals(0, 4),
                &FiveInRowMove::Rivals(0, 5),
                &FiveInRowMove::Mine(0, 6),
            ])),
            -24.0
        );
        assert_eq!(
            FiveInRow::score_from_row(&Vec::from([
                &FiveInRowMove::Rivals(0, 2),
                &FiveInRowMove::Rivals(0, 3),
                &FiveInRowMove::Rivals(0, 4),
                &FiveInRowMove::Mine(0, 5),
                &FiveInRowMove::Mine(0, 6),
                &FiveInRowMove::Mine(0, 7),
                &FiveInRowMove::Mine(0, 8),
            ])),
            7.0
        );
    }
}
