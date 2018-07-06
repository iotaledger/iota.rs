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

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct StoreTransactionsResponse {
    duration: i64,
    error: Option<String>,
    exception: Option<String>,
}

impl StoreTransactionsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the duration attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the duration attribute
    fn exception(&self) -> &Option<String> {
        &self.exception
    }
}
