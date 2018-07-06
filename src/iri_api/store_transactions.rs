use super::responses::StoreTransactionsResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Store transactions into the local storage.
/// The trytes to be used for this call are
/// returned by attachToTangle.
pub fn store_transactions(uri: &str, trytes: &[String]) -> Result<StoreTransactionsResponse> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "storeTransactions",
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
