#[path = "api/api.rs"]
mod api;
#[path = "five_in_row/five_in_row.rs"]
mod five_in_row;
#[path = "game/game.rs"]
mod game;
#[path = "game/gameplay.rs"]
mod gameplay;

use std::boxed::Box;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_id = String::from("***");
    let user_token = String::from("***");

    let mut gameplays = 0;
    let mut wins = 0;
    let mut losses = 0;
    let mut errors = 0;
    let api = api::jobs_cz::JobsApi::new(&user_id, &user_token);

    loop {
        gameplays = gameplays + 1;

        // create a new game
        let mut maybe_game_play = gameplay::GamePlay::from_api(api.clone()).await;
        if let Ok(game_play) = maybe_game_play.as_mut() {
            // start to play and wait for the winner id
            let maybe_winner = game_play.play().await;

            if let Ok(winner) = maybe_winner {
                if winner.eq(&user_id) {
                    wins = wins + 1;
                    println!("I won the game ✌🥇");
                } else {
                    losses = losses + 1;
                    println!("I lost the game 😢");
                }
            } else {
                println!("No winner");
                errors = errors + 1;
            }
        } else {
            println!("Error in the game 😢");
            errors = errors + 1;
        }

        println!("==========================\n\n");
        println!("\t- wins: {}", wins);
        println!("\t- losses: {}", losses);
        println!("\t- errors: {}", errors);
        println!("\t- total games: {}", gameplays);
        println!("==========================\n\n");
    }
}
