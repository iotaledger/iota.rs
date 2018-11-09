use super::responses::GetNodeInfoResponse;
use crate::Result;
use reqwest::Client;
/// Gets information about the specified node
pub async fn get_node_info(client: Client, uri: String) -> Result<GetNodeInfoResponse> {
    let body = json!({
        "command": "getNodeInfo",
    });

    Ok(client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?)
}
