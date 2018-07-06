use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn store_transactions(
    uri: &str,
    trytes: &[String],
) -> Result<StoreTransactionsResponse, Error> {
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

#[derive(Deserialize, Debug)]
pub struct StoreTransactionsResponse {
    duration: i64,
    error: Option<String>,
    exception: Option<String>,
}

impl StoreTransactionsResponse {
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
