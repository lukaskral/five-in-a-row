use crate::api::fetch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPayload {
    pub nickname: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub statusCode: u16,
    pub userId: String,
    pub userToken: String,
}

pub async fn invoke_registration(
    client: &reqwest::Client,
    payload: &RegisterPayload,
) -> Result<RegisterResponse, fetch::Error> {
    let res: RegisterResponse =
        fetch::post_data(client, "https://piskvorky.jobs.cz/api/v1/user", payload).await?;
    Ok(res)
}
