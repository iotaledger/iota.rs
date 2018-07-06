use crate::Result;
use reqwest::header::{ContentType, Headers};

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
pub fn get_transactions_to_approve(
    uri: &str,
    depth: usize,
    reference: &Option<String>,
) -> Result<GetTransactionsToApprove> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let mut body = json!({
        "command": "getTransactionsToApprove",
        "depth": depth,
    });

    if let Some(reference) = reference {
        body["reference"] = json!(reference);
    }

    let to_approve: GetTransactionsToApprove = client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = to_approve.error() {
        return Err(format_err!("{}", error));
    }

    Ok(to_approve)
}

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct GetTransactionsToApprove {
    duration: i64,
    error: Option<String>,
    #[serde(rename = "trunkTransaction")]
    trunk_transaction: Option<String>,
    #[serde(rename = "branchTransaction")]
    branch_transaction: Option<String>,
}

impl GetTransactionsToApprove {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the error attribute
    fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the trunk_transaction attribute
    pub fn trunk_transaction(&self) -> Option<String> {
        self.trunk_transaction.clone()
    }
    /// Returns the branch_transaction attribute
    pub fn branch_transaction(&self) -> Option<String> {
        self.branch_transaction.clone()
    }
}
