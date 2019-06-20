use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Returns the raw transaction data (trytes) of a specific
/// transaction. These trytes can then be easily converted
/// into the actual transaction object. See utility functions
/// for more details.
pub fn get_trytes(
    client: &Client,
    uri: String,
    hashes: Vec<String>,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "getTrytes",
        "hashes": hashes,
    });

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
