use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Removes a list of neighbors to your node.
/// This is only temporary, and if you have your neighbors
/// added via the command line, they will be retained after
/// you restart your node.
pub fn remove_neighbors(
    client: &Client,
    uri: &str,
    uris: &[String],
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "removeNeighbors",
        "uris": uris,
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
