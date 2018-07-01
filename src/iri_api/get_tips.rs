use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn get_tips(uri: &str) -> Result<GetTipsResponse, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTips",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct GetTipsResponse {
    duration: i64,
    hashes: Vec<String>,
}
