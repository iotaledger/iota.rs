use failure::Error;
use reqwest::header::{ContentType, Headers};
use serde_json::Value;

pub fn were_addresses_spent_from(uri: &str, addresses: &[String]) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "wereAddressesSpentFrom",
        "addresses": addresses,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
