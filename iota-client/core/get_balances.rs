use reqwest::{Client, Error};

/// Struct used to provide named arguments for `get_balances`
#[derive(Clone, Debug)]
pub struct GetBalancesOptions {
    /// Address to check
    pub addresses: Vec<String>,
    /// Stop searching after we've found this much Iota
    pub threshold: i32,
    /// Tips to search
    pub tips: Vec<String>,
}

/// Provides sane defaults for the fields
/// * `addresses` - Empty vector
/// * `threshold` - 100
/// * `tips` - Empty vector
impl Default for GetBalancesOptions {
    fn default() -> Self {
        GetBalancesOptions {
            addresses: vec![],
            threshold: 100,
            tips: vec![],
        }
    }
}

/// Returns the balance based on the latest confirmed milestone.
/// In addition to the balances, it also returns the referencing tips (or milestone),
/// as well as the index with which the confirmed balance was
/// determined. The balances is returned as a list in the same
/// order as the addresses were provided as input.
pub(crate) async fn get_balances(
    client: &Client,
    uri: &str,
    options: GetBalancesOptions,
) -> Result<GetBalancesResponse, Error> {
    let mut body = json!({
        "command": "getBalances",
        "addresses": options.addresses,
        "threshold": options.threshold,
    });

    if !options.tips.is_empty() {
        body["tips"] = json!(options.tips);
    }

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
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct GetBalancesResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// Balances if found
    balances: Option<Vec<String>>,
    /// Milestone index if found
    #[serde(rename = "milestoneIndex")]
    milestone_index: Option<i64>,
    /// References if found
    references: Option<Vec<String>>,
}

impl GetBalancesResponse {
    /// Returns any potential errors
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the balances attribute
    pub fn balances(&self) -> &Option<Vec<String>> {
        &self.balances
    }
    /// Takes ownership of the balances attribute
    pub fn take_balances(self) -> Option<Vec<String>> {
        self.balances
    }
    /// Returns the milestone_index attribute
    pub fn milestone_index(&self) -> Option<i64> {
        self.milestone_index
    }
    /// Returns the references attribute
    pub fn references(&self) -> &Option<Vec<String>> {
        &self.references
    }
    /// Takes ownership of the references attribute
    pub fn take_references(self) -> Option<Vec<String>> {
        self.references
    }
}
