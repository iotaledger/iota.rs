use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Checks for consistency of given hashes, not part of the public api
pub fn check_consistency(
    client: &Client,
    uri: &str,
    hashes: &[String],
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "checkConsistency",
        "tails": hashes,
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
