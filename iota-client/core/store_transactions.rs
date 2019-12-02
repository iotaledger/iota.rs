use reqwest::{Client, Error, Response};

/// Store transactions into the local storage.
/// The trytes to be used for this call are
/// returned by attachToTangle.
pub(crate) async fn store_transactions(
    client: &Client,
    uri: &str,
    trytes: &[String],
) -> Result<Response, Error> {
    let body = json!({
        "command": "storeTransactions",
        "trytes": trytes,
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
        .await
}

/// This is a typed representation of the JSON response
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StoreTransactionsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
}

impl StoreTransactionsResponse {
    /// Returns the error attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    fn exception(&self) -> &Option<String> {
        &self.exception
    }
}
