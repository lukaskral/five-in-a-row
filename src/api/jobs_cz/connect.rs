use crate::api::jobs_cz::fetch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ConnectPayload {
    pub userToken: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ConnectResponse {
    pub statusCode: u16,
    pub gameToken: String,
    pub gameId: String,
}

pub async fn invoke_connection(
    client: &mut fetch::JobsApi,
    payload: &ConnectPayload,
) -> Result<ConnectResponse, fetch::Error> {
    let res: ConnectResponse = client
        .post_data("https://piskvorky.jobs.cz/api/v1/connect", payload)
        .await?;
    Ok(res)
}
