use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Finds transactions the match any of the provided parameters
pub fn find_transactions(
    uri: &str,
    bundles: Option<&[String]>,
    addresses: Option<&[String]>,
    tags: Option<&[String]>,
    approvees: Option<&[String]>,
) -> Result<FindTransactionsResponse> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let mut body = json!({
        "command": "findTransactions",
    });

    if let Some(b) = bundles {
        body["bundles"] = json!(b);
    }
    if let Some(a) = addresses {
        body["addresses"] = json!(a);
    }
    if let Some(t) = tags {
        body["tags"] = json!(t);
    }
    if let Some(a) = approvees {
        body["approvees"] = json!(a);
    }

    let resp: FindTransactionsResponse = client
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
pub struct FindTransactionsResponse {
    duration: i64,
    error: Option<String>,
    hashes: Option<Vec<String>>,
}

impl FindTransactionsResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the hashes attribute
    pub fn hashes(&self) -> &Option<Vec<String>> {
        &self.hashes
    }
    /// Takes ownership of the hashes attribute
    pub fn take_hashes(self) -> Option<Vec<String>> {
        self.hashes
    }
}
