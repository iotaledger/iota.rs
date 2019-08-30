use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Returns the set of neighbors you are connected with, as
/// well as their activity count. The activity counter is reset
/// after restarting IRI.
pub(crate) fn get_neighbors(
    client: &Client,
    uri: &str,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "getNeighbors",
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
