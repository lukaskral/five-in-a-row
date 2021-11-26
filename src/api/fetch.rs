use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub enum Error {
    ApiErr(reqwest::Error),
    JsonErr(serde_json::Error),
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ApiErr(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::JsonErr(e)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiErr(_) => write!(f, "Api Error"),
            Self::JsonErr(_) => write!(f, "Json Error"),
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::ApiErr(err) => Some(err),
            Self::JsonErr(err) => Some(err),
        }
    }
}

pub async fn post_data<'de, P: Serialize, R: Deserialize<'de>>(
    client: &reqwest::Client,
    url: &str,
    payload: P,
) -> Result<R, Error> {
    let body = json!(payload);
    loop {
        let maybe_response = client.post(url).body(body.to_string()).send().await;

        if let Ok(response) = maybe_response {
            if response.status() == 429 {
                sleep(Duration::from_millis(1100));
                continue;
            } else {
                let response_text = response.text().await?;
                let res: R = serde_json::from_str(&response_text)?;
                return Ok(res);
            }
        }
    }
}
