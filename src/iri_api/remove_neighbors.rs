use super::responses::RemoveNeighborsResponse;
use crate::Result;
use reqwest::Client;
/// Removes a list of neighbors to your node.
/// This is only temporary, and if you have your neighbors
/// added via the command line, they will be retained after
/// you restart your node.
pub async fn remove_neighbors(
    client: Client,
    uri: String,
    uris: Vec<String>,
) -> Result<RemoveNeighborsResponse> {
    let body = json!({
        "command": "removeNeighbors",
        "uris": uris,
    });

    Ok(client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?)
}
