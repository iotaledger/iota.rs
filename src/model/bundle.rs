use model::transaction::Transaction;
use pow::kerl::Kerl;
use pow::traits::ICurl;
use std::iter;
use utils::converter;
const EMPTY_HASH: &str =
    "999999999999999999999999999999999999999999999999999999999999999999999999999999999";

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Bundle {
    transactions: Vec<Transaction>,
    length: usize,
}

impl Bundle {
    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn transactions_mut(&mut self) -> &mut [Transaction] {
        &mut self.transactions
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn length_mut(&mut self) -> &mut usize {
        &mut self.length
    }

    fn add_entry(
        &mut self,
        signature_message_length: usize,
        address: &str,
        value: i64,
        tag: &str,
        timestamp: i64,
    ) {
        for i in 0..signature_message_length {
            let mut trx = Transaction::default();
            *trx.address_mut() = Some(address.to_string());
            *trx.timestamp_mut() = Some(timestamp);
            *trx.tag_mut() = Some(tag.to_string());
            match i {
                0 => *trx.value_mut() = Some(value),
                _ => *trx.value_mut() = Some(0),
            }
            self.transactions.push(trx);
        }
    }

    pub fn finalize(&mut self) {
        let mut curl = Kerl::default();
        let mut valid = true;
        let mut hash = [0; 243];
        let mut hash_in_trytes = String::new();
        let mut normalized_bundle_value: [i8; 81];
        let mut obsolete_tag_trits: Vec<i8>;
        while valid {
            curl.reset();
            for i in 0..self.transactions.len() {
                let value_trits = converter::trits(self.transactions[i].value().unwrap());
                let timestamp_trits = converter::trits(self.transactions[i].timestamp().unwrap());

                *self.transactions[i].current_index_mut() = Some(i);

                let current_index_trits =
                    converter::trits(self.transactions[i].current_index().unwrap() as i64);

                *self.transactions[i].last_index_mut() = Some(self.transactions.len() - 1);

                let last_index_trits =
                    converter::trits(self.transactions[i].last_index().unwrap() as i64);
                let address = self.transactions[i].address().clone().unwrap();
                let obsolete_tag = self.transactions[i].obsolete_tag().clone().unwrap();
                let mut t = converter::trits_from_string(&format!(
                    "{}{}{}{}{}{}",
                    address,
                    converter::trytes(&value_trits),
                    obsolete_tag,
                    converter::trytes(&timestamp_trits),
                    converter::trytes(&current_index_trits),
                    converter::trytes(&last_index_trits)
                ));
                curl.absorb(&mut t);
            }
            curl.squeeze(&mut hash);
            hash_in_trytes = converter::trytes(&hash);
            normalized_bundle_value = normalized_bundle(&hash_in_trytes);

            let mut found_value = false;
            for b in normalized_bundle_value.iter() {
                if *b == 13 {
                    found_value = true;
                    obsolete_tag_trits = converter::trits_from_string(
                        &self.transactions[0].obsolete_tag().clone().unwrap(),
                    );
                    converter::increment(&mut obsolete_tag_trits, 81);
                    *self.transactions[0].obsolete_tag_mut() =
                        Some(converter::trytes(&obsolete_tag_trits));
                }
            }
            valid = !found_value;
        }
        for i in 0..self.transactions.len() {
            *self.transactions[i].bundle_mut() = Some(hash_in_trytes.clone());
        }
    }

    pub fn add_trytes(&mut self, signature_fragments: &[String]) {
        let empty_signature_fragment = iter::repeat("9").take(2187).collect::<String>();
        let empty_hash = EMPTY_HASH;
        let empty_timestamp = 999_999_999;
        for i in 0..self.transactions.len() {
            *self.transactions[i].signature_fragments_mut() =
                if signature_fragments.len() <= 1 || signature_fragments[i].is_empty() {
                    Some(empty_signature_fragment.clone())
                } else {
                    Some(signature_fragments[i].clone())
                };
            *self.transactions[i].trunk_transaction_mut() = Some(empty_hash.to_string());
            *self.transactions[i].branch_transaction_mut() = Some(empty_hash.to_string());
            *self.transactions[i].attachment_timestamp_mut() = Some(empty_timestamp);
            *self.transactions[i].attachment_timestamp_lower_bound_mut() = Some(empty_timestamp);
            *self.transactions[i].attachment_timestamp_upper_bound_mut() = Some(empty_timestamp);
            *self.transactions[i].nonce_mut() =
                Some(iter::repeat("9").take(27).collect::<String>());
        }
    }
}

pub fn normalized_bundle(bundle_hash: &str) -> [i8; 81] {
    let mut normalized_bundle = [0; 81];
    for i in 0..3 {
        let mut sum = 0;
        for j in 0..27 {
            let mut t = String::new();
            t.push(bundle_hash.chars().nth(i * 27 + j).unwrap());
            normalized_bundle[i * 27 + j] = converter::value(&converter::trits_from_string(&t));
            sum += normalized_bundle[i * 27 + j];
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
