use crate::api::jobs_cz::fetch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RegisterPayload {
    pub nickname: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RegisterResponse {
    pub statusCode: u16,
    pub userId: String,
    pub userToken: String,
}

#[allow(dead_code)]
pub async fn invoke_registration(
    client: &mut fetch::JobsApi,
    payload: &RegisterPayload,
) -> Result<RegisterResponse, fetch::Error> {
    let res: RegisterResponse = client
        .post_data("https://piskvorky.jobs.cz/api/v1/user", payload)
        .await?;
    Ok(res)
}
