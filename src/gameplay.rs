use crate::game::{score::Score, Game};
use rand::Rng;

pub struct GamePlay<G: Game> {
    pub game: G,
}

impl<G: Game> GamePlay<G> {
    pub fn suggest_move(&self, myself: bool) -> Result<G::Move, G::Error> {
        let possibilities: Vec<(G::Move, Score)> = self
            .game
            .get_possible_moves(myself)
            .iter()
            .filter_map(|mv| {
                let mut game_test = self.game.clone();
                let r = game_test.do_move(*mv);
                match r {
                    Ok(_) => {
                        let score = game_test.get_score();
                        Some((mv.to_owned(), score))
                    }
                    Err(_) => None,
                }
            })
            .collect::<Vec<_>>();
        let max_score = possibilities.iter().fold(Score::Loss, |max, pos| {
            let (_, score) = pos;
            if myself {
                Score::max(max, *score)
            } else {
                Score::min(max, *score)
            }
        }) - 0.2;
        let p = possibilities
            .iter()
            .filter(|p| {
                let (_, score) = p;
                if myself {
                    *score >= max_score
                } else {
                    *score <= max_score
                }
            })
            .map(|p| p.to_owned())
            .collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        let idx = (rng.gen::<f64>() * f64::from(p.len() as i32))
            .floor()
            .rem_euclid(2f64.powi(32)) as usize;
        println!(
            "Found best move ({} options, getting opt. n. {})",
            p.len(),
            idx
        );
        if let Some(item) = p.get(idx) {
            let (mv, _) = *item;
            Ok(mv)
        } else {
            Err(self.game.get_error(None))
        }
    }

    pub fn add_move(&mut self, mv: G::Move) -> Result<(), G::Error> {
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
        let game_play = GamePlay { game: game };
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(suggested, FiveInRowMove::Mine(0, 4));
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
        let game_play = GamePlay { game: game };
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(suggested, FiveInRowMove::Mine(-1, 0));
    }
}
