use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Check if a list of addresses was ever spent from.
pub fn were_addresses_spent_from(
    client: &Client,
    uri: String,
    addresses: Vec<String>,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "wereAddressesSpentFrom",
        "addresses": addresses,
    });

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
