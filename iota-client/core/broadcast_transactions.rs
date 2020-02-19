use reqwest::{Client, Error};

/// Broadcast a list of transactions to all neighbors.
/// The input trytes for this call are provided by attachToTangle.
pub(crate) async fn broadcast_transactions(
    client: &Client,
    uri: &str,
    trytes: &[String],
) -> Result<BroadcastTransactionsResponse, Error> {
    let body = json!({
        "command": "broadcastTransactions",
        "trytes": trytes,
    });

    client
        .post(uri)
        .header("Content-Type", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
        .await?
        .json()
        .await
}

/// This is a typed representation of the JSON response
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct BroadcastTransactionsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exception that occurred
    exception: Option<String>,
}

impl BroadcastTransactionsResponse {
    /// Returns any potential errors
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns any potential exceptions
    pub fn exception(&self) -> &Option<String> {
        &self.exception
    }
}
