use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::Result;
use iota_conversion::Trinary;
use iota_crypto::{Curl, Sponge};

/// Right pads a string to a certain length in place
///
/// * `x` - the string to be padded
/// * `len` - the target length of the string
/// * `pad` - the char to pad with
pub fn right_pad_string(x: &mut String, len: usize, pad: char) {
    while x.len() < len {
        x.push(pad);
    }
}

/// Right pads a vector to a certain length in place
///
/// * `x` - the vec to be padded
/// * `len` - the target length of the string`
/// * `pad` - the element to pad with
pub fn right_pad_vec<T>(x: &mut Vec<T>, len: usize, pad: T)
where
    T: Copy,
{
    while x.len() < len {
        x.push(pad);
    }
}

/// Represents an IOTA transaction
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub signature_fragments: String,
    pub address: String,
    pub value: i64,
    pub obsolete_tag: String,
    pub timestamp: i64,
    pub current_index: usize,
    pub last_index: usize,
    pub bundle: String,
    pub trunk_transaction: String,
    pub branch_transaction: String,
    pub nonce: String,
    pub persistence: bool,
    pub tag: String,
    pub attachment_timestamp: i64,
    pub attachment_timestamp_lower_bound: i64,
    pub attachment_timestamp_upper_bound: i64,
}

// impl fmt::Display for Transaction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", serde_json::to_string_pretty(self))
//     }
// }

/// This type provides some errors that can occur
#[derive(Copy, Clone, Debug, Fail)]
pub enum TransactionParseError {
    /// This error occurs if the string being parsed is empty
    #[fail(display = "tryte string is empty")]
    TryteStringEmpty,
    /// This error occurs when the block of sixteen 9s at index 2279
    /// isn't present in the provided string
    #[fail(display = "Should be sixteen 9's at index 2279")]
    NineSectionMissing,
}

impl FromStr for Transaction {
    type Err = failure::Error;

    fn from_str(trytes: &str) -> Result<Self> {
        ensure!(!trytes.is_empty(), TransactionParseError::TryteStringEmpty);
        for c in trytes.chars().skip(2279).take(16) {
            ensure!(c == '9', TransactionParseError::NineSectionMissing);
        }
        let transaction_trits = trytes.trits();

        let mut hash = [0; 243];
        let mut curl = Curl::default();
        curl.reset();
        curl.absorb(&transaction_trits)?;
        curl.squeeze(&mut hash)?;

        let mut transaction = Transaction::default();
        transaction.hash = hash.trytes()?;
        transaction.signature_fragments = trytes[0..2187].into();
        transaction.address = trytes[2187..2268].into();
        transaction.value = iota_conversion::long_value(&transaction_trits[6804..6837]);
        transaction.obsolete_tag = trytes[2295..2322].into();
        transaction.timestamp = iota_conversion::long_value(&transaction_trits[6966..6993]);
        transaction.current_index =
            iota_conversion::long_value(&transaction_trits[6993..7020]) as usize;
        transaction.last_index =
            iota_conversion::long_value(&transaction_trits[7020..7047]) as usize;
        transaction.bundle = trytes[2349..2430].into();
        transaction.trunk_transaction = trytes[2430..2511].into();
        transaction.branch_transaction = trytes[2511..2592].into();

        transaction.tag = trytes[2592..2619].into();
        transaction.attachment_timestamp =
            iota_conversion::long_value(&transaction_trits[7857..7884]);
        transaction.attachment_timestamp_lower_bound =
            iota_conversion::long_value(&transaction_trits[7884..7911]);
        transaction.attachment_timestamp_upper_bound =
            iota_conversion::long_value(&transaction_trits[7911..7938]);
        transaction.nonce = trytes[2646..2673].into();
        Ok(transaction)
    }
}

impl TryInto<String> for Transaction {
    type Error = failure::Error;

    fn try_into(self) -> Result<String> {
        to_string(&self)
    }
}

impl TryInto<String> for &Transaction {
    type Error = failure::Error;

    fn try_into(self) -> Result<String> {
        to_string(self)
    }
}

fn to_string(tx: &Transaction) -> Result<String> {
    let mut value_trits = tx.value.trits();
    right_pad_vec(&mut value_trits, 81, 0);

    let mut timestamp_trits = tx.timestamp.trits();
    right_pad_vec(&mut timestamp_trits, 27, 0);

    let mut current_index_trits = (tx.current_index as i64).trits();
    right_pad_vec(&mut current_index_trits, 27, 0);

    let mut last_index_trits = (tx.last_index as i64).trits();
    right_pad_vec(&mut last_index_trits, 27, 0);

    let mut attachment_timestamp_trits = tx.attachment_timestamp.trits();
    right_pad_vec(&mut attachment_timestamp_trits, 27, 0);

    let mut attachment_timestamp_lower_bound_trits = tx.attachment_timestamp_lower_bound.trits();
    right_pad_vec(&mut attachment_timestamp_lower_bound_trits, 27, 0);

    let mut attachment_timestamp_upper_bound_trits = tx.attachment_timestamp_upper_bound.trits();
    right_pad_vec(&mut attachment_timestamp_upper_bound_trits, 27, 0);

    Ok(tx.signature_fragments.clone()
        + &tx.address
        + &value_trits.trytes()?
        + &tx.obsolete_tag
        + &timestamp_trits.trytes()?
        + &current_index_trits.trytes()?
        + &last_index_trits.trytes()?
        + &tx.bundle
        + &tx.trunk_transaction
        + &tx.branch_transaction
        + &tx.tag
        + &attachment_timestamp_trits.trytes()?
        + &attachment_timestamp_lower_bound_trits.trytes()?
        + &attachment_timestamp_upper_bound_trits.trytes()?
        + &tx.nonce)
}
