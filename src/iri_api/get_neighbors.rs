use super::responses::GetNeighborsResponse;
use crate::Result;
use reqwest::Client;
/// Returns the set of neighbors you are connected with, as
/// well as their activity count. The activity counter is reset
/// after restarting IRI.
pub async fn get_neighbors(client: Client, uri: String) -> Result<GetNeighborsResponse> {
    let body = json!({
        "command": "getNeighbors",
    });

    let resp: GetNeighborsResponse = client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = resp.error() {
        return Err(format_err!("{}", error));
    }

    Ok(resp)
}
