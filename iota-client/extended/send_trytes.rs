use iota_model::Transaction;

use crate::client::Client;
use crate::core::attach_to_tangle::attach_to_tangle_local;
use crate::options::{AttachOptions, GetTransactionsToApproveOptions};
use crate::Result;

/// SendTrytesOptions
#[derive(Clone, Debug, PartialEq)]
pub struct SendTrytesOptions<'a> {
    /// The depth for getting transactions to approve
    pub depth: usize,
    /// The minimum weight magnitude for doing proof of work
    pub min_weight_magnitude: usize,
    /// Perform PoW locally
    pub local_pow: bool,
    /// Optionally specify how many threads to use, defaults to max available
    pub threads: usize,
    /// Optionally used as the reference to start searching for transactions to approve
    pub reference: Option<&'a str>,
}

impl<'a> Default for SendTrytesOptions<'a> {
    fn default() -> Self {
        SendTrytesOptions {
            depth: 3,
            min_weight_magnitude: 14,
            local_pow: true,
            threads: num_cpus::get(),
            reference: None,
        }
    }
}

impl<'a> Client<'a> {
    /// Send trytes is a helper function that:
    ///
    /// 1. Gets transactions to approve
    /// 2. Does PoW
    /// 3. Sends your transactions to the IRI
    ///
    /// You should probably use `send_transfers`
    ///
    /// * `trytes` - A slice of strings that are tryte-encoded transactions
    /// * `depth` - The depth to search for transactions to approve
    /// * `min_weight_magnitude` - The PoW difficulty factor (14 on mainnet, 9 on testnet)
    /// * `local_pow` - Whether or not to do local PoW
    /// * `options` - See `SendTrytesOptions`
    pub async fn send_trytes(
        &mut self,
        trytes: &[String],
        options: SendTrytesOptions<'_>,
    ) -> Result<Vec<Transaction>> {
        let to_approve = self
            .get_transactions_to_approve(GetTransactionsToApproveOptions {
                depth: options.depth,
                reference: options.reference,
            })
            .await?;
        let attach_options = AttachOptions {
            threads: options.threads,
            trunk_transaction: &to_approve
                .trunk_transaction()
                .clone()
                .ok_or_else(|| format_err!("Trunk transaction is empty"))?,
            branch_transaction: &to_approve
                .branch_transaction()
                .clone()
                .ok_or_else(|| format_err!("Branch transaction is empty"))?,
            trytes,
            ..AttachOptions::default()
        };
        let trytes_list = if options.local_pow {
            let res = attach_to_tangle_local(attach_options)?;
            res.trytes().unwrap()
        } else {
            let attached = self.attach_to_tangle(attach_options).await?;
            attached.trytes().unwrap()
        };
        self.store_and_broadcast(&trytes_list).await?;
        Ok(trytes_list
            .iter()
            .map(|trytes| trytes.parse().unwrap())
            .collect())
    }
}
