use crate::utils::input_validator;
use failure::Error;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;
use serde_json::Value;

pub fn check_consistency(uri: &str, hashes: &[String]) -> Result<Value, Error> {
    for hash in hashes {
        ensure!(
            input_validator::is_hash(hash),
            "Provided hash is not valid: {:?}",
            hash
        );
    }

    let client = Client::new();
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
