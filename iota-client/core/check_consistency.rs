use reqwest::{Client, Error};
use serde_json::Value;

/// Checks for consistency of given hashes, not part of the public api
pub(crate) async fn check_consistency(
    client: &Client,
    uri: &str,
    hashes: &[String],
) -> Result<Value, Error> {
    let body = json!({
        "command": "checkConsistency",
        "tails": hashes,
    });

    client
        .post(uri)
        .header("Content-Type", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
        .await?
        .json()
        .await
}
