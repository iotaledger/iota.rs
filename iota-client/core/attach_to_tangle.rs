use chrono::prelude::*;
use reqwest::r#async::{Client, Response};
use reqwest::Error;
use tokio::prelude::*;

use iota_conversion::Trinary;
use iota_model::*;
use iota_pow::{PearlDiver, PowOptions};
use iota_validation::input_validator;

use crate::Result;

use std::convert::TryInto;

lazy_static! {
    /// This is a computed constant that represent the maximum allowed timestamp value
    pub static ref MAX_TIMESTAMP_VALUE: i64 = (3_i64.pow(27) - 1) / 2;
}

/// Struct used to provide named arguments for `attach_to_tangle`
#[derive(Clone, Debug)]
pub struct AttachOptions<'a, 'b, 'c> {
    /// Number of threads to use for proof of work
    pub threads: usize,
    /// Trunk transaction encoded as a tryte string
    pub trunk_transaction: &'a str,
    /// Branch transaction encoded as a tryte string
    pub branch_transaction: &'b str,
    /// Difficulty factor to use for proof of work
    pub min_weight_magnitude: usize,
    /// Trytes to attach to tangle
    pub trytes: &'c [String],
}

/// Provides sane defaults for the fields
/// * `threads` - Number of CPUs
/// * `trunk_transaction` - Empty string
/// * `branch_transaction` - Empty string
/// * `min_weight_magnitude` - 14
/// * `trytes` - Empty vector
impl<'a, 'b, 'c> Default for AttachOptions<'a, 'b, 'c> {
    fn default() -> Self {
        AttachOptions {
            threads: num_cpus::get(),
            trunk_transaction: "",
            branch_transaction: "",
            min_weight_magnitude: 14,
            trytes: &[],
        }
    }
}

/// Performs proof of work
///
/// * `uri` - the uri used to make the request
/// * `trunk_transaction` - trunk transaction to confirm
/// * `branch_transaction` - branch transaction to confirm
/// * `min_weight_magnitude` - Difficulty of PoW
/// * `trytes` - tryes to use for PoW
pub(crate) fn attach_to_tangle(
    client: &Client,
    uri: &str,
    options: AttachOptions<'_, '_, '_>,
) -> impl Future<Item = Response, Error = Error> {
    let body = json!({
        "command": "attachToTangle",
        "trunkTransaction": options.trunk_transaction,
        "branchTransaction": options.branch_transaction,
        "minWeightMagnitude": options.min_weight_magnitude,
        "trytes": options.trytes,
    });

    client
        .post(uri)
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
pub fn attach_to_tangle_local(
    options: AttachOptions<'_, '_, '_>,
) -> Result<AttachToTangleResponse> {
    ensure!(
        input_validator::is_hash(&options.trunk_transaction),
        "Provided trunk transaction is not valid: {:?}",
        options.trunk_transaction
    );
    ensure!(
        input_validator::is_hash(&options.branch_transaction),
        "Provided branch transaction is not valid: {:?}",
        options.branch_transaction
    );
    ensure!(
        input_validator::is_array_of_trytes(&options.trytes),
        "Provided trytes are not valid: {:?}",
        options.trytes
    );

    let mut result_trytes: Vec<String> = Vec::with_capacity(options.trytes.len());
    let mut previous_transaction = String::new();
    for i in 0..options.trytes.len() {
        let mut tx: Transaction = options.trytes[i].parse()?;

        tx.trunk_transaction = if previous_transaction.is_empty() {
            options.trunk_transaction.into()
        } else {
            previous_transaction.clone()
        };

        tx.branch_transaction = if previous_transaction.is_empty() {
            options.branch_transaction
        } else {
            options.trunk_transaction
        }
        .into();

        if tx.tag.is_empty() || tx.tag == "9".repeat(27) {
            tx.tag = tx.obsolete_tag.clone();
        }
        tx.attachment_timestamp = Utc::now().timestamp_millis();
        tx.attachment_timestamp_lower_bound = 0;
        tx.attachment_timestamp_upper_bound = *MAX_TIMESTAMP_VALUE;
        let tx_trytes: String = tx.try_into()?;
        let tx_trits = tx_trytes.trits();
        let result_trits = PearlDiver::default().search(
            tx_trits,
            PowOptions {
                min_weight_magnitude: options.min_weight_magnitude,
                ..PowOptions::default()
            },
        )?;
        result_trytes.push(result_trits.trytes()?);
        previous_transaction = result_trytes[i].parse::<Transaction>()?.hash.into();
    }
    result_trytes.reverse();
    Ok(AttachToTangleResponse::new(
        None,
        None,
        None,
        Some(result_trytes),
    ))
}

/// This is a typed representation of the JSON response
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AttachToTangleResponse {
    /// This is only used when using PoW Box service
    #[serde(rename = "jobId")]
    job_id: Option<String>,
    /// Any errors that occurred
    error: Option<String>,
    /// Any exceptions that occurred
    exception: Option<String>,
    /// Trytes returned by PoW
    trytes: Option<Vec<String>>,
}

impl AttachToTangleResponse {
    /// Creates a new repsonse
    ///
    /// * `job_id` - This is only used when using PoW Box service
    /// * `error` - Any errors that occurred
    /// * `exception` - Any exceptions that occurred
    /// * `trytes` -  trytes returned by PoW
    pub fn new(
        job_id: Option<String>,
        error: Option<String>,
        exception: Option<String>,
        trytes: Option<Vec<String>>,
    ) -> AttachToTangleResponse {
        AttachToTangleResponse {
            job_id,
            error,
            exception,
            trytes,
        }
    }
    /// Returns the id attribute
    pub fn job_id(&self) -> &Option<String> {
        &self.job_id
    }
    /// Returns the error attribute
    pub fn error(&self) -> &Option<String> {
        &self.error
    }
    /// Returns the exception attribute
    pub fn exception(&self) -> &Option<String> {
        &self.exception
    }
    /// Returns the trytes attribute
    pub fn trytes(self) -> Option<Vec<String>> {
        self.trytes
    }
}
