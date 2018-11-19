use super::responses::GetBalancesResponse;
use crate::Result;
use reqwest::Client;
/// Returns the balance based on the latest confirmed milestone.
/// In addition to the balances, it also returns the referencing tips (or milestone),
/// as well as the index with which the confirmed balance was
/// determined. The balances is returned as a list in the same
/// order as the addresses were provided as input.
pub async fn get_balances(
    client: Client,
    uri: String,
    addresses: Vec<String>,
    threshold: i32,
    tips: Option<Vec<String>>,
) -> Result<GetBalancesResponse> {
    let mut body = json!({
            "command": "getBalances",
            "addresses": addresses,
            "threshold": threshold,
        });

    if let Some(tips) = tips {
        body["tips"] = json!(tips);
    }

    Ok(client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?)
}
