use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Returns the balance based on the latest confirmed milestone.
/// In addition to the balances, it also returns the referencing tips (or milestone),
/// as well as the index with which the confirmed balance was
/// determined. The balances is returned as a list in the same
/// order as the addresses were provided as input.
pub fn get_balances(
    uri: &str,
    addresses: &[String],
    threshold: i32,
) -> Result<GetBalancesResponse> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getBalances",
        "addresses": addresses,
        "threshold": threshold,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

/// This is a typed representation of the JSON response
#[derive(Clone, Deserialize, Debug)]
pub struct GetBalancesResponse {
    duration: i64,
    error: Option<String>,
    balances: Option<Vec<String>>,
    #[serde(rename = "milestoneIndex")]
    milestone_index: Option<i64>,
    references: Option<Vec<String>>,
}

impl GetBalancesResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the duration attribute
    pub fn balances(&self) -> &Option<Vec<String>> {
        &self.balances
    }
    /// Returns the duration attribute
    pub fn take_balances(self) -> Option<Vec<String>> {
        self.balances
    }
    /// Returns the duration attribute
    pub fn milestone_index(&self) -> Option<i64> {
        self.milestone_index
    }
    /// Returns the duration attribute
    pub fn references(&self) -> &Option<Vec<String>> {
        &self.references
    }
    /// Returns the duration attribute
    pub fn take_references(self) -> Option<Vec<String>> {
        self.references
    }
}
