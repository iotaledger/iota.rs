use ascii::AsciiStr;
lazy_static! {
    pub static ref TRYTE_ALPHABET_ASCII: &'static AsciiStr =
        AsciiStr::from_ascii("9ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
}

pub const TRYTE_ALPHABET: [char; 27] = [
    '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub const TRITS_PER_BYTE: usize = 5;
pub const TRITS_PER_TRYTE: usize = 3;
pub const SEED_LENGTH_MAX: usize = 81;
pub const ADDRESS_LENGTH_WITHOUT_CHECKSUM: usize = 81;
pub const ADDRESS_LENGTH_WITH_CHECKSUM: usize = 90;
pub const MESSAGE_LENGTH: usize = 2187;
pub const TAG_LENGTH: usize = 27;

pub const INVALID_TRYTES_INPUT_ERROR: &str = "Invalid trytes provided.";
pub const INVALID_HASHES_INPUT_ERROR: &str = "Invalid hashes provided.";
pub const INVALID_TAIL_HASH_INPUT_ERROR: &str = "Invalid tail hash provided.";
pub const INVALID_SEED_INPUT_ERROR: &str = "Invalid seed provided.";
pub const INVALID_SECURITY_LEVEL_INPUT_ERROR: &str = "Invalid security level provided.";
pub const INVALID_ATTACHED_TRYTES_INPUT_ERROR: &str = "Invalid attached trytes provided.";
pub const INVALID_TRANSFERS_INPUT_ERROR: &str = "Invalid transfers provided.";
pub const INVALID_ADDRESSES_INPUT_ERROR: &str = "Invalid addresses provided.";
pub const INVALID_INPUT_ERROR: &str = "Invalid input provided.";

pub const INVALID_BUNDLE_ERROR: &str = "Invalid bundle.";
pub const INVALID_BUNDLE_SUM_ERROR: &str = "Invalid bundle sum.";
pub const INVALID_BUNDLE_HASH_ERROR: &str = "Invalid bundle hash.";
pub const INVALID_SIGNATURES_ERROR: &str = "Invalid signatures.";
pub const INVALID_VALUE_TRANSFER_ERROR: &str =
    "Invalid value transfer: the transfer does not require a signature.";

pub const NOT_ENOUGH_BALANCE_ERROR: &str = "Not enough balance.";
pub const NO_REMAINDER_ADDRESS_ERROR: &str = "No remainder address defined.";

pub const GET_TRYTES_RESPONSE_ERROR: &str = "Get trytes response was null.";
pub const GET_BUNDLE_RESPONSE_ERROR: &str = "Get bundle response was null.";
pub const GET_INCLUSION_STATE_RESPONSE_ERROR: &str = "Get inclusion state response was null.";

pub const SENDING_TO_USED_ADDRESS_ERROR: &str = "Sending to a used address.";
pub const PRIVATE_KEY_REUSE_ERROR: &str = "Private key reuse detect!";
pub const SEND_TO_INPUTS_ERROR: &str = "Send to inputs!";
