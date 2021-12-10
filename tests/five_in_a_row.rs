#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use game_play::{
        api::game_connection::GameConnection,
        five_in_a_row::{mv::FiveInRowMove, FiveInRow},
        game::{error::Error, Game},
        gameplay::GamePlay,
    };
    use std::collections::VecDeque;

    pub struct MockConnection {}
    #[async_trait]
    impl GameConnection<FiveInRow> for MockConnection {
        async fn start_game(&mut self) -> Result<FiveInRow, Error<FiveInRow>> {
            Err(Error::Invalid)
        }
        async fn put_move(&mut self, _: &FiveInRowMove) -> Result<(), Error<FiveInRow>> {
            Ok(())
        }
        async fn await_move(
            &mut self,
        ) -> Result<(Option<FiveInRowMove>, Option<String>), Error<FiveInRow>> {
            Err(Error::Invalid)
        }
    }

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
        let mut game_play = GamePlay::<FiveInRow, MockConnection>::from_game(game);
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
        let mut game_play = GamePlay::<FiveInRow, MockConnection>::from_game(game);
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
        let mut game_play = GamePlay::<FiveInRow, MockConnection>::from_game(game);
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
        let mut game_play = GamePlay::<FiveInRow, MockConnection>::from_game(game);
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
        let mut game_play = GamePlay::<FiveInRow, MockConnection>::from_game(game);
        game_play
            .compute_suggestions(true, VecDeque::new(), 3)
            .unwrap();
        let suggested = game_play.suggest_move(true).unwrap();
        assert_eq!(*suggested.get_move(), FiveInRowMove::Mine(-1, 1));
    }

    #[test]
    fn it_suggests_correct_move_6() {
        // https://piskvorky.jobs.cz/detail-hry/d8feaf9f-f272-4e33-8615-5832a4940a6f/
        let moves = Vec::from([
            FiveInRowMove::Mine(0, 0),
            FiveInRowMove::Rivals(1, -1),
            FiveInRowMove::Mine(0, -1),
            FiveInRowMove::Rivals(0, -2),
            FiveInRowMove::Mine(0, 1),
            FiveInRowMove::Rivals(0, 2),
            FiveInRowMove::Mine(1, 0),
            FiveInRowMove::Rivals(2, 0),
            FiveInRowMove::Mine(-1, -3),
            FiveInRowMove::Rivals(3, 1),
            FiveInRowMove::Mine(4, 2),
            FiveInRowMove::Rivals(1, 1),
            FiveInRowMove::Mine(3, -1),
            FiveInRowMove::Rivals(-2, 4),
            FiveInRowMove::Mine(-1, 3),
            FiveInRowMove::Rivals(2, -1),
            FiveInRowMove::Mine(-1, -2),
            FiveInRowMove::Rivals(2, 1),
        ]);
        let game = FiveInRow::from_moves(moves);
        game.visualize();
        let mut game_play = GamePlay::<FiveInRow, MockConnection>::from_game(game);
        game_play
            .compute_suggestions(true, VecDeque::new(), 2)
            .unwrap();
        let suggested = game_play.suggest_move(true).unwrap();
        let suggested_move = *suggested.get_move();
        assert!(
            suggested_move == FiveInRowMove::Mine(2, -2)
                || suggested_move == FiveInRowMove::Mine(2, 2)
                || suggested_move == FiveInRowMove::Mine(-2, -3)
                || suggested_move == FiveInRowMove::Mine(-3, -4),
            "Expected (2, -2), (2, 2), (-2, -3) or (-3, -4) got {:?}",
            suggested_move
        );
    }

    // #[test]
    fn it_suggests_correct_move_7() {
        // https://piskvorky.jobs.cz/detail-hry/9829163b-c578-4b0b-a334-baab9863c76f/
        let moves = Vec::from([
            FiveInRowMove::Mine(0, 0),
            FiveInRowMove::Rivals(-1, 0),
            FiveInRowMove::Mine(-4, 0),
            FiveInRowMove::Rivals(-2, 1),
            FiveInRowMove::Mine(0, -1),
            FiveInRowMove::Rivals(0, 1),
            FiveInRowMove::Mine(0, -2),
            FiveInRowMove::Rivals(-1, 2),
            FiveInRowMove::Mine(0, -3),
            FiveInRowMove::Rivals(0, -4),
            FiveInRowMove::Mine(-1, 1),
            FiveInRowMove::Rivals(-2, 3),
            FiveInRowMove::Mine(1, 0),
            FiveInRowMove::Rivals(-3, 2),
            FiveInRowMove::Mine(-2, 2),
            FiveInRowMove::Rivals(1, -1),
            FiveInRowMove::Mine(-3, 3),
            FiveInRowMove::Rivals(-4, 4),
            FiveInRowMove::Mine(-3, 4),
            FiveInRowMove::Rivals(-2, -1),
            FiveInRowMove::Mine(-3, -2),
            FiveInRowMove::Rivals(-3, 0),
            FiveInRowMove::Mine(-4, -1),
            FiveInRowMove::Rivals(-4, 1),
            FiveInRowMove::Mine(-1, -2),
            FiveInRowMove::Rivals(-5, 0),
            FiveInRowMove::Mine(-1, 4),
            FiveInRowMove::Rivals(-6, -1),
        ]);
        let game = FiveInRow::from_moves(moves);
        game.visualize();
        let mut game_play = GamePlay::<FiveInRow, MockConnection>::from_game(game);
        game_play
            .compute_suggestions(true, VecDeque::new(), 2)
            .unwrap();
        let suggested = game_play.suggest_move(true).unwrap();
        let suggested_move = *suggested.get_move();

        // TODO inspect game and find the place where the algorithm failed
        /*
        assert!(
            suggested_move == FiveInRowMove::Mine(2, -2)
                || suggested_move == FiveInRowMove::Mine(2, 2)
                || suggested_move == FiveInRowMove::Mine(-2, -3)
                || suggested_move == FiveInRowMove::Mine(-3, -4),
            "Expected (2, -2), (2, 2), (-2, -3) or (-3, -4) got {:?}",
            suggested_move
        );
        */
    }
}
