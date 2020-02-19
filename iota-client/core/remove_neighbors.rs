use reqwest::{Client, Error};

/// Removes a list of neighbors to your node.
/// This is only temporary, and if you have your neighbors
/// added via the command line, they will be retained after
/// you restart your node.
pub(crate) async fn remove_neighbors(
    client: &Client,
    uri: &str,
    uris: &[String],
) -> Result<RemoveNeighborsResponse, Error> {
    let body = json!({
        "command": "removeNeighbors",
        "uris": uris,
    });

    client
        .post(uri)
        .header("Content-Type", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
        .await?
        .json()
        .await
}

/// This is a typed representation of the JSON response
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RemoveNeighborsResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
    /// Amount of neighbors removed
    #[serde(rename = "removedNeighbors")]
    removed_neighbors: Option<usize>,
}

impl RemoveNeighborsResponse {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    pub fn exception(&self) -> &Option<String> {
        &self.exception
    }
    /// Returns a reference to the amount of removed neighbors
    pub fn removed_neighbors(&self) -> &Option<usize> {
        &self.removed_neighbors
    }
    /// Consumes the response and returns the amount of removed neighbors if any
    pub fn take_removed_neighbors(self) -> Option<usize> {
        self.removed_neighbors
    }
}
