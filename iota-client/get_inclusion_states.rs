use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

#[derive(Clone, Debug, Default)]
pub struct GetInclusionStatesOptions {
    pub transactions: Vec<String>,
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
pub fn get_inclusion_states(
    client: &Client,
    uri: String,
    options: GetInclusionStatesOptions,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "getInclusionStates",
        "transactions": options.transactions,
        "tips": options.tips,
    });

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
