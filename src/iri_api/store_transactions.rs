use super::responses::StoreTransactionsResponse;
use crate::Result;
use reqwest::Client;
/// Store transactions into the local storage.
/// The trytes to be used for this call are
/// returned by attachToTangle.
pub async fn store_transactions(
    client: Client,
    uri: String,
    trytes: Vec<String>,
) -> Result<StoreTransactionsResponse> {
    let body = json!({
        "command": "storeTransactions",
        "trytes": trytes,
    });

    Ok(client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?)
}
