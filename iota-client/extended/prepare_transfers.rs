use iota_model::Inputs;

/// PrepareTransfersOptions
#[derive(Clone, Debug, PartialEq)]
pub struct PrepareTransfersOptions<'a, 'b> {
    /// Optional inputs to use if you're sending iota
    pub inputs: Option<Inputs>,
    /// Optional remainder address to use, if not provided, one will be generated
    pub remainder_address: Option<&'a str>,
    /// Security to use when generating addresses (1-3)
    pub security: usize,
    /// Optional key to use if you want to hmac the transfers
    pub hmac_key: Option<&'b str>,
}

impl<'a, 'b> Default for PrepareTransfersOptions<'a, 'b> {
    fn default() -> Self {
        PrepareTransfersOptions {
            inputs: None,
            remainder_address: None,
            security: 3,
            hmac_key: None,
        }
    }
}

/// AddRemainderOptions
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AddRemainderOptions<'a, 'b, 'c, 'd> {
    /// The tryte-encoded seed. It should be noted that this seed is not transferred.
    pub(crate) seed: &'a str,
    /// The tag to add to each bundle entry
    pub(crate) tag: &'b str,
    /// The address used for sending the remainder value (of the last input)
    pub(crate) remainder_address: Option<&'c str>,
    /// The signature fragments (message), used for signing. Should be 2187 characters long, can be padded with 9s.
    pub(crate) signature_fragments: Vec<String>,
    /// Check if hmac is added
    pub(crate) added_hmac: bool,
    /// Optional key to use if you want to hmac the transfers
    pub(crate) hmac_key: Option<&'d str>,
    /// Security to use when generating addresses (1-3)
    pub(crate) security: usize,
}
