use crate::api::jobs_cz::fetch;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct StatusPayload {
    pub userToken: String,
    pub gameToken: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Coordinate {
    pub playerId: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct StatusResponse {
    pub statusCode: u16,
    pub playerCrossId: Option<String>,
    pub playerCircleId: Option<String>,
    pub actualPlayerId: Option<String>,
    pub winnerId: Option<String>,
    pub coordinates: Vec<Coordinate>,
}

pub async fn fetch_status(
    client: &mut fetch::JobsApi,
    payload: &StatusPayload,
) -> Result<StatusResponse, fetch::Error> {
    let res: StatusResponse = client
        .post_data("https://piskvorky.jobs.cz/api/v1/checkStatus", payload)
        .await?;
    Ok(res)
}

pub async fn fetch_last_status(
    client: &mut fetch::JobsApi,
    payload: &StatusPayload,
) -> Result<StatusResponse, fetch::Error> {
    let res: StatusResponse = client
        .post_data("https://piskvorky.jobs.cz/api/v1/checkLastStatus", payload)
        .await?;
    Ok(res)
}

pub async fn wait_my_turn(
    client: &mut fetch::JobsApi,
    player_id: &str,
    payload: &StatusPayload,
) -> Result<StatusResponse, fetch::Error> {
    let time = Instant::now();
    let mut reported = false;
    loop {
        let last_status = fetch_last_status(client, payload).await?;
        let maybe_current_id = last_status.actualPlayerId.clone();
        let maybe_cross_id = last_status.playerCrossId.clone();
        let maybe_circle_id = last_status.playerCircleId.clone();
        let maybe_winner_id = last_status.winnerId.clone();
        if let (Some(current_player_id), Some(_), Some(_)) =
            (maybe_current_id, maybe_cross_id, maybe_circle_id)
        {
            if current_player_id.eq(player_id) {
                return Ok(last_status);
            } else {
                if !reported {
                    reported = true;
                    println!("Waiting for rival's move...");
                }
                if time.elapsed().as_secs() > 300 {
                    return Err(fetch::Error::RivalTimeoutError);
                }
            }
        } else {
            if !reported {
                reported = true;
                println!("Waiting for rival to connect...");
            }
            if time.elapsed().as_secs() > 900 {
                return Err(fetch::Error::RivalTimeoutError);
            }
        }
        if maybe_winner_id.is_some() {
            return Ok(last_status);
        }
        if last_status.statusCode == 226 {
            return Err(fetch::Error::FinishedUnexpectedly);
        }
        if last_status.statusCode >= 400 {
            return Err(fetch::Error::Invalid);
        }
    }
}
