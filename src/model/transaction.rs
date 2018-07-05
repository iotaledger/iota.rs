use crate::crypto::{Curl, Sponge};
use crate::utils::{self, converter};
use failure::Error;
use serde_json;
use std::fmt;
use std::str::FromStr;

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
    pub fn attachment_timestamp_lower_bound(&self) -> Option<i64> {
        self.attachment_timestamp_lower_bound.clone()
    }
    pub fn attachment_timestamp_lower_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_lower_bound
    }
    pub fn attachment_timestamp_upper_bound(&self) -> Option<i64> {
        self.attachment_timestamp_upper_bound.clone()
    }
    pub fn attachment_timestamp_upper_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_upper_bound
    }
    pub fn hash(&self) -> Option<String> {
        self.hash.clone()
    }
    pub fn hash_mut(&mut self) -> &mut Option<String> {
        &mut self.hash
    }
    pub fn signature_fragments(&self) -> Option<String> {
        self.signature_fragments.clone()
    }
    pub fn signature_fragments_mut(&mut self) -> &mut Option<String> {
        &mut self.signature_fragments
    }
    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }
    pub fn address_mut(&mut self) -> &mut Option<String> {
        &mut self.address
    }
    pub fn value(&self) -> Option<i64> {
        self.value
    }
    pub fn value_mut(&mut self) -> &mut Option<i64> {
        &mut self.value
    }
    pub fn tag(&self) -> Option<String> {
        self.tag.clone()
    }
    pub fn tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }
    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp
    }
    pub fn timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.timestamp
    }
    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }
    pub fn current_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.current_index
    }
    pub fn last_index(&self) -> Option<usize> {
        self.last_index
    }
    pub fn last_index_mut(&mut self) -> &mut Option<usize> {
        &mut self.last_index
    }
    pub fn bundle(&self) -> Option<String> {
        self.bundle.clone()
    }
    pub fn bundle_mut(&mut self) -> &mut Option<String> {
        &mut self.bundle
    }
    pub fn trunk_transaction(&self) -> Option<String> {
        self.trunk_transaction.clone()
    }
    pub fn trunk_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.trunk_transaction
    }
    pub fn branch_transaction(&self) -> Option<String> {
        self.branch_transaction.clone()
    }
    pub fn branch_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.branch_transaction
    }
    pub fn nonce(&self) -> Option<String> {
        self.nonce.clone()
    }
    pub fn nonce_mut(&mut self) -> &mut Option<String> {
        &mut self.nonce
    }
    pub fn persistence(&self) -> Option<bool> {
        self.persistence
    }
    pub fn persistence_mut(&mut self) -> &mut Option<bool> {
        &mut self.persistence
    }
    pub fn obsolete_tag(&self) -> Option<String> {
        self.obsolete_tag.clone()
    }
    pub fn obsolete_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.obsolete_tag
    }
    pub fn attachment_timestamp(&self) -> Option<i64> {
        self.attachment_timestamp.clone()
    }
    pub fn attachment_timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp
    }

    pub fn to_trytes(&self) -> String {
        let mut res = String::new();

        let mut value_trits = converter::trits(self.value.unwrap_or_default());
        utils::right_pad_vec(&mut value_trits, 81, 0);

        let mut timestamp_trits = converter::trits(self.timestamp.unwrap_or_default());
        utils::right_pad_vec(&mut timestamp_trits, 27, 0);

        let mut current_index_trits =
            converter::trits(self.current_index.unwrap_or_default() as i64);
        utils::right_pad_vec(&mut current_index_trits, 27, 0);
        let mut last_index_trits = converter::trits(self.last_index.unwrap_or_default() as i64);
        utils::right_pad_vec(&mut last_index_trits, 27, 0);

        if let Some(current_index) = self.current_index {
            res += &converter::trits_to_string(&converter::trits(current_index as i64)[0..27])
                .unwrap();
        }
        if let Some(last_index) = self.last_index {
            res +=
                &converter::trits_to_string(&converter::trits(last_index as i64)[0..27]).unwrap();
        }

        if let Some(signature_fragments) = &self.signature_fragments {
            res += &signature_fragments;
        }
        if let Some(address) = &self.address {
            res += &address;
        }

        if let Some(obsolete_tag) = &self.obsolete_tag {
            res += &obsolete_tag;
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
            res += &converter::trits_to_string(&converter::trits(attachment_timestamp)[0..27])
                .unwrap();
        }
        if let Some(attachment_timestamp_lower_bound) = self.attachment_timestamp_lower_bound {
            res += &converter::trits_to_string(
                &converter::trits(attachment_timestamp_lower_bound)[0..27],
            ).unwrap();
        }
        if let Some(attachment_timestamp_upper_bound) = self.attachment_timestamp_upper_bound {
            res += &converter::trits_to_string(
                &converter::trits(attachment_timestamp_upper_bound)[0..27],
            ).unwrap();
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
    type Err = Error;

    fn from_str(trytes: &str) -> Result<Self, Self::Err> {
        ensure!(!trytes.is_empty(), TransactionParseError::TryteStringEmpty);
        for c in trytes.chars().skip(2279).take(16) {
            ensure!(c == '9', TransactionParseError::NineSectionMissing);
        }
        let transaction_trits = converter::trits_from_string(trytes);

        let mut hash = [0; 243];
        let mut curl = Curl::default();
        curl.reset();
        curl.absorb(&transaction_trits)?;
        curl.squeeze(&mut hash)?;

        let mut transaction = Transaction::default();
        *transaction.hash_mut() = Some(converter::trits_to_string(&hash)?);

        *transaction.signature_fragments_mut() = Some(trytes[0..2187].to_string());
        *transaction.address_mut() = Some(trytes[2187..2268].to_string());
        *transaction.value_mut() = Some(converter::long_value(&transaction_trits[6804..6837]));
        *transaction.obsolete_tag_mut() = Some(trytes[2295..2322].to_string());
        *transaction.timestamp_mut() = Some(converter::long_value(&transaction_trits[6966..6993]));
        *transaction.current_index_mut() =
            Some(converter::long_value(&transaction_trits[6993..7020]) as usize);
        *transaction.last_index_mut() =
            Some(converter::long_value(&transaction_trits[7020..7047]) as usize);
        *transaction.bundle_mut() = Some(trytes[2349..2430].to_string());
        *transaction.trunk_transaction_mut() = Some(trytes[2430..2511].to_string());
        *transaction.branch_transaction_mut() = Some(trytes[2511..2592].to_string());

        *transaction.tag_mut() = Some(trytes[2592..2619].to_string());
        *transaction.attachment_timestamp_mut() =
            Some(converter::long_value(&transaction_trits[7857..7884]));
        *transaction.attachment_timestamp_lower_bound_mut() =
            Some(converter::long_value(&transaction_trits[7884..7911]));
        *transaction.attachment_timestamp_upper_bound_mut() =
            Some(converter::long_value(&transaction_trits[7911..7938]));
        *transaction.nonce_mut() = Some(trytes[2646..2673].to_string());
        Ok(transaction)
    }
}
