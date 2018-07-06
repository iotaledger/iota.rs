use crate::utils::input_validator;

use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Get the inclusion states of a set of transactions. This is
/// for determining if a transaction was accepted and confirmed
/// by the network or not. You can search for multiple tips (and
/// thus, milestones) to get past inclusion states of transactions.
///
/// This API call simply returns a list of boolean values in the
/// same order as the transaction list you submitted, thus you get
/// a true/false whether a transaction is confirmed or not.
pub fn get_inclusion_states(
    uri: &str,
    transactions: &[String],
    tips: &[String],
) -> Result<GetInclusionStatesResponse> {
    ensure!(
        input_validator::is_array_of_hashes(transactions),
        "Provided transactions are not valid: {:?}",
        transactions
    );
    ensure!(
        input_validator::is_array_of_hashes(tips),
        "Provided tips are not valid: {:?}",
        tips
    );

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getInclusionStates",
        "transactions": transactions,
        "tips": tips,
    });

    let resp: GetInclusionStatesResponse = client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = resp.error() {
        return Err(format_err!("{}", error));
    }

    Ok(resp)
}

/// This is a typed representation of the JSON response
#[derive(Deserialize, Debug)]
pub struct GetInclusionStatesResponse {
    duration: i64,
    error: Option<String>,
    states: Option<Vec<bool>>,
}

impl GetInclusionStatesResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    fn error(&self) -> Option<String> {
        self.error.clone()
    }
    /// Returns the states attribute
    pub fn states(self) -> Option<Vec<bool>> {
        self.states
    }
    /// Returns a specfic index into the states attribute
    pub fn state(self, index: usize) -> bool {
        self.states.unwrap_or_default()[index]
    }
}
