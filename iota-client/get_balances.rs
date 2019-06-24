use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

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
pub fn get_balances(
    client: &Client,
    uri: &str,
    options: GetBalancesOptions,
) -> impl Future<Item = Response, Error = Error> {
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
}
