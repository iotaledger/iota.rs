use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::Future;

#[derive(Clone, Debug)]
pub struct GetBalancesOptions {
    pub addresses: Vec<String>,
    pub threshold: i32,
    pub tips: Option<Vec<String>>,
}

impl Default for GetBalancesOptions {
    fn default() -> Self {
        GetBalancesOptions {
            addresses: vec![],
            threshold: 100,
            tips: None,
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

    if let Some(tips) = options.tips {
        body["tips"] = json!(tips);
    }

    client
        .post(uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}
