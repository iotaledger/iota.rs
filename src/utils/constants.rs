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
const SEED_LENGTH_MAX: i32 = 81;
const ADDRESS_LENGTH_WITHOUT_CHECKSUM: i32 = 81;
const ADDRESS_LENGTH_WITH_CHECKSUM: i32 = 90;
const MESSAGE_LENGTH: i32 = 2187;
const TAG_LENGTH: i32 = 27;

const INVALID_TRYTES_INPUT_ERROR: &str = "Invalid trytes provided.";
const INVALID_HASHES_INPUT_ERROR: &str = "Invalid hashes provided.";
const INVALID_TAIL_HASH_INPUT_ERROR: &str = "Invalid tail hash provided.";
const INVALID_SEED_INPUT_ERROR: &str = "Invalid seed provided.";
const INVALID_SECURITY_LEVEL_INPUT_ERROR: &str = "Invalid security level provided.";
const INVALID_ATTACHED_TRYTES_INPUT_ERROR: &str = "Invalid attached trytes provided.";
const INVALID_TRANSFERS_INPUT_ERROR: &str = "Invalid transfers provided.";
const INVALID_ADDRESSES_INPUT_ERROR: &str = "Invalid addresses provided.";
const INVALID_INPUT_ERROR: &str = "Invalid input provided.";

const INVALID_BUNDLE_ERROR: &str = "Invalid bundle.";
const INVALID_BUNDLE_SUM_ERROR: &str = "Invalid bundle sum.";
const INVALID_BUNDLE_HASH_ERROR: &str = "Invalid bundle hash.";
const INVALID_SIGNATURES_ERROR: &str = "Invalid signatures.";
const INVALID_VALUE_TRANSFER_ERROR: &str =
    "Invalid value transfer: the transfer does not require a signature.";

const NOT_ENOUGH_BALANCE_ERROR: &str = "Not enough balance.";
const NO_REMAINDER_ADDRESS_ERROR: &str = "No remainder address defined.";

const GET_TRYTES_RESPONSE_ERROR: &str = "Get trytes response was null.";
const GET_BUNDLE_RESPONSE_ERROR: &str = "Get bundle response was null.";
const GET_INCLUSION_STATE_RESPONSE_ERROR: &str = "Get inclusion state response was null.";

const SENDING_TO_USED_ADDRESS_ERROR: &str = "Sending to a used address.";
const PRIVATE_KEY_REUSE_ERROR: &str = "Private key reuse detect!";
const SEND_TO_INPUTS_ERROR: &str = "Send to inputs!";
