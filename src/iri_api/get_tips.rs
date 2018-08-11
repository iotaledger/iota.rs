use super::responses::GetTipsResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};
use reqwest::Client;
/// Returns the list of tups
pub async fn get_tips(client: Client, uri: String) -> Result<GetTipsResponse> {
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTips",
    });

    Ok(client
        .post(&uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
