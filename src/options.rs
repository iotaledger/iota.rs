use super::model::*;

/// SendTransferOptions
///
/// * `threads` - Optionally specify the number of threads to use for PoW. This is ignored if `local_pow` is false.
/// * `inputs` - Optionally specify which inputs to use when trying to find funds for transfers
/// * `reference` - Optionally specify where to start searching for transactions to approve
/// * `remainder_address` - Optionally specify where to send remaining funds after spending from addresses, automatically generated if not specified
/// * `security` - Optioanlly specify the security to use for address generation (1-3). Default is 2
/// * `hmac_key` - Optionally specify an HMAC key to use for this transaction
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SendTransferOptions {
    pub threads: Option<usize>,
    pub inputs: Option<Inputs>,
    pub reference: Option<String>,
    pub remainder_address: Option<String>,
    pub security: Option<usize>,
    pub hmac_key: Option<String>,
}

/// GetNewAddressOptions
///
/// * `security` - Security factor 1-3 with 3 being most secure
/// * `index` - How many iterations of generating to skip
/// * `total` - Number of addresses to generate. If total isn't provided, we generate until we find an unused address
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetNewAddressOptions {
    pub security: Option<usize>,
    pub index: Option<usize>,
    pub total: Option<usize>,
}

/// SendTrytesOptions
///
/// * `thread` - Optionally specify how many threads to use, defaults to max available
/// * `reference` - Optionally used as the reference to start searching for transactions to approve
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SendTrytesOptions {
    pub threads: Option<usize>,
    pub reference: Option<String>,
}

/// GetInputsOptions
///
/// * `start` - The start index for addresses to search
/// * `end` - The end index for addresses to search
/// * `threshold` - The amount of Iota you're trying to find in the wallet
/// * `security` - The security to use for address generation
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetInputsOptions {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub threshold: Option<i64>,
    pub security: Option<usize>,
}

/// PrepareTransfersOptions
///
/// * `inputs` - Optional inputs to use if you're sending iota
/// * `remainder_address` - Optional remainder address to use, if not provided, one will be generated
/// * `security` - Security to use when generating addresses (1-3)
/// * `hmac_key` - Optional key to use if you want to hmac the transfers
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PrepareTransfersOptions {
    pub inputs: Option<Inputs>,
    pub remainder_address: Option<String>,
    pub security: Option<usize>,
    pub hmac_key: Option<String>,
}
