use iota_model::{Inputs, Transaction, Transfer};

use crate::client::Client;
use crate::options::{PrepareTransfersOptions, SendTrytesOptions};
use crate::Result;

/// SendTransferOptions
#[derive(Clone, Debug, PartialEq)]
pub struct SendTransferOptions<'a, 'b, 'c> {
    /// The depth for getting transactions to approve
    pub depth: usize,
    /// The minimum weight magnitude for doing proof of work
    pub min_weight_magnitude: usize,
    /// Perform PoW locally
    pub local_pow: bool,
    /// Optionally specify the number of threads to use for PoW. This is ignored if `local_pow` is false.
    pub threads: usize,
    /// Optionally specify which inputs to use when trying to find funds for transfers
    pub inputs: Option<Inputs>,
    /// Optionally specify where to start searching for transactions to approve
    pub reference: Option<&'a str>,
    /// Optionally specify where to send remaining funds after spending from addresses, automatically generated if not specified
    pub remainder_address: Option<&'b str>,
    /// Optioanlly specify the security to use for address generation (1-3). Default is 2
    pub security: usize,
    /// Optionally specify an HMAC key to use for this transaction
    pub hmac_key: Option<&'c str>,
}

impl<'a, 'b, 'c> Default for SendTransferOptions<'a, 'b, 'c> {
    fn default() -> Self {
        SendTransferOptions {
            depth: 3,
            min_weight_magnitude: 14,
            local_pow: true,
            threads: num_cpus::get(),
            inputs: None,
            reference: None,
            remainder_address: None,
            security: 3,
            hmac_key: None,
        }
    }
}

impl<'a> Client<'a> {
    /// Prepares and sends a slice of transfers
    /// This helper does everything for you, PoW and such
    ///
    /// * `transfers` - A slice of transfers to send
    /// * `seed` - The wallet seed to use
    /// * `depth` - The depth to search when looking for transactions to approve
    /// * `min_weight_magnitude` - The PoW difficulty factor (14 on mainnet, 9 on testnet)
    /// * `local_pow` - Whether or not to do local PoW
    /// * `options` - See `SendTransferOptions`
    pub fn send_transfers(
        &mut self,
        transfers: impl Into<Vec<Transfer>>,
        seed: &str,
        options: SendTransferOptions<'_, '_, '_>,
    ) -> Result<Vec<Transaction>> {
        let transfers = transfers.into();
        let trytes = self.prepare_transfers(
            seed,
            transfers,
            PrepareTransfersOptions {
                inputs: options.inputs,
                remainder_address: options.remainder_address,
                security: options.security,
                hmac_key: options.hmac_key,
            },
        )?;
        let t = self.send_trytes(
            &trytes,
            SendTrytesOptions {
                depth: options.depth,
                min_weight_magnitude: options.min_weight_magnitude,
                local_pow: options.local_pow,
                threads: options.threads,
                reference: options.reference,
            },
        )?;
        Ok(t)
    }
}
