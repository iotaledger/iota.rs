use super::responses::WereAddressesSpentFromResponse;
use crate::utils::{self, input_validator};
use crate::Result;
use reqwest::Client;
/// Check if a list of addresses was ever spent from.
pub async fn were_addresses_spent_from(
    client: Client,
    uri: String,
    addresses: Vec<String>,
) -> Result<WereAddressesSpentFromResponse> {
    let addresses: Vec<String> = addresses
        .iter()
        .filter(|address| input_validator::is_address(address))
        .map(|address| utils::remove_checksum(address))
        .collect();
    ensure!(!addresses.is_empty(), "No valid addresses provided.");

    let body = json!({
        "command": "wereAddressesSpentFrom",
        "addresses": addresses,
    });

    Ok(client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?)
}
