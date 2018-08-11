use super::responses::GetNodeInfoResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;
/// Gets information about the specified node
pub async fn get_node_info(client: Client, uri: String) -> Result<GetNodeInfoResponse> {
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNodeInfo",
    });

    Ok(client
        .post(&uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
