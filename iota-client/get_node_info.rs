use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

/// Gets information about the specified node
pub(crate) fn get_node_info(
    client: &Client,
    uri: &str,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "getNodeInfo",
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
