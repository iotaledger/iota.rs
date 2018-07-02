use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn remove_neighbors(uri: &str, uris: &[String]) -> Result<RemoveNeighborsResponse, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "removeNeighbors",
        "uris": uris,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

#[derive(Deserialize, Debug)]
pub struct RemoveNeighborsResponse {
    #[serde(rename = "removedNeighbors")]
    removed_neighbors: usize,
}
