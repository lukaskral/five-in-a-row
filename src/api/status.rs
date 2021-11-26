use crate::api::fetch;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusPayload {
    pub userToken: String,
    pub gameToken: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub playerId: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub statusCode: u16,
    pub playerCrossId: Option<String>,
    pub playerCircleId: Option<String>,
    pub actualPlayerId: Option<String>,
    pub winnerId: Option<String>,
    pub coordinates: Vec<Coordinate>,
}

pub async fn fetch_status(
    client: &reqwest::Client,
    payload: &StatusPayload,
) -> Result<StatusResponse, fetch::Error> {
    let res: StatusResponse = fetch::post_data(
        client,
        "https://piskvorky.jobs.cz/api/v1/checkStatus",
        payload,
    )
    .await?;
    Ok(res)
}

pub async fn fetch_last_status(
    client: &reqwest::Client,
    payload: &StatusPayload,
) -> Result<StatusResponse, fetch::Error> {
    let res: StatusResponse = fetch::post_data(
        client,
        "https://piskvorky.jobs.cz/api/v1/checkLastStatus",
        payload,
    )
    .await?;
    Ok(res)
}

pub async fn wait_my_turn(
    client: &reqwest::Client,
    player_id: &str,
    payload: &StatusPayload,
) -> Result<StatusResponse, fetch::Error> {
    loop {
        let last_status = fetch_last_status(client, payload).await?;
        let maybe_current_id = last_status.actualPlayerId.clone();
        let maybe_cross_id = last_status.playerCrossId.clone();
        let maybe_circle_id = last_status.playerCircleId.clone();
        if let (Some(current_player_id), Some(_), Some(_)) =
            (maybe_current_id, maybe_cross_id, maybe_circle_id)
        {
            if current_player_id.eq(player_id) {
                return Ok(last_status);
            } else {
                println!("Waiting for rival's move");
            }
        } else {
            println!("Waiting for rival to connect");
        }
    }
}
