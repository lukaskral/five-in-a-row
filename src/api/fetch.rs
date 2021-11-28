use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use std::convert::TryFrom;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub enum Error {
    ApiErr(reqwest::Error),
    JsonErr(serde_json::Error),
    TimeError(std::num::TryFromIntError),
    RivalTimeoutError,
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
impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Self::TimeError(e)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiErr(_) => write!(f, "Api Error"),
            Self::JsonErr(_) => write!(f, "Json Error"),
            Self::TimeError(_) => write!(f, "Time Error"),
            Self::RivalTimeoutError => write!(f, "Rival Time Out Error"),
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::ApiErr(err) => Some(err),
            Self::JsonErr(err) => Some(err),
            Self::TimeError(err) => Some(err),
            Self::RivalTimeoutError => None,
        }
    }
}

pub struct JobsApi {
    client: reqwest::Client,
    time: Instant,
    last_call: i64,
}

impl JobsApi {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client: client,
            time: Instant::now(),
            last_call: 0,
        }
    }

    pub async fn post_data<P: Serialize + std::fmt::Debug, R: DeserializeOwned>(
        &mut self,
        url: &str,
        payload: P,
    ) -> Result<R, Error> {
        let body = json!(payload);
        let remaining: i64 =
            1000 - (i64::try_from(self.time.elapsed().as_millis())? - self.last_call);
        if remaining > 0 {
            sleep(Duration::from_millis(u64::try_from(remaining + 100)?));
        }
        //println!("\n\n====================\nRequest: {}\n{:?}", url, payload);
        loop {
            let maybe_response = self.client.post(url).body(body.to_string()).send().await;

            if let Ok(response) = maybe_response {
                if response.status() == 429 {
                    // println!("...waiting");
                    sleep(Duration::from_millis(1100));
                    continue;
                } else {
                    let response_text = response.text().await?;
                    // println!("====================\nResponse:\n{}", response_text);
                    let res: R = serde_json::from_str(&response_text)?;
                    self.last_call = i64::try_from(self.time.elapsed().as_millis())?;
                    return Ok(res);
                }
            }
        }
    }
}
