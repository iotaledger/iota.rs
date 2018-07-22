use super::responses::AddNeighborsResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;

/// Add a list of neighbors to your node. It should be noted that
/// this is only temporary, and the added neighbors will be removed
/// from your set of neighbors after you relaunch IRI.
pub async fn add_neighbors(client: &Client, uri: String, uris: Vec<String>) -> Result<AddNeighborsResponse> {
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "addNeighbors",
        "uris": uris,
    });

    Ok(client
        .post(&uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
