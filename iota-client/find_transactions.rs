use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Finds transactions the match any of the provided parameters
pub fn find_transactions(
    client: &Client,
    uri: String,
    bundles: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    approvees: Option<Vec<String>>,
) -> impl Future<Item = Response, Error = Error> {
    let mut body = json!({
        "command": "findTransactions",
    });

    if let Some(b) = bundles {
        body["bundles"] = json!(b);
    }
    if let Some(a) = addresses {
        body["addresses"] = json!(a);
    }
    if let Some(t) = tags {
        body["tags"] = json!(t);
    }
    if let Some(a) = approvees {
        body["approvees"] = json!(a);
    }

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
