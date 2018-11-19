use super::responses::AddNeighborsResponse;
use crate::Result;
use reqwest::Client;

/// Add a list of neighbors to your node. It should be noted that
/// this is only temporary, and the added neighbors will be removed
/// from your set of neighbors after you relaunch IRI.
pub async fn add_neighbors(
    client: Client,
    uri: String,
    uris: Vec<String>,
) -> Result<AddNeighborsResponse> {
    let body = json!({
        "command": "addNeighbors",
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
