use pow::traits::ICurl;
use pow::curl;
use pow::kerl::Kerl;
use utils::converter::{trits, trits_to_string, trits_from_string, long_value}; 
use std::str::FromStr;
use std::error;
use std::fmt;

#[derive(Default, Clone)]
pub struct Transaction<C: ICurl> {
    customCurl: Option<C>,
    hash: Option<String>,
    signature_fragments: Option<String>,
    address: Option<String>,
    value: Option<i64>,
    obsolete_tag: Option<String>,
    timestamp: Option<i64>,
    current_index: Option<i64>,
    last_index: Option<i64>,
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

impl Transaction<Kerl> {
    fn attachment_timestamp_lower_bound(&self) -> &Option<i64> {
        &self.attachment_timestamp_lower_bound
    }
    fn attachment_timestamp_lower_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_lower_bound
    }
    fn attachment_timestamp_upper_bound(&self) -> &Option<i64> {
        &self.attachment_timestamp_upper_bound
    }
    fn attachment_timestamp_upper_bound_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp_upper_bound
    }
    fn hash(&self) -> &Option<String> {
        &self.hash
    }
    fn hash_mut(&mut self) -> &mut Option<String> {
        &mut self.hash
    }
    fn signature_fragments(&self) -> &Option<String> {
        &self.signature_fragments
    }
    fn signature_fragments_mut(&mut self) -> &mut Option<String> {
        &mut self.signature_fragments
    }
    fn address(&self) -> &Option<String> {
        &self.address
    }
    fn address_mut(&mut self) -> &mut Option<String> {
        &mut self.address
    }
    fn value(&self) -> &Option<i64> {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Option<i64> {
        &mut self.value
    }
    fn tag(&self) -> &Option<String> {
        &self.tag
    }
    fn tag_mut(&mut self) -> &mut Option<String> {
        &mut self.tag
    }
    fn timestamp(&self) -> &Option<i64> {
        &self.timestamp
    }
    fn timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.timestamp
    }
    fn current_index(&self) -> &Option<i64> {
        &self.current_index
    }
    fn current_index_mut(&mut self) -> &mut Option<i64> {
        &mut self.current_index
    }
    fn last_index(&self) -> &Option<i64> {
        &self.last_index
    }
    fn last_index_mut(&mut self) -> &mut Option<i64> {
        &mut self.last_index
    }
    fn bundle(&self) -> &Option<String> {
        &self.bundle
    }
    fn bundle_mut(&mut self) -> &mut Option<String> {
        &mut self.bundle
    }
    fn trunk_transaction(&self) -> &Option<String> {
        &self.trunk_transaction
    }
    fn trunk_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.trunk_transaction
    }
     fn branch_transaction(&self) -> &Option<String> {
        &self.branch_transaction
    }
    fn branch_transaction_mut(&mut self) -> &mut Option<String> {
        &mut self.branch_transaction
    }
     fn nonce(&self) -> &Option<String> {
        &self.nonce
    }
    fn nonce_mut(&mut self) -> &mut Option<String> {
        &mut self.nonce
    }
     fn persistence(&self) -> &Option<bool> {
        &self.persistence
    }
    fn persistence_mut(&mut self) -> &mut Option<bool> {
        &mut self.persistence
    }
    fn obsolete_tag(&self) -> &Option<String> {
        &self.obsolete_tag
    }
    fn obsolete_tag_mut(&mut self) -> &mut Option<String> {
        &mut self.obsolete_tag
    }
    fn attachment_timestamp(&self) -> &Option<i64> {
        &self.attachment_timestamp
    }
    fn attachment_timestamp_mut(&mut self) -> &mut Option<i64> {
        &mut self.attachment_timestamp
    }
    fn to_trytes(&self) -> String {
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
            res += &trits_to_string(&trits(current_index)[0..27]).unwrap();
        }
        if let Some(last_index) = self.last_index {
            res += &trits_to_string(&trits(last_index)[0..27]).unwrap();
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

#[derive(Debug, Clone)]
pub struct TransactionParseError;

impl fmt::Display for TransactionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse string into an IOTA transaction")
    }
}

impl error::Error for TransactionParseError {
    fn description(&self) -> &str {
        "Failed to parse string into an IOTA transaction"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl FromStr for Transaction<Kerl> {
        type Err = TransactionParseError;

        fn from_str(trytes: &str) -> Result<Self, Self::Err> {
            if trytes.is_empty() {
                return Err(TransactionParseError);
            }
            for i in 2279..2295 {
                if trytes.chars().nth(i).unwrap() != '9' {
                    return Err(TransactionParseError);
                }
            }
            let transaction_trits = trits_from_string(trytes);
            let mut hash = [0; 243];


            let mut transaction = Transaction::default();
            let mut curl = curl::Curl::new(curl::Mode::CURLP81);
            curl.reset();
            curl.absorb(&transaction_trits);
            curl.squeeze(&mut hash);

            *transaction.hash_mut() = Some(trits_to_string(&hash).unwrap());
            *transaction.signature_fragments_mut() = Some(trytes[0..2187].to_string());
            *transaction.address_mut() = Some(trytes[2187..2268].to_string());
            *transaction.value_mut() = Some(long_value(&transaction_trits[6804..6837]));
            *transaction.obsolete_tag_mut() = Some(trytes[2295..2322].to_string());
            *transaction.timestamp_mut() = Some(long_value(&transaction_trits[6966..6993]));
            *transaction.current_index_mut() = Some(long_value(&transaction_trits[6993..7020]));
            *transaction.last_index_mut() = Some(long_value(&transaction_trits[7020..7047]));
            *transaction.bundle_mut() = Some(trytes[2349..2430].to_string());
            *transaction.trunk_transaction_mut() = Some(trytes[2430..2511].to_string());
            *transaction.branch_transaction_mut() = Some(trytes[2511..2592].to_string());
            *transaction.tag_mut() = Some(trytes[2592..2619].to_string());
            *transaction.attachment_timestamp_mut() = Some(long_value(&transaction_trits[7857..7884]));
            *transaction.attachment_timestamp_lower_bound_mut() = Some(long_value(&transaction_trits[7884..7911]));
            *transaction.attachment_timestamp_upper_bound_mut() = Some(long_value(&transaction_trits[7911..7938]));
            *transaction.nonce_mut() = Some(trytes[2646..2673].to_string());
            Ok(transaction)
        }
}