use crate::utils::input_validator;
use crate::Result;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;
use serde_json::Value;

/// Checks for consistency of given hashes, not part of the public api
pub fn check_consistency(client: &Client, uri: &str, hashes: &[String]) -> Result<Value> {
    for hash in hashes {
        ensure!(
            input_validator::is_hash(hash),
            "Provided hash is not valid: {:?}",
            hash
        );
    }

    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "checkConsistency",
        "tails": hashes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
