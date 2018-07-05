use crate::utils::input_validator;
use failure::Error;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;
use serde_json::Value;

pub fn broadcast_transactions(uri: &str, trytes: &[String]) -> Result<Value, Error> {
    ensure!(
        input_validator::is_array_of_attached_trytes(trytes),
        "Provided trytes are not valid: {:?}",
        trytes
    );

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "broadcastTransactions",
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
