use std::fmt;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::Result;
use iota_constants::HASH_TRINARY_SIZE as HASH_LENGTH;
use iota_conversion::Trinary;
use iota_crypto::{Kerl, Sponge};

use super::transaction::Transaction;

const EMPTY_HASH: &str =
    "999999999999999999999999999999999999999999999999999999999999999999999999999999999";

/// Represents a bundle of transactions
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Bundle(Vec<Transaction>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BundleEntry<'a, 'b> {
    pub signature_message_length: usize,
    pub address: &'a str,
    pub value: i64,
    pub tag: &'b str,
    pub timestamp: i64,
}

impl Deref for Bundle {
    type Target = Vec<Transaction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bundle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Bundle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

impl Bundle {
    /// Greates a new bundle using the provided transactions
    pub fn new(transactions: impl Into<Vec<Transaction>>) -> Bundle {
        Bundle(transactions.into())
    }

    /// Adds an entry into the bundle
    pub fn add_entry(&mut self, entry: BundleEntry) {
        for i in 0..entry.signature_message_length {
            let mut trx = Transaction::default();
            trx.address = entry.address.into();
            trx.tag = entry.tag.into();
            trx.obsolete_tag = entry.tag.into();
            trx.timestamp = entry.timestamp;
            match i {
                0 => trx.value = entry.value,
                _ => trx.value = 0,
            }
            self.0.push(trx);
        }
    }

    /// Adds trytes into the bundle
    pub fn add_trytes(&mut self, signature_fragments: &[String]) {
        let empty_signature_fragment = "9".repeat(2187);
        let empty_hash = EMPTY_HASH;
        let empty_timestamp = 999_999_999;

        for (i, bundle) in self.0.iter_mut().enumerate() {
            let new_sig = if signature_fragments.is_empty() || signature_fragments[i].is_empty() {
                &empty_signature_fragment
            } else {
                &signature_fragments[i]
            };
            bundle.signature_fragments = new_sig.clone();
            bundle.trunk_transaction = empty_hash.into();
            bundle.branch_transaction = empty_hash.into();
            bundle.attachment_timestamp = empty_timestamp;
            bundle.attachment_timestamp_lower_bound = empty_timestamp;
            bundle.attachment_timestamp_upper_bound = empty_timestamp;
            bundle.nonce = "9".repeat(27);
        }
    }

    /// Finalizes the bundle
    pub fn finalize(&mut self) -> Result<()> {
        let mut valid_bundle = false;
        let mut kerl = Kerl::default();
        while !valid_bundle {
            kerl.reset();
            for bundle in &mut self.0 {
                let value_trits = bundle.value.trits_with_length(81);
                let timestamp_trits = bundle.timestamp.trits_with_length(27);
                let current_index_trits = (bundle.current_index as i64).trits_with_length(27);
                let last_index_trits = (bundle.last_index as i64).trits_with_length(27);
                let bundle_essence = bundle.address.clone()
                    + &value_trits.trytes()?
                    + &bundle.obsolete_tag
                    + &timestamp_trits.trytes()?
                    + &current_index_trits.trytes()?
                    + &last_index_trits.trytes()?;
                kerl.absorb(&bundle_essence.trits())?;
            }
            let mut hash = [0; HASH_LENGTH];
            kerl.squeeze(&mut hash)?;
            let hash_trytes = hash.trytes()?;
            for bundle in &mut self.0 {
                bundle.bundle = hash_trytes.clone();
            }
            let normalized_hash = Bundle::normalized_bundle(&hash_trytes);
            if normalized_hash.contains(&13) {
                let increased_tag = crate::trit_adder::add(&self.0[0].obsolete_tag.trits(), &[1]);
                self.0[0].obsolete_tag = increased_tag.trytes()?;
            } else {
                valid_bundle = true;
            }
        }
        Ok(())
    }

    /// Normalizes a bundle hash
    pub fn normalized_bundle(bundle_hash: &str) -> [i8; 81] {
        let mut normalized_bundle = [0; 81];
        for i in 0..3 {
            let mut sum: i64 = 0;
            for j in 0..27 {
                let mut t = String::new();
                t.push(bundle_hash.chars().nth(i * 27 + j).unwrap());
                normalized_bundle[i * 27 + j] = iota_conversion::value(&t.trits());
                sum += i64::from(normalized_bundle[i * 27 + j]);
            }
            if sum >= 0 {
                while sum > 0 {
                    for j in 0..27 {
                        if normalized_bundle[i * 27 + j] > -13 {
                            normalized_bundle[i * 27 + j] -= 1;
                            break;
                        }
                    }
                    sum -= 1;
                }
            } else {
                while sum < 0 {
                    for j in 0..27 {
                        if normalized_bundle[i * 27 + j] < 13 {
                            normalized_bundle[i * 27 + j] += 1;
                            break;
                        }
                    }
                    sum += 1;
                }
            }
        }
        normalized_bundle
    }
}
