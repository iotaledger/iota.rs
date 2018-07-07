use super::transaction::Transaction;
use crate::crypto::{Kerl, Sponge, HASH_LENGTH};
use crate::utils::{converter, trit_adder};
use serde_json;
use std::fmt;

use crate::Result;

const EMPTY_HASH: &str =
    "999999999999999999999999999999999999999999999999999999999999999999999999999999999";

/// Represents a bundle of transactions
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Bundle {
    bundle: Vec<Transaction>,
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
    pub fn new(transactions: &[Transaction]) -> Bundle {
        Bundle {
            bundle: transactions.to_vec(),
        }
    }

    /// Provides a view into the transactions inside this bundle
    pub fn bundle(&self) -> &[Transaction] {
        &self.bundle
    }

    /// Provides a mutable view into the transactions inside
    /// this bundle
    pub fn bundle_mut(&mut self) -> &mut [Transaction] {
        &mut self.bundle
    }

    /// Setter accepting anything that can be turned into the relevant type
    pub fn set_bundle<T>(&mut self, new_value: T)
    where
        T: Into<Vec<Transaction>>,
    {
        self.bundle = new_value.into();
    }

    /// Adds an entry into the bundle
    pub fn add_entry(
        &mut self,
        signature_message_length: usize,
        address: &str,
        value: i64,
        tag: &str,
        timestamp: i64,
    ) {
        for i in 0..signature_message_length {
            let mut trx = Transaction::default();
            trx.set_address(address);
            trx.set_tag(tag);
            trx.set_obsolete_tag(tag);
            trx.set_timestamp(timestamp);
            match i {
                0 => trx.set_value(value),
                _ => trx.set_value(0),
            }
            self.bundle.push(trx);
        }
    }

    /// Adds trytes into the bundle
    pub fn add_trytes(&mut self, signature_fragments: &[String]) {
        let empty_signature_fragment = "9".repeat(2187);
        let empty_hash = EMPTY_HASH;
        let empty_timestamp = 999_999_999;

        for (i, bundle) in self.bundle.iter_mut().enumerate() {
            let new_sig: String =
                if signature_fragments.is_empty() || signature_fragments[i].is_empty() {
                    empty_signature_fragment.clone()
                } else {
                    signature_fragments[i].clone()
                };
            bundle.set_signature_fragments(new_sig);
            bundle.set_trunk_transaction(empty_hash);
            bundle.set_branch_transaction(empty_hash);
            bundle.set_attachment_timestamp(empty_timestamp);
            bundle.set_attachment_timestamp_lower_bound(empty_timestamp);
            bundle.set_attachment_timestamp_upper_bound(empty_timestamp);
            bundle.set_nonce("9".repeat(27));
        }
    }

    /// Finalizes the bundle
    pub fn finalize(&mut self) -> Result<()> {
        let mut valid_bundle = false;
        let mut kerl = Kerl::default();
        while !valid_bundle {
            kerl.reset();
            for bundle in &mut self.bundle {
                let value_trits = converter::trits_with_length(bundle.value().unwrap(), 81);
                let timestamp_trits = converter::trits_with_length(bundle.timestamp().unwrap(), 27);
                let current_index_trits = converter::trits_with_length(
                    bundle.current_index().unwrap_or_default() as i64,
                    27,
                );
                let last_index_trits = converter::trits_with_length(
                    bundle.last_index().unwrap_or_default() as i64,
                    27,
                );
                let bundle_essence = converter::trits_from_string(
                    &(bundle.address().unwrap_or_default().to_string()
                        + &converter::trytes(&value_trits)
                        + &bundle.obsolete_tag().unwrap_or_default()
                        + &converter::trytes(&timestamp_trits)
                        + &converter::trytes(&current_index_trits)
                        + &converter::trytes(&last_index_trits)),
                );
                kerl.absorb(&bundle_essence)?;
            }
            let mut hash = [0; HASH_LENGTH];
            kerl.squeeze(&mut hash)?;
            let hash_trytes = converter::trytes(&hash);
            for bundle in &mut self.bundle {
                bundle.set_bundle(hash_trytes.clone());
            }
            let normalized_hash = Bundle::normalized_bundle(&hash_trytes.clone());
            if normalized_hash.contains(&13) {
                let increased_tag = trit_adder::add(
                    &converter::trits_from_string(
                        &self.bundle[0].obsolete_tag().unwrap_or_default(),
                    ),
                    &[1],
                );
                self.bundle[0].set_obsolete_tag(converter::trytes(&increased_tag));
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
                normalized_bundle[i * 27 + j] = converter::value(&converter::trits_from_string(&t));
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
