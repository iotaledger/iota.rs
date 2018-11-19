use crate::utils::input_validator;
use crate::Result;
use reqwest::Client;
use serde_json::Value;

/// Checks for consistency of given hashes, not part of the public api
pub async fn check_consistency(client: Client, uri: String, hashes: Vec<String>) -> Result<Value> {
    for hash in &hashes {
        ensure!(
            input_validator::is_hash(hash),
            "Provided hash is not valid: {:?}",
            hash
        );
    }

    let body = json!({
        "command": "checkConsistency",
        "tails": hashes,
    });

    Ok(client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?)
}
