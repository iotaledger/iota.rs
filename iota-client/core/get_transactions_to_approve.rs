use reqwest::{Client, Error};

/// Struct used to provide named arguments for `get_transactions_to_approve`
#[derive(Clone, Debug)]
pub struct GetTransactionsToApproveOptions<'a> {
    /// How deep to search
    pub depth: usize,
    /// Where to start search
    pub reference: Option<&'a str>,
}

/// Provide sane defaults for the fields
/// * `depth` - 3
/// * `reference` - None
impl<'a> Default for GetTransactionsToApproveOptions<'a> {
    fn default() -> Self {
        GetTransactionsToApproveOptions {
            depth: 3,
            reference: None,
        }
    }
}

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
pub(crate) async fn get_transactions_to_approve(
    client: &Client,
    uri: &str,
    options: GetTransactionsToApproveOptions<'_>,
) -> Result<GetTransactionsToApprove, Error> {
    let mut body = json!({
        "command": "getTransactionsToApprove",
        "depth": options.depth,
    });

    if let Some(reference) = options.reference {
        body["reference"] = json!(reference);
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
#[derive(Clone, Serialize, Default, Deserialize, Debug)]
pub struct GetTransactionsToApprove {
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
    /// Trunk transaction to approve
    #[serde(rename = "trunkTransaction")]
    trunk_transaction: Option<String>,
    /// Branch transaction to approve
    #[serde(rename = "branchTransaction")]
    branch_transaction: Option<String>,
}

impl GetTransactionsToApprove {
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    pub fn exception(&self) -> &Option<String> {
        &self.exception
    }
    /// Returns the trunk_transaction attribute
    pub fn trunk_transaction(&self) -> &Option<String> {
        &self.trunk_transaction
    }
    /// Returns the branch_transaction attribute
    pub fn branch_transaction(&self) -> &Option<String> {
        &self.branch_transaction
    }
}
