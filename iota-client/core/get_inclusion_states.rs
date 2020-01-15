use reqwest::{Client, Error};

/// Struct used to provide named arguments for `get_inclusion_states`
#[derive(Clone, Debug, Default)]
pub struct GetInclusionStatesOptions {
    /// Transactions to search for
    pub transactions: Vec<String>,
    /// Tips to search
    pub tips: Vec<String>,
}

/// Get the inclusion states of a set of transactions. This is
/// for determining if a transaction was accepted and confirmed
/// by the network or not. You can search for multiple tips (and
/// thus, milestones) to get past inclusion states of transactions.
///
/// This API call simply returns a list of boolean values in the
/// same order as the transaction list you submitted, thus you get
/// a true/false whether a transaction is confirmed or not.
pub(crate) async fn get_inclusion_states(
    client: &Client,
    uri: &str,
    options: GetInclusionStatesOptions,
) -> Result<GetInclusionStatesResponse, Error> {
    let body = json!({
        "command": "getInclusionStates",
        "transactions": options.transactions,
        "tips": options.tips,
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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetInclusionStatesResponse {
    /// Any errors that occurred
    error: Option<String>,
    /// States if found
    states: Option<Vec<bool>>,
}

impl GetInclusionStatesResponse {
    /// Returns any potential errors
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the states attribute
    pub fn states(&self) -> &Option<Vec<bool>> {
        &self.states
    }
}
