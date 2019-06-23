use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

#[derive(Clone, Default, Debug)]
pub struct FindTransactionsOptions {
    pub bundles: Vec<String>,
    pub addresses: Vec<String>,
    pub tags: Vec<String>,
    pub approvees: Vec<String>,
}

/// Finds transactions the match any of the provided parameters
pub fn find_transactions(
    client: &Client,
    uri: &str,
    options: FindTransactionsOptions,
) -> impl Future<Item = Response, Error = Error> {
    let mut body = json!({
        "command": "findTransactions",
    });

    if !options.bundles.is_empty() {
        body["bundles"] = json!(options.bundles);
    }
    if !options.addresses.is_empty() {
        body["addresses"] = json!(options.addresses);
    }
    if !options.tags.is_empty() {
        body["tags"] = json!(options.tags);
    }
    if !options.approvees.is_empty() {
        body["approvees"] = json!(options.approvees);
    }

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
