use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::*;

/// Add a list of neighbors to your node. It should be noted that
/// this is only temporary, and the added neighbors will be removed
/// from your set of neighbors after you relaunch IRI.
pub(crate) fn add_neighbors(
    client: &Client,
    uri: &str,
    uris: &[String],
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "addNeighbors",
        "uris": uris,
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}

/// This is a typed representation of the JSON response
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct AddNeighborsResponse {
    #[serde(rename = "addedNeighbors")]
    added_neighbors: Option<usize>,
    error: Option<String>,
    exception: Option<String>,
}

impl AddNeighborsResponse {
    /// Returns the added neighbors
    pub fn added_neighbors(&self) -> &Option<usize> {
        &self.added_neighbors
    }
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    pub fn exception(&self) -> &Option<String> {
        &self.exception
    }
}
