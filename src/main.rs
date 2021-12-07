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
    /*
    let reg_data = api::register::invoke_registration(
        &mut client,
        &api::register::RegisterPayload {
            nickname: String::from("🤖 https://github.com/lukaskral/five-in-a-row"),
            email: String::from("robot@lukaskral.eu"),
        },
    )
    .await?;
    // { statusCode: 201, userId: "d93f341a-6a35-4c13-9053-5457ea9c8c42", userToken: "790b2456-d8a9-42fe-a5d4-70b69a6a2b02" }
    println!("User: {:?}", reg_data);
    return Ok(());
    */

    //username 🤖
    //let user_id = String::from("d93f341a-6a35-4c13-9053-5457ea9c8c42");
    //let user_token = String::from("790b2456-d8a9-42fe-a5d4-70b69a6a2b02");

    //username 🤖 https://github.com/lukaskral/five-in-a-row
    let user_id = String::from("0c759a9a-402b-4407-9840-26bb080c17df");
    let user_token = String::from("42e71bcf-0f16-4389-bbc1-ffcf9099c135");

    let mut gameplays = 0;
    let mut wins = 0;
    let mut losses = 0;
    let mut errors = 0;
    let api = api::jobs_cz::JobsApi::new(&user_id, &user_token);

    loop {
        gameplays += 1;

        // create a new game
        let mut maybe_game_play = gameplay::GamePlay::from_api(api.clone()).await;
        if let Ok(game_play) = maybe_game_play.as_mut() {
            // start to play and wait for the winner id
            let maybe_winner = game_play.play().await;

            if let Ok(winner) = maybe_winner {
                if winner.eq(&user_id) {
                    wins += 1;
                    println!("I won the game ✌🥇");
                } else {
                    losses += 1;
                    println!("I lost the game 😢");
                }
            } else {
                println!("No winner");
                errors += 1;
            }
        } else {
            println!("Error in the game 😢");
            errors += 1;
        }

        println!("==========================\n\n");
        println!("\t- wins: {}", wins);
        println!("\t- losses: {}", losses);
        println!("\t- errors: {}", errors);
        println!("\t- total games: {}", gameplays);
        println!("==========================\n\n");
    }
}
