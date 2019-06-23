use chrono::prelude::*;
use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::*;

use iota_conversion;
use iota_model::*;
use iota_pow::PearlDiver;
use iota_validation::input_validator;

use crate::Result;

use super::responses::AttachToTangleResponse;

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
    client: &Client,
    uri: String,
    trunk_transaction: String,
    branch_transaction: String,
    min_weight_magnitude: usize,
    trytes: Vec<String>,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "attachToTangle",
        "trunkTransaction": trunk_transaction,
        "branchTransaction": branch_transaction,
        "minWeightMagnitude": min_weight_magnitude,
        "trytes": trytes,
    });

    client
        .post(&uri)
        .header("ContentType", "application/json")
        .header("X-IOTA-API-Version", "1")
        .body(body.to_string())
        .send()
}

/// Performs proof of work locally
///
/// * `threads` - Optionally specify the number of threads
/// to use for Pow. Defaults to CPU thread count.
/// * `trunk_transaction` - trunk transaction to confirm
/// * `branch_transaction` - branch transaction to confirm
/// * `min_weight_magnitude` - Difficulty of PoW
/// * `trytes` - tryes to use for PoW
pub fn attach_to_tangle_local<T: Copy + Into<Option<usize>>>(
    threads: T,
    trunk_transaction: String,
    branch_transaction: String,
    min_weight_magnitude: usize,
    trytes: Vec<String>,
) -> Result<AttachToTangleResponse> {
    ensure!(
        input_validator::is_hash(&trunk_transaction),
        "Provided trunk transaction is not valid: {:?}",
        trunk_transaction
    );
    ensure!(
        input_validator::is_hash(&branch_transaction),
        "Provided branch transaction is not valid: {:?}",
        branch_transaction
    );
    ensure!(
        input_validator::is_array_of_trytes(&trytes),
        "Provided trytes are not valid: {:?}",
        trytes
    );

    let mut result_trytes: Vec<String> = Vec::with_capacity(trytes.len());
    let mut previous_transaction: Option<String> = None;
    for i in 0..trytes.len() {
        let mut tx: Transaction = trytes[i].parse()?;

        let new_trunk_tx = if let Some(previous_transaction) = &previous_transaction {
            previous_transaction.clone()
        } else {
            trunk_transaction.clone()
        };
        tx.set_trunk_transaction(new_trunk_tx);

        let new_branch_tx = if previous_transaction.is_some() {
            trunk_transaction.clone()
        } else {
            branch_transaction.clone()
        };
        tx.set_branch_transaction(new_branch_tx);

        let tag = tx.tag().unwrap_or_default();
        if tag.is_empty() || tag == "9".repeat(27) {
            *tx.tag_mut() = tx.obsolete_tag();
        }
        tx.set_attachment_timestamp(Utc::now().timestamp_millis());
        tx.set_attachment_timestamp_lower_bound(0);
        tx.set_attachment_timestamp_upper_bound(*MAX_TIMESTAMP_VALUE);
        let tx_trits = iota_conversion::trits_from_string(&tx.to_trytes());
        let result_trits = PearlDiver::default().search(
            tx_trits,
            min_weight_magnitude,
            threads.into(),
        )?;
        result_trytes.push(iota_conversion::trits_to_string(&result_trits)?);
        previous_transaction = result_trytes[i].parse::<Transaction>()?.hash();
    }
    result_trytes.reverse();
    Ok(AttachToTangleResponse::new(
        None,
        None,
        None,
        Some(result_trytes),
    ))
}
