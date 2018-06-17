use pow::curl;
use pow::traits::ICurl;
use std::str::FromStr;
use utils::converter::{long_value, trits, trits_from_string, trits_to_string};
use std::fmt;
use serde_json;

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
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
    attachment_timestamp: Option<i64>,
    tag: Option<String>,
    attachment_timestamp_lower_bound: Option<i64>,
    attachment_timestamp_upper_bound: Option<i64>,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap_or_default())
    }
}

impl Transaction {
    pub fn attachment_timestamp_lower_bound(&self) -> &Option<i64> {
        &self.attachment_timestamp_lower_bound
    }
    pub fn attachment_timestamp_lower_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_lower_bound
    }
    pub fn attachment_timestamp_upper_bound(&self) -> &Option<i64> {
        &self.attachment_timestamp_upper_bound
    }
    pub fn attachment_timestamp_upper_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_upper_bound
    }
    pub fn hash(&self) -> &Option<String> {
        &self.hash
    }
    pub fn hash_mut(&mut self) -> &mut Option<String> {
        &mut self.hash
    }
    pub fn signature_fragments(&self) -> &Option<String> {
        &self.signature_fragments
    }
    pub fn signature_fragments_mut(&mut self) -> &mut Option<String> {
        &mut self.signature_fragments
    }
    pub fn address(&self) -> &Option<String> {
        &self.address
    }
    pub fn address_mut(&mut self) -> &mut Option<String> {
        &mut self.address
    }
    pub fn value(&self) -> &Option<i64> {
        &self.value
    }
    pub fn value_mut(&mut self) -> &mut Option<i64> {
        &mut self.value
    }
    pub fn tag(&self) -> &Option<String> {
        &self.tag
    }
    pub fn tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }
    pub fn timestamp(&self) -> &Option<i64> {
        &self.timestamp
    }
    pub fn timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.timestamp
    }
    pub fn current_index(&self) -> &Option<usize> {
        &self.current_index
    }
    pub fn current_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.current_index
    }
    pub fn last_index(&self) -> &Option<usize> {
        &self.last_index
    }
    pub fn last_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.last_index
    }
    pub fn bundle(&self) -> &Option<String> {
        &self.bundle
    }
    pub fn bundle_mut(&mut self) -> &mut Option<String> {
        &mut self.bundle
    }
    pub fn trunk_transaction(&self) -> &Option<String> {
        &self.trunk_transaction
    }
    pub fn trunk_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.trunk_transaction
    }
    pub fn branch_transaction(&self) -> &Option<String> {
        &self.branch_transaction
    }
    pub fn branch_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.branch_transaction
    }
    pub fn nonce(&self) -> &Option<String> {
        &self.nonce
    }
    pub fn nonce_mut(&mut self) -> &mut Option<String> {
        &mut self.nonce
    }
    pub fn persistence(&self) -> &Option<bool> {
        &self.persistence
    }
    pub fn persistence_mut(&mut self) -> &mut Option<bool> {
        &mut self.persistence
    }
    pub fn obsolete_tag(&self) -> &Option<String> {
        &self.obsolete_tag
    }
    pub fn obsolete_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.obsolete_tag
    }
    pub fn attachment_timestamp(&self) -> &Option<i64> {
        &self.attachment_timestamp
    }
    pub fn attachment_timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp
    }
    pub fn to_trytes(&self) -> String {
        let mut res = String::new();
        if let Some(signature_fragments) = &self.signature_fragments {
            res += &signature_fragments;
        }
        if let Some(address) = &self.address {
            res += &address;
        }
        if let Some(value) = self.value {
            res += &trits_to_string(&trits(value)[0..81]).unwrap();
        }
        if let Some(obsolete_tag) = &self.obsolete_tag {
            res += &obsolete_tag;
        }
        if let Some(timestamp) = self.timestamp {
            res += &trits_to_string(&trits(timestamp)[0..27]).unwrap();
        }
        if let Some(current_index) = self.current_index {
            res += &trits_to_string(&trits(current_index as i64)[0..27]).unwrap();
        }
        if let Some(last_index) = self.last_index {
            res += &trits_to_string(&trits(last_index as i64)[0..27]).unwrap();
        }
        if let Some(bundle) = &self.bundle {
            res += &bundle;
        }
        if let Some(trunk_transaction) = &self.trunk_transaction {
            res += &trunk_transaction;
        }
        if let Some(branch_transaction) = &self.branch_transaction {
            res += &branch_transaction;
        }
        if let Some(tag) = &self.tag {
            res += &tag;
        }
        if let Some(attachment_timestamp) = self.attachment_timestamp {
            res += &trits_to_string(&trits(attachment_timestamp)[0..27]).unwrap();
        }
        if let Some(attachment_timestamp_lower_bound) = self.attachment_timestamp_lower_bound {
            res += &trits_to_string(&trits(attachment_timestamp_lower_bound)[0..27]).unwrap();
        }
        if let Some(attachment_timestamp_upper_bound) = self.attachment_timestamp_upper_bound {
            res += &trits_to_string(&trits(attachment_timestamp_upper_bound)[0..27]).unwrap();
        }
        res
    }
}

#[derive(Debug, Fail)]
pub enum TransactionParseError {
    #[fail(display = "tryte string is empty")]
    TryteStringEmpty,
    #[fail(display = "Should be sixteen 9's at index 2279")]
    NineSectionMissing,
}

impl FromStr for Transaction {
    type Err = TransactionParseError;

    fn from_str(trytes: &str) -> Result<Self, Self::Err> {
        if trytes.is_empty() {
            return Err(TransactionParseError::TryteStringEmpty);
        }
        for i in 2279..2295 {
            if trytes.chars().nth(i).unwrap() != '9' {
                return Err(TransactionParseError::NineSectionMissing);
            }
        }
        let mut transaction_trits = trits_from_string(trytes);
        let mut hash = [0; 243];

        let mut transaction = Transaction::default();
        let mut curl = curl::Curl::default();
        curl.reset();
        curl.absorb(&mut transaction_trits);
        curl.squeeze(&mut hash);

        *transaction.hash_mut() = Some(trits_to_string(&hash).unwrap());
        *transaction.signature_fragments_mut() = Some(trytes[0..2187].to_string());
        *transaction.address_mut() = Some(trytes[2187..2268].to_string());
        *transaction.value_mut() = Some(long_value(&transaction_trits[6804..6837]));
        *transaction.obsolete_tag_mut() = Some(trytes[2295..2322].to_string());
        *transaction.timestamp_mut() = Some(long_value(&transaction_trits[6966..6993]));
        *transaction.current_index_mut() =
            Some(long_value(&transaction_trits[6993..7020]) as usize);
        *transaction.last_index_mut() = Some(long_value(&transaction_trits[7020..7047]) as usize);
        *transaction.bundle_mut() = Some(trytes[2349..2430].to_string());
        *transaction.trunk_transaction_mut() = Some(trytes[2430..2511].to_string());
        *transaction.branch_transaction_mut() = Some(trytes[2511..2592].to_string());
        *transaction.tag_mut() = Some(trytes[2592..2619].to_string());
        *transaction.attachment_timestamp_mut() = Some(long_value(&transaction_trits[7857..7884]));
        *transaction.attachment_timestamp_lower_bound_mut() =
            Some(long_value(&transaction_trits[7884..7911]));
        *transaction.attachment_timestamp_upper_bound_mut() =
            Some(long_value(&transaction_trits[7911..7938]));
        *transaction.nonce_mut() = Some(trytes[2646..2673].to_string());
        Ok(transaction)
    }
}
