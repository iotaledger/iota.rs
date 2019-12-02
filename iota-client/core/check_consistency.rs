use reqwest::{Client, Error, Response};

/// Checks for consistency of given hashes, not part of the public api
pub(crate) async fn check_consistency(
    client: &Client,
    uri: &str,
    hashes: &[String],
) -> Result<Response, Error> {
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
        .await
}
