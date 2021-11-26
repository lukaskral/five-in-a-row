use crate::api::fetch;
use crate::api::status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayPayload {
    pub userToken: String,
    pub gameToken: String,
    pub positionX: i32,
    pub positionY: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    playerId: String,
    x: i32,
    y: i32,
}

pub async fn invoke_move(
    client: &reqwest::Client,
    payload: &PlayPayload,
) -> Result<status::StatusResponse, fetch::Error> {
    let res: status::StatusResponse =
        fetch::post_data(client, "https://piskvorky.jobs.cz/api/v1/play", payload).await?;
    Ok(res)
}

/*

pub async fn invoke_move(
    client: &reqwest::Client,
    payload: &PlayPayload,
) -> Result<status::StatusResponse, fetch::Error> {
    sleep(Duration::from_millis(1500));
    let body = json!(payload);
    let response_text = client
        .post("https://piskvorky.jobs.cz/api/v1/play")
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;
    println!("\t- play: {:?}", response_text);
    let res: status::StatusResponse = serde_json::from_str(&response_text)?;
    Ok(res)
}
*/
