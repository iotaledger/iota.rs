use crate::error::Result;
use bee_crypto::ternary::Hash;
use bee_ternary::{T3B1Buf, TryteBuf};
use bee_transaction::bundled::BundledTransaction as Transaction;

use crate::response::{AttachToTangleResponse, AttachToTangleResponseBuilder};
use crate::util::tx_trytes;
use crate::Client;

//pow
use crate::extended::{PearlDiver, PowOptions};
use chrono::prelude::*;
use iota_conversion::Trinary;
use iota_model::Transaction as PoWTransaction;
use std::convert::TryInto;

/// This is a computed constant that represent the maximum allowed timestamp value
/// (3_i64.pow(27) - 1) / 2
static MAX_TIMESTAMP_VALUE: i64 = 3812798742493;

/// Builder to construct attachToTangle API
#[derive(Debug)]
pub struct AttachToTangleBuilder<'a> {
    client: &'a Client,
    trunk_transaction: String,
    branch_transaction: String,
    min_weight_magnitude: u8,
    trytes: Vec<String>,
    pow_local: bool,
}

impl<'a> AttachToTangleBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            trunk_transaction: Default::default(),
            branch_transaction: Default::default(),
            min_weight_magnitude: client.mwm,
            trytes: Default::default(),
            pow_local: true,
        }
    }

    /// Set trunk transaction hash
    pub fn trunk_transaction(mut self, trunk_transaction: &Hash) -> Self {
        self.trunk_transaction = trunk_transaction
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>();
        self
    }

    /// Set branch transaction hash
    pub fn branch_transaction(mut self, branch_transaction: &Hash) -> Self {
        self.branch_transaction = branch_transaction
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>();
        self
    }

    /// Set difficulty of PoW
    pub fn min_weight_magnitude(mut self, min_weight_magnitude: u8) -> Self {
        self.min_weight_magnitude = min_weight_magnitude;
        self
    }

    /// Set local PoW
    pub fn local_pow(mut self, local_pow: bool) -> Self {
        self.pow_local = local_pow;
        self
    }

    /// Add slice of transaction trytes. When sending transactions in a bundle,
    /// make sure that the trytes of the last transaction in the bundle are in index 0 of the array.
    pub fn trytes(mut self, trytes: &[Transaction]) -> Self {
        self.trytes = trytes.iter().map(|tx| tx_trytes(tx)).collect();
        self
    }

    /// Send attachToTangle request
    pub async fn send(self) -> Result<AttachToTangleResponse> {
        match self.pow_local {
            true => {
                let attach_options = AttachOptions {
                    threads: num_cpus::get(),
                    trunk_transaction: &self.trunk_transaction,
                    branch_transaction: &self.branch_transaction,
                    min_weight_magnitude: self.min_weight_magnitude as usize,
                    trytes: &self.trytes,
                };
                attach_to_tangle_local(attach_options)
            }
            false => {
                let body = json!({
                    "command": "attachToTangle",
                    "trunkTransaction": self.trunk_transaction,
                    "branchTransaction": self.branch_transaction,
                    "minWeightMagnitude": self.min_weight_magnitude,
                    "trytes": self.trytes,
                });
                let client = self.client;
                let res: AttachToTangleResponseBuilder = response!(client, body);
                res.build().await
            }
        }
    }
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
    // ensure!(
    //     input_validator::is_hash(&options.trunk_transaction),
    //     "Provided trunk transaction is not valid: {:?}",
    //     options.trunk_transaction
    // );
    // ensure!(
    //     input_validator::is_hash(&options.branch_transaction),
    //     "Provided branch transaction is not valid: {:?}",
    //     options.branch_transaction
    // );
    // ensure!(
    //     input_validator::is_array_of_trytes(&options.trytes),
    //     "Provided trytes are not valid: {:?}",
    //     options.trytes
    // );

    let mut result_trytes: Vec<String> = Vec::with_capacity(options.trytes.len());
    let mut previous_transaction = String::new();
    for i in 0..options.trytes.len() {
        let mut tx: PoWTransaction = options.trytes[i].parse().unwrap();

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
        tx.attachment_timestamp_upper_bound = MAX_TIMESTAMP_VALUE;
        let tx_trytes: String = tx.try_into().unwrap();
        let tx_trits = tx_trytes.trits();
        let result_trits = PearlDiver::default()
            .search(
                tx_trits,
                PowOptions {
                    min_weight_magnitude: options.min_weight_magnitude,
                    ..PowOptions::default()
                },
            )
            .unwrap();
        result_trytes.push(result_trits.trytes().unwrap());
        previous_transaction = result_trytes[i].parse::<PoWTransaction>().unwrap().hash;
    }
    result_trytes.reverse();
    let bundled_transaction = result_trytes
        .iter()
        .map(|tx| Transaction::from_trits(TryteBuf::try_from_str(&tx).unwrap().as_trits()).unwrap())
        .collect();
    Ok(AttachToTangleResponse {
        trytes: bundled_transaction,
    })
}
