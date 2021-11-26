#[path = "api.rs"]
mod api;
#[path = "five_in_row.rs"]
mod five_in_row;
#[path = "game.rs"]
mod game;
#[path = "gameplay.rs"]
mod gameplay;

use five_in_row::{mv::FiveInRowMove, FiveInRow};
use gameplay::GamePlay;
use std::boxed::Box;
use std::error::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_id = String::from("***");
    let user_token = String::from("***");

    let status_payload = api::status::StatusPayload {
        gameToken: game_token.clone(),
        userToken: user_token.clone(),
    };
    println!(
        "Game status: {:?} ({} s)",
        status_payload,
        now.elapsed().as_secs()
    );

    let stat_data = api::status::fetch_status(&client, &status_payload).await?;
    let game = FiveInRow::from_api_coordinates(stat_data.coordinates, &user_id);
    let mut game_play = GamePlay { game: game };

    println!("New game 🃏: {:?}", game_token);

    let winner = loop {
        let stat_data = api::status::wait_my_turn(&client, &user_id, &status_payload).await?;
        if let Some(winner_id) = stat_data.winnerId {
            break Ok::<String, Box<dyn Error>>(winner_id);
        }

        let maybe_cross_id = stat_data.playerCrossId.clone();
        let (my_symbol, rivals_symbol) = if let Some(cross_id) = maybe_cross_id {
            if cross_id.eq(&user_id) {
                ("❌", "⭕")
            } else {
                ("⭕", "❌")
            }
        } else {
            ("💀", "💻")
        };

        let maybe_coord = stat_data.coordinates.get(0);
        if let Some(coord) = maybe_coord {
            let rivals_move = FiveInRowMove::from_api_coordinates(&user_id, coord);
            game_play.add_move(rivals_move)?;
            println!(
                "Rival's move {}: {:?} ({} s)",
                rivals_symbol,
                rivals_move,
                now.elapsed().as_secs()
            );
        }

        let my_move = game_play.suggest_move(true)?;
        println!(
            "My move {}: {:?} ({} s)",
            my_symbol,
            my_move,
            now.elapsed().as_secs()
        );
        api::play::invoke_move(
            &client,
            &api::play::PlayPayload {
                userToken: user_token.clone(),
                gameToken: game_token.clone(),
                positionX: my_move.get_x(),
                positionY: my_move.get_y(),
            },
        )
        .await?;
        game_play.add_move(my_move)?;
    }?;
    println!("\r\n==========================");
    if winner.eq(&user_id) {
        println!("I won the game ✌🥇");
    } else {
        println!("I lost the game 😢");
    }
    println!("Total game time {} s", now.elapsed().as_secs());
    Ok(())
}
