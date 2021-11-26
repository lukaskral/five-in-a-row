use crate::game::Game;

pub struct GamePlay<G: Game> {
    pub game: G,
}

impl<G: Game> GamePlay<G> {
    pub fn suggest_move(&self, myself: bool) -> Result<G::Move, G::Error> {
        let possibilities: Vec<(G::Move, f64)> = self
            .game
            .get_possible_moves()
            .iter()
            .filter_map(|mv| {
                let mut game_test = self.game.clone();
                let r = game_test.do_move(*mv);
                match r {
                    Ok(_) => Some((mv.to_owned(), game_test.get_score())),
                    Err(_) => None,
                }
            })
            .collect::<Vec<_>>();
        let max_score = possibilities.iter().fold(0.0, |max, pos| {
            let (_, score) = pos;
            if myself {
                f64::max(max, *score)
            } else {
                f64::min(max, *score)
            }
        });
        let p = possibilities
            .iter()
            .filter(|p| {
                let (_, score) = p;
                if myself {
                    *score >= max_score - 0.5
                } else {
                    *score <= max_score + 0.5
                }
            })
            .map(|p| p.to_owned())
            .collect::<Vec<_>>();
        if let Some(item) = p.get(p.len() / 2) {
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
