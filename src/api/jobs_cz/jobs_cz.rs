#[path = "connect.rs"]
pub mod connect;
#[path = "fetch.rs"]
pub mod fetch;
#[path = "play.rs"]
pub mod play;
#[path = "player.rs"]
pub mod player;
#[path = "register.rs"]
pub mod register;
#[path = "status.rs"]
pub mod status;

use crate::api::game_connection::GameConnection;
use crate::five_in_a_row::{mv::FiveInRowMove, FiveInRow};
use crate::game::error::Error;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct JobsApi {
    user_id: String,
    user_token: String,
    client: fetch::JobsApi,
    game_token: Option<String>,
}

impl JobsApi {
    pub fn new(user_id: &str, user_token: &str) -> Self {
        Self {
            client: fetch::JobsApi::new(reqwest::Client::new()),
            user_id: String::from(user_id),
            user_token: String::from(user_token),
            game_token: None,
        }
    }
}

#[async_trait]
impl GameConnection<FiveInRow> for JobsApi {
    async fn start_game(&mut self) -> Result<FiveInRow, Error<FiveInRow>> {
        let con_data = connect::invoke_connection(
            &mut self.client,
            &connect::ConnectPayload {
                userToken: String::from(&self.user_token),
            },
        )
        .await?;
        self.game_token = Some(String::from(con_data.gameToken));
        let game_token = self.game_token.as_ref().ok_or(Error::ApiInvalidData)?;

        println!("Connected, game token: {}", game_token);
        let status_payload = status::StatusPayload {
            gameToken: game_token.clone(),
            userToken: String::from(&self.user_token),
        };

        let stat_data = status::fetch_status(&mut self.client, &status_payload).await?;
        let game = FiveInRow::from_api_coordinates(stat_data.coordinates, &self.user_id);

        Ok(game)
    }

    async fn put_move(&mut self, mv: &FiveInRowMove) -> Result<(), Error<FiveInRow>> {
        let game_token = self.game_token.as_ref().ok_or(Error::ApiInvalidData)?;
        play::invoke_move(
            &mut self.client,
            &play::PlayPayload {
                userToken: String::from(&self.user_token),
                gameToken: String::from(game_token),
                positionX: mv.get_x(),
                positionY: mv.get_y(),
            },
        )
        .await?;
        Ok(())
    }

    async fn await_move(
        &mut self,
    ) -> Result<(Option<FiveInRowMove>, Option<String>), Error<FiveInRow>> {
        let game_token = self.game_token.as_ref().ok_or(Error::ApiInvalidData)?;
        let status_payload = status::StatusPayload {
            gameToken: game_token.clone(),
            userToken: String::from(&self.user_token),
        };
        let stat_data =
            status::wait_my_turn(&mut self.client, &self.user_id, &status_payload).await?;
        let rivals_move = stat_data.coordinates.get(0).map_or(None, |coord| {
            Some(FiveInRowMove::from_api_coordinates(&self.user_id, coord))
        });
        let winner_id = stat_data.winnerId;
        Ok((rivals_move, winner_id))
    }
}
