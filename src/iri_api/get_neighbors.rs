use crate::model::Neighbor;
use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn get_neighbors(uri: &str) -> Result<GetNeighborsResponse, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNeighbors",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct GetNeighborsResponse {
    duration: i64,
    error: Option<String>,
    neighbors: Option<Vec<Neighbor>>,
}
