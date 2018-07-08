use super::responses::AttachToTangleResponse;
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
///
/// * `uri` - the uri used to make the request
/// * `trunk_transaction` - trunk transaction to confirm
/// * `branch_transaction` - branch transaction to confirm
/// * `min_weight_magnitude` - Difficulty of PoW
/// * `trytes` - tryes to use for PoW
pub fn attach_to_tangle(
    uri: &str,
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
        .post(uri)
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

/// Performs proof of work locally
///
/// * `threads` - Optionally specify the number of threads
/// to use for Pow. Defaults to CPU thread count.
/// * `trunk_transaction` - trunk transaction to confirm
/// * `branch_transaction` - branch transaction to confirm
/// * `min_weight_magnitude` - Difficulty of PoW
/// * `trytes` - tryes to use for PoW
pub fn attach_to_tangle_local(
    threads: Option<usize>,
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

    let mut result_trytes: Vec<String> = Vec::with_capacity(trytes.len());
    let mut previous_transaction: Option<String> = None;
    let mut pearl_diver = PearlDiver::new();
    for i in 0..trytes.len() {
        let mut tx: Transaction = trytes[i].parse()?;

        let new_trunk_tx = if let Some(previous_transaction) = &previous_transaction {
            previous_transaction.to_string()
        } else {
            trunk_transaction.to_string()
        };
        tx.set_trunk_transaction(new_trunk_tx);

        let new_branch_tx = if previous_transaction.is_some() {
            trunk_transaction
        } else {
            branch_transaction
        };
        tx.set_branch_transaction(new_branch_tx);

        let tag = tx.tag().unwrap_or_default();
        if tag.is_empty() || tag == "9".repeat(27) {
            *tx.tag_mut() = tx.obsolete_tag();
        }
        tx.set_attachment_timestamp(Utc::now().timestamp_millis());
        tx.set_attachment_timestamp_lower_bound(0);
        tx.set_attachment_timestamp_upper_bound(*MAX_TIMESTAMP_VALUE);
        let mut tx_trits = converter::trits_from_string(&tx.to_trytes());
        pearl_diver.search(&mut tx_trits, min_weight_magnitude, threads)?;
        result_trytes.push(converter::trits_to_string(&tx_trits)?);
        previous_transaction = result_trytes[i].parse::<Transaction>()?.hash();
    }
    result_trytes.reverse();
    Ok(AttachToTangleResponse::new(
        0,
        None,
        None,
        None,
        Some(result_trytes),
    ))
}
