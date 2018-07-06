use chrono::prelude::*;
use crate::crypto::PearlDiver;
use crate::model::*;
use crate::utils::converter;
use crate::utils::input_validator;
use crate::Result;
use reqwest::header::{ContentType, Headers};
use std::time::Duration;

lazy_static! {
    /// This is a computed constant that represent the maximum allowed timestamp value
    pub static ref MAX_TIMESTAMP_VALUE: i64 = (3_i64.pow(27) - 1) / 2;
}

/// Performs proof of work
/// `uri` - If None, local PoW is done, otherwise, we ask IRI
/// `trunk_transaction` - trunk transaction to confirm
/// `branch_transaction` - branch transaction to confirm
/// `min_weight_magnitude` - Difficulty of PoW
/// `trytes` - tryes to use for PoW
pub fn attach_to_tangle(
    uri: Option<String>,
    trunk_transaction: &str,
    branch_transaction: &str,
    min_weight_magnitude: usize,
    trytes: &[String],
) -> Result<AttachToTangleResponse> {
    ensure!(
        input_validator::is_hash(trunk_transaction),
        "Provided trunk transaction is not valid: {:?}",
        trunk_transaction
    );
    ensure!(
        input_validator::is_hash(branch_transaction),
        "Provided branch transaction is not valid: {:?}",
        branch_transaction
    );
    ensure!(
        input_validator::is_array_of_trytes(trytes),
        "Provided trytes are not valid: {:?}",
        trytes
    );

    if uri == None {
        let mut result_trytes: Vec<String> = Vec::with_capacity(trytes.len());
        let mut previous_transaction: Option<String> = None;
        let mut pearl_diver = PearlDiver::new();
        for i in 0..trytes.len() {
            let mut tx: Transaction = trytes[i].parse()?;
            *tx.trunk_transaction_mut() = if let Some(previous_transaction) = &previous_transaction
            {
                Some(previous_transaction.to_string())
            } else {
                Some(trunk_transaction.to_string())
            };
            *tx.branch_transaction_mut() = if previous_transaction.is_some() {
                Some(trunk_transaction.to_string())
            } else {
                Some(branch_transaction.to_string())
            };
            let tag = tx.tag().unwrap_or_default();
            if tag.is_empty() || tag == "9".repeat(27) {
                *tx.tag_mut() = tx.obsolete_tag();
            }
            *tx.attachment_timestamp_mut() = Some(Utc::now().timestamp_millis());
            *tx.attachment_timestamp_lower_bound_mut() = Some(0);
            *tx.attachment_timestamp_upper_bound_mut() = Some(*MAX_TIMESTAMP_VALUE);
            let mut tx_trits = converter::trits_from_string(&tx.to_trytes());
            pearl_diver.search(&mut tx_trits, min_weight_magnitude)?;
            result_trytes.push(converter::trits_to_string(&tx_trits)?);
            previous_transaction = result_trytes[i].parse::<Transaction>()?.hash();
        }
        result_trytes.reverse();
        return Ok(AttachToTangleResponse {
            duration: 0,
            id: None,
            error: None,
            exception: None,
            trytes: Some(result_trytes),
        });
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "attachToTangle",
        "trunkTransaction": trunk_transaction,
        "branchTransaction": branch_transaction,
        "minWeightMagnitude": min_weight_magnitude,
        "trytes": trytes,
    });

    let attach_resp: AttachToTangleResponse = client
        .post(&uri.unwrap())
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = attach_resp.error() {
        return Err(format_err!("{}", error));
    }
    if let Some(exception) = attach_resp.exception() {
        return Err(format_err!("{}", exception));
    }

    Ok(attach_resp)
}

/// This is a typed representation of the JSON response
/// `duration` will be zero if local PoW is selected.
#[derive(Deserialize, Debug)]
pub struct AttachToTangleResponse {
    duration: i64,
    id: Option<String>,
    error: Option<String>,
    exception: Option<String>,
    trytes: Option<Vec<String>>,
}

impl AttachToTangleResponse {
    /// Returns the duration attribute
    pub fn duration(&self) -> i64 {
        self.duration
    }
    /// Returns the id attribute
    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }
    /// Returns the error attribute
    fn error(&self) -> Option<String> {
        self.error.clone()
    }
    /// Returns the exception attribute
    fn exception(&self) -> Option<String> {
        self.exception.clone()
    }
    /// Returns the trytes attribute
    pub fn trytes(self) -> Option<Vec<String>> {
        self.trytes
    }
}
