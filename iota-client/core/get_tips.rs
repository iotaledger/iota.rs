use reqwest::{Client, Error};

/// Returns the list of tips
pub(crate) async fn get_tips(client: &Client, uri: &str) -> Result<GetTipsResponse, Error> {
    let body = json!({
        "command": "getTips",
    });

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
        .await?
        .json()
        .await
}

/// This is a typed representation of the JSON response
#[derive(Clone, Serialize, Default, Deserialize, Debug)]
pub struct GetTipsResponse {
    /// Hashes of tips
    hashes: Vec<String>,
}

impl GetTipsResponse {
    /// Returns the hashes attribute
    pub fn hashes(&self) -> &[String] {
        &self.hashes
    }
    /// Takes ownership the hashes attribute
    pub fn take_hashes(self) -> Vec<String> {
        self.hashes
    }
}
