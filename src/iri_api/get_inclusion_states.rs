use super::responses::GetInclusionStatesResponse;
use crate::utils::input_validator;
use crate::Result;
use reqwest::Client;
/// Get the inclusion states of a set of transactions. This is
/// for determining if a transaction was accepted and confirmed
/// by the network or not. You can search for multiple tips (and
/// thus, milestones) to get past inclusion states of transactions.
///
/// This API call simply returns a list of boolean values in the
/// same order as the transaction list you submitted, thus you get
/// a true/false whether a transaction is confirmed or not.
pub async fn get_inclusion_states(
    client: Client,
    uri: String,
    transactions: Vec<String>,
    tips: Vec<String>,
) -> Result<GetInclusionStatesResponse> {
    ensure!(
        input_validator::is_array_of_hashes(&transactions),
        "Provided transactions are not valid: {:?}",
        transactions
    );
    ensure!(
        input_validator::is_array_of_hashes(&tips),
        "Provided tips are not valid: {:?}",
        tips
    );

    let body = json!({
        "command": "getInclusionStates",
        "transactions": transactions,
        "tips": tips,
    });

    let resp: GetInclusionStatesResponse = client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = resp.error() {
        return Err(format_err!("{}", error));
    }

    Ok(resp)
}
