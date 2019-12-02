use reqwest::{Client, Error, Response};

/// Check if a list of addresses was ever spent from.
pub(crate) async fn were_addresses_spent_from(
    client: &Client,
    uri: &str,
    addresses: &[String],
) -> Result<Response, Error> {
    let body = json!({
        "command": "wereAddressesSpentFrom",
        "addresses": addresses,
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
#[derive(Clone, Serialize, Default, Deserialize, Debug)]
pub struct WereAddressesSpentFromResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// States of addresses if found
    states: Option<Vec<bool>>,
}

impl WereAddressesSpentFromResponse {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the states attribute
    pub fn states(self) -> Option<Vec<bool>> {
        self.states
    }
    /// Returns a specfic index into the states attribute
    pub fn state(self, index: usize) -> bool {
        self.states.unwrap_or_default()[index]
    }
}
