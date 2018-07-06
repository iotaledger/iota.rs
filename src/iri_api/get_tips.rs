use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Returns the list of tups
pub fn get_tips(uri: &str) -> Result<GetTipsResponse> {
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

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct GetTipsResponse {
    duration: i64,
    hashes: Vec<String>,
}

impl GetTipsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the hashes attribute
    pub fn hashes(&self) -> &[String] {
        &self.hashes
    }
    /// Takes ownership the hashes attribute
    pub fn take_hashes(self) -> Vec<String> {
        self.hashes
    }
}
