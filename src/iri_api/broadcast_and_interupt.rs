use crate::utils::input_validator;
use failure::Error;
use reqwest::header::{ContentType, Headers};
use reqwest::{self, Response};
use serde_json::Value;

pub fn interrupt_attaching_to_tangle(uri: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    Ok(client.post(uri).send()?)
}

pub fn broadcast_transactions(uri: &str, trytes: &[String]) -> Result<Value, Error> {
    assert!(input_validator::is_array_of_attached_trytes(trytes));

    let client = reqwest::Client::new();
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