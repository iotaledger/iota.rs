#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Constants used throughout iota related crates

/// A char array holding all acceptable characters in the tryte
/// alphabet. Used because strings can't be cheaply indexed in rust.
pub const TRYTE_ALPHABET: [char; 27] = [
    '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Default Minimum Weight Magnitude on IOTA mainnet
pub const DEFAULT_MWM: usize = 14;

/// The minimum value a trit can have
pub const MIN_TRIT_VALUE: i8 = -1;
/// The maximum value a trit can have
pub const MAX_TRIT_VALUE: i8 = 1;
/// The minimum value a tryte can have
pub const MIN_TRYTE_VALUE: i8 = -13;
/// The maximum value a tryte can have
pub const MAX_TRYTE_VALUE: i8 = 13;

/// Radix of ternary numeral system
pub const TRINARY_RADIX: usize = 3;

/// Size of hash in trits
pub const HASH_TRINARY_SIZE: usize = 243;
/// Size of hash in trytes
pub const HASH_TRYTES_SIZE: usize = HASH_TRINARY_SIZE / 3;
/// Size of hash in bytes
pub const HASH_BYTES_SIZE: usize = 48;
/// Size of key segment per fragment
pub const KEY_SEGMENTS_PER_FRAGMENT: usize = 27;
/// Round of key segment hash
pub const KEY_SEGMENT_HASH_ROUNDS: usize = 26;
/// Lenght of key fragment which is 6561
pub const KEY_FRAGMENT_LENGTH: usize = HASH_TRINARY_SIZE * KEY_SEGMENTS_PER_FRAGMENT;

/// Size of address checksum in trytes
pub const ADDRESS_CHECKSUM_TRYTES_SIZE: usize = 9;
/// Size of address with checksum in trytes
pub const ADDRESS_WITH_CHECKSUM_TRYTES_SIZE: usize =
    HASH_TRYTES_SIZE + ADDRESS_CHECKSUM_TRYTES_SIZE;
/// Size of minimum checksum
pub const MIN_CHECKSUM_TRYTES_SIZE: usize = 3;

/// Maximum value of attachment timstamp
pub const UPPER_BOUND_ATTACHMENT_TIMESTAMP: usize = (3 ^ 27 - 1) / 2;
/// Minimum value of attachment timstamp
pub const LOWER_BOUND_ATTACHMENT_TIMESTAMP: usize = 0;

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

/// Size of signature message fragment in trtis
pub const SIGNATURE_MESSAGE_FRAGMENT_TRINARY_SIZE: usize = 6561;
/// Size of address in trtis
pub const ADDRESS_TRINARY_SIZE: usize = 243;
/// Size of value in trtis
pub const VALUE_SIZE_TRINARY: usize = 81;
/// Size of obselte tag in trtis
pub const OBSOLETE_TAG_TRINARY_SIZE: usize = 81;
/// Size of timestamp in trtis
pub const TIMESTAMP_TRINARY_SIZE: usize = 27;
/// Size of current index in trtis
pub const CURRENT_INDEX_TRINARY_SIZE: usize = 27;
/// Size of last index trinary in trtis
pub const LAST_INDEX_TRINARY_SIZE: usize = 27;
/// Size of bundle hash in trtis
pub const BUNDLE_TRINARY_SIZE: usize = 243;
/// Size of trunk transaction hash in trtis
pub const TRUNK_TRANSACTION_TRINARY_SIZE: usize = 243;
/// Size of brnach transaction hash in trtis
pub const BRANCH_TRANSACTION_TRINARY_SIZE: usize = 243;
/// Size of tag in trtis
pub const TAG_TRINARY_SIZE: usize = 81;
/// Size of attachment timestamp in trtis
pub const ATTACHMENT_TIMESTAMP_TRINARY_SIZE: usize = 27;
/// Size of upper bound attachment timestamp in trtis
pub const ATTACHMENT_TIMESTAMP_LOWER_BOUND_TRINARY_SIZE: usize = 27;
/// Size of lower bound attachment timestamp in trtis
pub const ATTACHMENT_TIMESTAMP_UPPER_BOUND_TRINARY_SIZE: usize = 27;
/// Size of nonce in trtis
pub const NONCE_TRINARY_SIZE: usize = 81;

/// Size of a whole transaction object in trits
pub const TRANSACTION_TRINARY_SIZE: usize = SIGNATURE_MESSAGE_FRAGMENT_TRINARY_SIZE
    + ADDRESS_TRINARY_SIZE
    + VALUE_SIZE_TRINARY
    + OBSOLETE_TAG_TRINARY_SIZE
    + TIMESTAMP_TRINARY_SIZE
    + CURRENT_INDEX_TRINARY_SIZE
    + LAST_INDEX_TRINARY_SIZE
    + BUNDLE_TRINARY_SIZE
    + TRUNK_TRANSACTION_TRINARY_SIZE
    + BRANCH_TRANSACTION_TRINARY_SIZE
    + TAG_TRINARY_SIZE
    + ATTACHMENT_TIMESTAMP_TRINARY_SIZE
    + ATTACHMENT_TIMESTAMP_LOWER_BOUND_TRINARY_SIZE
    + ATTACHMENT_TIMESTAMP_UPPER_BOUND_TRINARY_SIZE
    + NONCE_TRINARY_SIZE;

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

/// Security levels of a private key and address pair. The greater the security
/// level, the larger and more secure the signature of a spent address is
/// against brute force attacks.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SecurityLevel {
    /// lowest security
    Low = 1,
    /// used by Trinity
    Medium = 2,
    /// most secure
    High = 3,
}

/// Number of Iota Signature Scheme fragments
pub const ISS_FRAGMENTS: usize = 27;
/// Length of Iota Signature Scheme key
pub const ISS_KEY_LENGTH: usize = HASH_TRINARY_SIZE * ISS_FRAGMENTS;
/// Lenght of Iota Signature Scheme chunk
pub const ISS_CHUNK_LENGTH: usize = HASH_TRINARY_SIZE / TRINARY_RADIX;
