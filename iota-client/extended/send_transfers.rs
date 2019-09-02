use iota_model::Inputs;

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
