use iota_model::Neighbor;
use reqwest::{Client, Error, Response};

/// Returns the set of neighbors you are connected with, as
/// well as their activity count. The activity counter is reset
/// after restarting IRI.
pub(crate) async fn get_neighbors(client: &Client, uri: &str) -> Result<Response, Error> {
    let body = json!({
        "command": "getNeighbors",
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
        .await
}

/// This is a typed representation of the JSON response
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GetNeighborsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Neighbors if found
    neighbors: Option<Vec<Neighbor>>,
}

impl GetNeighborsResponse {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the neighbors attribute
    pub fn neighbors(self) -> Option<Vec<Neighbor>> {
        self.neighbors
    }
}
