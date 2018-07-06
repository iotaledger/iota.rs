use crate::utils::input_validator;
use failure::Error;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;

pub fn broadcast_transactions(
    uri: &str,
    trytes: &[String],
) -> Result<BroadcastTransactionsResponse, Error> {
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

#[derive(Deserialize, Debug)]
pub struct BroadcastTransactionsResponse {
    duration: i64,
    error: Option<String>,
    exception: Option<String>,
}

impl BroadcastTransactionsResponse {
    pub fn duration(&self) -> i64 {
        self.duration
    }
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    pub fn exception(&self) -> Option<String> {
        self.exception.clone()
    }
}
