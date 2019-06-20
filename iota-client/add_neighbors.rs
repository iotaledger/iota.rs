use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::*;

/// Add a list of neighbors to your node. It should be noted that
/// this is only temporary, and the added neighbors will be removed
/// from your set of neighbors after you relaunch IRI.
pub fn add_neighbors(
    client: &Client,
    uri: String,
    uris: Vec<String>,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "addNeighbors",
        "uris": uris,
    });

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
