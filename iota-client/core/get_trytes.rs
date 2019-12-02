use reqwest::{Client, Error, Response};

/// Returns the raw transaction data (trytes) of a specific
/// transaction. These trytes can then be easily converted
/// into the actual transaction object. See utility functions
/// for more details.
pub(crate) async fn get_trytes(
    client: &Client,
    uri: &str,
    hashes: &[String],
) -> Result<Response, Error> {
    let body = json!({
        "command": "getTrytes",
        "hashes": hashes,
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
pub struct GetTrytesResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Trytes if found
    trytes: Option<Vec<String>>,
}

impl GetTrytesResponse {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the trytes attribute
    pub fn trytes(&self) -> &Option<Vec<String>> {
        &self.trytes
    }
    /// Takes ownership the trytes attribute
    pub fn take_trytes(self) -> Option<Vec<String>> {
        self.trytes
    }
}
