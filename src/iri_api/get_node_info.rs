use super::responses::GetNodeInfoResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Gets information about the specified node
pub fn get_node_info(uri: &str) -> Result<GetNodeInfoResponse> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNodeInfo",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
