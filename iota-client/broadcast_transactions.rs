use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::*;

/// Broadcast a list of transactions to all neighbors.
/// The input trytes for this call are provided by attachToTangle.
pub fn broadcast_transactions(
    client: &Client,
    uri: String,
    trytes: Vec<String>,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "broadcastTransactions",
        "trytes": trytes,
    });

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
