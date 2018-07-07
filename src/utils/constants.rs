/// A char array holding all acceptable characters in the tryte
/// alphabet. Used because strings can't be cheaply indexed in rust.
pub const TRYTE_ALPHABET: [char; 27] = [
    '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// The minimum value a trit can have
pub const MIN_TRIT_VALUE: i8 = -1;
/// The maximum value a trit can have
pub const MAX_TRIT_VALUE: i8 = 1;
/// The minimum value a tryte can have
pub const MIN_TRYTE_VALUE: i8 = -13;
/// The maximum value a tryte can have
pub const MAX_TRYTE_VALUE: i8 = 13;

/// The number of trits in a byte
pub const TRITS_PER_BYTE: usize = 5;
/// The number of trits in a tryte
pub const TRITS_PER_TRYTE: usize = 3;
/// The maximum allowed seed length
pub const SEED_LENGTH_MAX: usize = 81;
/// The length of an address without a checksum
pub const ADDRESS_LENGTH_WITHOUT_CHECKSUM: usize = 81;
/// The length of an address with a checksum
pub const ADDRESS_LENGTH_WITH_CHECKSUM: usize = 90;
/// The mandatory length of a message segment
pub const MESSAGE_LENGTH: usize = 2187;
/// The mandatory length of a tag segment
pub const TAG_LENGTH: usize = 27;

/// INVALID_TRYTES_INPUT_ERROR
pub const INVALID_TRYTES_INPUT_ERROR: &str = "Invalid trytes provided.";
/// INVALID_HASHES_INPUT_ERROR
pub const INVALID_HASHES_INPUT_ERROR: &str = "Invalid hashes provided.";
/// INVALID_TAIL_HASH_INPUT_ERROR
pub const INVALID_TAIL_HASH_INPUT_ERROR: &str = "Invalid tail hash provided.";
/// INVALID_SEED_INPUT_ERROR
pub const INVALID_SEED_INPUT_ERROR: &str = "Invalid seed provided.";
/// INVALID_SECURITY_LEVEL_INPUT_ERROR
pub const INVALID_SECURITY_LEVEL_INPUT_ERROR: &str = "Invalid security level provided.";
/// INVALID_ATTACHED_TRYTES_INPUT_ERROR
pub const INVALID_ATTACHED_TRYTES_INPUT_ERROR: &str = "Invalid attached trytes provided.";
/// INVALID_TRANSFERS_INPUT_ERROR
pub const INVALID_TRANSFERS_INPUT_ERROR: &str = "Invalid transfers provided.";
/// INVALID_ADDRESSES_INPUT_ERROR
pub const INVALID_ADDRESSES_INPUT_ERROR: &str = "Invalid addresses provided.";
/// INVALID_INPUT_ERROR
pub const INVALID_INPUT_ERROR: &str = "Invalid input provided.";

/// INVALID_BUNDLE_ERROR
pub const INVALID_BUNDLE_ERROR: &str = "Invalid bundle.";
/// INVALID_BUNDLE_SUM_ERROR
pub const INVALID_BUNDLE_SUM_ERROR: &str = "Invalid bundle sum.";
/// INVALID_BUNDLE_HASH_ERROR
pub const INVALID_BUNDLE_HASH_ERROR: &str = "Invalid bundle hash.";
/// INVALID_SIGNATURES_ERROR
pub const INVALID_SIGNATURES_ERROR: &str = "Invalid signatures.";
/// INVALID_VALUE_TRANSFER_ERROR
pub const INVALID_VALUE_TRANSFER_ERROR: &str =
    "Invalid value transfer: the transfer does not require a signature.";

/// NOT_ENOUGH_BALANCE_ERROR
pub const NOT_ENOUGH_BALANCE_ERROR: &str = "Not enough balance.";
/// NO_REMAINDER_ADDRESS_ERROR
pub const NO_REMAINDER_ADDRESS_ERROR: &str = "No remainder address defined.";

/// GET_TRYTES_RESPONSE_ERROR
pub const GET_TRYTES_RESPONSE_ERROR: &str = "Get trytes response was null.";
/// GET_BUNDLE_RESPONSE_ERROR
pub const GET_BUNDLE_RESPONSE_ERROR: &str = "Get bundle response was null.";
/// GET_INCLUSION_STATE_RESPONSE_ERROR
pub const GET_INCLUSION_STATE_RESPONSE_ERROR: &str = "Get inclusion state response was null.";

/// SENDING_TO_USED_ADDRESS_ERROR
pub const SENDING_TO_USED_ADDRESS_ERROR: &str = "Sending to a used address.";
/// PRIVATE_KEY_REUSE_ERROR
pub const PRIVATE_KEY_REUSE_ERROR: &str = "Private key reuse detect!";
/// SEND_TO_INPUTS_ERROR
pub const SEND_TO_INPUTS_ERROR: &str = "Send to inputs!";
