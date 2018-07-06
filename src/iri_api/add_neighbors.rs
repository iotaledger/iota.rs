use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Add a list of neighbors to your node. It should be noted that
/// this is only temporary, and the added neighbors will be removed
/// from your set of neighbors after you relaunch IRI.
pub fn add_neighbors(uri: &str, uris: &[String]) -> Result<AddNeighborsResponse> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "addNeighbors",
        "uris": uris,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

/// This is a typed representation of the JSON response
#[derive(Copy, Clone, Deserialize, Debug)]
pub struct AddNeighborsResponse {
    #[serde(rename = "addedNeighbors")]
    added_neighbors: usize,
}
