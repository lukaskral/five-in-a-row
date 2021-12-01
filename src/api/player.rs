use crate::api::fetch;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Payload {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Response {
    pub id: String,
    pub name: String,
    pub score: Option<i32>,
}

pub async fn fetch_player(payload: &Payload) -> Result<Response, fetch::Error> {
    let url = format!(
        "https://piskvorky.jobs.cz/detail-hrace/{}/",
        payload.user_id
    );
    let body = reqwest::get(url).await?.text().await?;

    let re_name = Regex::new(r"<h1>Player: ([^<]*)</h1>").unwrap();
    let name_capture = re_name
        .captures(&body)
        .map_or(Err(fetch::Error::ParseError), |name| Ok(name))?;

    let re_score = Regex::new(r#"(?m)<div[^>]*>\s*<div[^>]*>\s*Celkem bodů:\s*</div>\s*<div class="col-md-9">\s*(\d+)\s*</div>\s*</div>"#).unwrap();
    let maybe_score = re_score.captures(&body).map_or(None, |score_capture| {
        String::from(&score_capture[1])
            .parse::<i32>()
            .map_or(None, |score| Some(score))
    });

    Ok(Response {
        id: String::from(&payload.user_id),
        name: String::from(&name_capture[1]),
        score: maybe_score,
    })
}
