use super::responses::GetTipsResponse;
use crate::Result;
use reqwest::Client;
/// Returns the list of tups
pub async fn get_tips(client: Client, uri: String) -> Result<GetTipsResponse> {
    let body = json!({
        "command": "getTips",
    });

    Ok(client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?)
}
