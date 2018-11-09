use super::responses::GetTransactionsToApprove;
use crate::Result;
use reqwest::Client;
/// Tip selection which returns `trunkTransaction` and
/// `branchTransaction`. The input value depth determines
/// how many milestones to go back to for finding the
/// transactions to approve. The higher your depth value,
/// the more work you have to do as you are confirming more
/// transactions. If the depth is too large (usually above 15,
/// it depends on the node's configuration) an error will be
/// returned. The reference is an optional hash of a transaction
/// you want to approve. If it can't be found at the specified
/// depth then an error will be returned.
pub async fn get_transactions_to_approve(
    client: &Client,
    uri: String,
    depth: usize,
    reference: Option<String>,
) -> Result<GetTransactionsToApprove> {
    let mut body = json!({
        "command": "getTransactionsToApprove",
        "depth": depth,
    });

    if let Some(reference) = reference {
        body["reference"] = json!(reference);
    }

    let to_approve: GetTransactionsToApprove = client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = to_approve.error() {
        return Err(format_err!("{}", error));
    }
    if let Some(exception) = to_approve.exception() {
        return Err(format_err!("{}", exception));
    }

    Ok(to_approve)
}
