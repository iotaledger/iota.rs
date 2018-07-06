use super::responses::GetNeighborsResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Returns the set of neighbors you are connected with, as
/// well as their activity count. The activity counter is reset
/// after restarting IRI.
pub fn get_neighbors(uri: &str) -> Result<GetNeighborsResponse> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNeighbors",
    });

    let resp: GetNeighborsResponse = client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = resp.error() {
        return Err(format_err!("{}", error));
    }

    Ok(resp)
}
