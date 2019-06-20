use std::fmt;
use std::str::FromStr;

use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json;

use iota_conversion;
use iota_crypto::{Curl, Sponge};

use crate::Result;

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
    hash: Option<String>,
    signature_fragments: Option<String>,
    address: Option<String>,
    value: Option<i64>,
    obsolete_tag: Option<String>,
    timestamp: Option<i64>,
    current_index: Option<usize>,
    last_index: Option<usize>,
    bundle: Option<String>,
    trunk_transaction: Option<String>,
    branch_transaction: Option<String>,
    nonce: Option<String>,
    persistence: Option<bool>,
    tag: Option<String>,
    attachment_timestamp: Option<i64>,
    attachment_timestamp_lower_bound: Option<i64>,
    attachment_timestamp_upper_bound: Option<i64>,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Transaction {
    /// Provides a view of the attachment_timestamp
    pub fn attachment_timestamp(&self) -> Option<i64> {
        self.attachment_timestamp
    }
    /// Provides a mutable view of the attachment_timestamp
    pub fn attachment_timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_attachment_timestamp<T>(&mut self, new_value: T)
    where
        T: Into<i64>,
    {
        self.attachment_timestamp = Some(new_value.into());
    }

    /// Provides a view of the attachment_timestamp_lower_bound
    pub fn attachment_timestamp_lower_bound(&self) -> Option<i64> {
        self.attachment_timestamp_lower_bound
    }
    /// Provides a mutable view of the attachment_timestamp_lower_bound
    pub fn attachment_timestamp_lower_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_lower_bound
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_attachment_timestamp_lower_bound<T>(&mut self, new_value: T)
    where
        T: Into<i64>,
    {
        self.attachment_timestamp_lower_bound = Some(new_value.into());
    }

    /// Provides a view of the attachment_timestamp_upper_bound
    pub fn attachment_timestamp_upper_bound(&self) -> Option<i64> {
        self.attachment_timestamp_upper_bound
    }
    /// Provides a mutable view of the attachment_timestamp_upper_bound
    pub fn attachment_timestamp_upper_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_upper_bound
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_attachment_timestamp_upper_bound<T>(&mut self, new_value: T)
    where
        T: Into<i64>,
    {
        self.attachment_timestamp_upper_bound = Some(new_value.into());
    }

    /// Provides a view of the hash
    pub fn hash(&self) -> Option<String> {
        self.hash.clone()
    }
    /// Provides a mutable view of the hash
    pub fn hash_mut(&mut self) -> &mut Option<String> {
        &mut self.hash
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_hash<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.hash = Some(new_value.into());
    }

    /// Provides a view of the signature_fragments
    pub fn signature_fragments(&self) -> Option<String> {
        self.signature_fragments.clone()
    }
    /// Provides a mutable view of the signature_fragments
    pub fn signature_fragments_mut(&mut self) -> &mut Option<String> {
        &mut self.signature_fragments
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_signature_fragments<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.signature_fragments = Some(new_value.into());
    }

    /// Provides a view of the address
    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }
    /// Provides a mutable view of the address
    pub fn address_mut(&mut self) -> &mut Option<String> {
        &mut self.address
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_address<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.address = Some(new_value.into());
    }

    /// Provides a view of the value
    pub fn value(&self) -> Option<i64> {
        self.value
    }
    /// Provides a mutable view of the value
    pub fn value_mut(&mut self) -> &mut Option<i64> {
        &mut self.value
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_value<T>(&mut self, new_value: T)
    where
        T: Into<i64>,
    {
        self.value = Some(new_value.into());
    }

    /// Provides a view of the tag
    pub fn tag(&self) -> Option<String> {
        self.tag.clone()
    }
    /// Provides a mutable view of the tag
    pub fn tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_tag<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.tag = Some(new_value.into());
    }

    /// Provides a view of the timestamp
    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }
    /// Provides a mutable view of the timestamp
    pub fn timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.timestamp
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_timestamp<T>(&mut self, new_value: T)
    where
        T: Into<i64>,
    {
        self.timestamp = Some(new_value.into());
    }

    /// Provides a view of the current_index
    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }
    /// Provides a mutable view of the current_index
    pub fn current_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.current_index
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_current_index<T>(&mut self, new_value: T)
    where
        T: Into<usize>,
    {
        self.current_index = Some(new_value.into());
    }

    /// Provides a view of the last_index
    pub fn last_index(&self) -> Option<usize> {
        self.last_index
    }
    /// Provides a mutable view of the last_index
    pub fn last_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.last_index
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_last_index<T>(&mut self, new_value: T)
    where
        T: Into<usize>,
    {
        self.last_index = Some(new_value.into());
    }

    /// Provides a view of the bundle
    pub fn bundle(&self) -> Option<String> {
        self.bundle.clone()
    }
    /// Provides a mutable view of the bundle
    pub fn bundle_mut(&mut self) -> &mut Option<String> {
        &mut self.bundle
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_bundle<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.bundle = Some(new_value.into());
    }

    /// Provides a view of the trunk_transaction
    pub fn trunk_transaction(&self) -> Option<String> {
        self.trunk_transaction.clone()
    }
    /// Provides a mutable view of the trunk_transaction
    pub fn trunk_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.trunk_transaction
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_trunk_transaction<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.trunk_transaction = Some(new_value.into());
    }

    /// Provides a view of the branch_transaction
    pub fn branch_transaction(&self) -> Option<String> {
        self.branch_transaction.clone()
    }
    /// Provides a mutable view of the branch_transaction
    pub fn branch_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.branch_transaction
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_branch_transaction<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.branch_transaction = Some(new_value.into());
    }

    /// Provides a view of the nonce
    pub fn nonce(&self) -> Option<String> {
        self.nonce.clone()
    }
    /// Provides a mutable view of the nonce
    pub fn nonce_mut(&mut self) -> &mut Option<String> {
        &mut self.nonce
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_nonce<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.nonce = Some(new_value.into());
    }

    /// Provides a view of the persistence
    pub fn persistence(&self) -> Option<bool> {
        self.persistence
    }
    /// Provides a mutable view of the persistence
    pub fn persistence_mut(&mut self) -> &mut Option<bool> {
        &mut self.persistence
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_persistence<T>(&mut self, new_value: T)
    where
        T: Into<bool>,
    {
        self.persistence = Some(new_value.into());
    }

    /// Provides a view of the obsolete_tag
    pub fn obsolete_tag(&self) -> Option<String> {
        self.obsolete_tag.clone()
    }
    /// Provides a mutable view of the obsolete_tag
    pub fn obsolete_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.obsolete_tag
    }
    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_obsolete_tag<T>(&mut self, new_value: T)
    where
        T: Into<String>,
    {
        self.obsolete_tag = Some(new_value.into());
    }

    /// Converts the transaction into a tryte-encoded string
    /// This DOES NOT consume the transaction
    pub fn to_trytes(&self) -> String {
        let mut value_trits = iota_conversion::trits(self.value.unwrap_or_default());
        right_pad_vec(&mut value_trits, 81, 0);

        let mut timestamp_trits = iota_conversion::trits(self.timestamp.unwrap_or_default());
        right_pad_vec(&mut timestamp_trits, 27, 0);

        let mut current_index_trits =
            iota_conversion::trits(self.current_index.unwrap_or_default() as i64);
        right_pad_vec(&mut current_index_trits, 27, 0);

        let mut last_index_trits =
            iota_conversion::trits(self.last_index.unwrap_or_default() as i64);
        right_pad_vec(&mut last_index_trits, 27, 0);

        let mut attachment_timestamp_trits =
            iota_conversion::trits(self.attachment_timestamp.unwrap_or_default());
        right_pad_vec(&mut attachment_timestamp_trits, 27, 0);

        let mut attachment_timestamp_lower_bound_trits =
            iota_conversion::trits(self.attachment_timestamp_lower_bound.unwrap_or_default());
        right_pad_vec(&mut attachment_timestamp_lower_bound_trits, 27, 0);

        let mut attachment_timestamp_upper_bound_trits =
            iota_conversion::trits(self.attachment_timestamp_upper_bound.unwrap_or_default());
        right_pad_vec(&mut attachment_timestamp_upper_bound_trits, 27, 0);

        self.signature_fragments().unwrap_or_default()
            + &self.address().unwrap_or_default()
            + &iota_conversion::trytes(&value_trits)
            + &self.obsolete_tag().unwrap_or_default()
            + &iota_conversion::trytes(&timestamp_trits)
            + &iota_conversion::trytes(&current_index_trits)
            + &iota_conversion::trytes(&last_index_trits)
            + &self.bundle().unwrap_or_default()
            + &self.trunk_transaction().unwrap_or_default()
            + &self.branch_transaction().unwrap_or_default()
            + &self.tag().unwrap_or_default()
            + &iota_conversion::trytes(&attachment_timestamp_trits)
            + &iota_conversion::trytes(&attachment_timestamp_lower_bound_trits)
            + &iota_conversion::trytes(&attachment_timestamp_upper_bound_trits)
            + &self.nonce().unwrap_or_default()
    }
}

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
    type Err = Error;

    fn from_str(trytes: &str) -> Result<Self> {
        ensure!(!trytes.is_empty(), TransactionParseError::TryteStringEmpty);
        for c in trytes.chars().skip(2279).take(16) {
            ensure!(c == '9', TransactionParseError::NineSectionMissing);
        }
        let transaction_trits = iota_conversion::trits_from_string(trytes);

        let mut hash = [0; 243];
        let mut curl = Curl::default();
        curl.reset();
        curl.absorb(&transaction_trits)?;
        curl.squeeze(&mut hash)?;

        let mut transaction = Transaction::default();
        transaction.set_hash(iota_conversion::trits_to_string(&hash)?);
        transaction.set_signature_fragments(&trytes[0..2187]);
        transaction.set_address(&trytes[2187..2268]);
        transaction.set_value(iota_conversion::long_value(&transaction_trits[6804..6837]));
        transaction.set_obsolete_tag(&trytes[2295..2322]);
        transaction.set_timestamp(iota_conversion::long_value(&transaction_trits[6966..6993]));
        transaction
            .set_current_index(iota_conversion::long_value(&transaction_trits[6993..7020]) as usize);
        transaction
            .set_last_index(iota_conversion::long_value(&transaction_trits[7020..7047]) as usize);
        transaction.set_bundle(&trytes[2349..2430]);
        transaction.set_trunk_transaction(&trytes[2430..2511]);
        transaction.set_branch_transaction(&trytes[2511..2592]);

        transaction.set_tag(&trytes[2592..2619]);
        transaction
            .set_attachment_timestamp(iota_conversion::long_value(&transaction_trits[7857..7884]));
        transaction.set_attachment_timestamp_lower_bound(iota_conversion::long_value(
            &transaction_trits[7884..7911],
        ));
        transaction.set_attachment_timestamp_upper_bound(iota_conversion::long_value(
            &transaction_trits[7911..7938],
        ));
        transaction.set_nonce(&trytes[2646..2673]);
        Ok(transaction)
    }
}

impl Into<String> for Transaction {
    fn into(self) -> String {
        self.to_trytes()
    }
}
