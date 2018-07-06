use crate::model::Neighbor;

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

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct GetNeighborsResponse {
    duration: i64,
    error: Option<String>,
    neighbors: Option<Vec<Neighbor>>,
}

impl GetNeighborsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the error attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the neighbors attribute
    pub fn neighbors(self) -> Option<Vec<Neighbor>> {
        self.neighbors
    }
}
