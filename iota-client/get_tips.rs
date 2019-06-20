use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Returns the list of tips
pub fn get_tips(client: &Client, uri: String) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "getTips",
    });

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
