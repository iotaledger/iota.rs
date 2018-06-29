use super::constants;
use super::converter;
use super::input_validator;
use super::signing;
use crate::model::bundle::{self, Bundle};
use crate::pow::curl::{Curl, STATE_LENGTH};
use crate::pow::sponge::{Sponge, HASH_LENGTH};

pub fn get_digest(seed: &str, security: usize, index: usize) -> String {
    let key = signing::key(&converter::trits_from_string_with_length(&seed, 243), index, security);
    converter::trytes(&key)
}

pub fn add_address_digest(digest_trytes: &str, curl_state_trytes: &str) -> String {
    let digest = converter::trits_from_string(digest_trytes);
    let offset = digest_trytes.len() * 3;
    let mut curl_state = vec![0; offset];
    if !curl_state_trytes.is_empty() {
        curl_state.copy_from_slice(&converter::trits_from_string_with_length(curl_state_trytes, offset));
    }
    let mut curl = Curl::default();
    curl.state_mut().copy_from_slice(&curl_state[0..STATE_LENGTH]);
    curl.absorb(&digest);
    converter::trytes(curl.state())
}

pub fn get_key(seed: &str, index: usize, security: usize) -> String {
    converter::trytes(&signing::key(
        &converter::trits_from_string(seed)[0..81 * security],
        index,
        security,
    ))
}

pub fn finalize_address(curl_state_trytes: &str) -> String {
    let curl_state = converter::trits_from_string(curl_state_trytes);
    let mut curl = Curl::default();
    curl.state_mut().copy_from_slice(&curl_state);
    let mut address_trits = [0; HASH_LENGTH];
    curl.squeeze(&mut address_trits);
    converter::trytes(&address_trits)
}

pub fn validate_address(multisig_address: &str, digests: &[Vec<i8>]) -> bool {
    let mut curl = Curl::default();
    for key_digest in digests {
        curl.absorb(key_digest);
    }
    let mut address_trits = [0; HASH_LENGTH];
    curl.squeeze(&mut address_trits);
    converter::trytes(&address_trits) == multisig_address
}

pub fn add_signature(bundle_to_sign: &mut Bundle, input_address: &str, key_trytes: &str) {
    let security = key_trytes.len() / constants::MESSAGE_LENGTH;

    let key = converter::trits_from_string(key_trytes);
    let mut num_signed_transactions = 0;
    for i in 0..bundle_to_sign.transactions().len() {
        if let Some(address) = bundle_to_sign.transactions()[i].address() {
            if address == input_address {
                if input_validator::is_nine_trytes(
                    &bundle_to_sign.transactions()[i]
                        .signature_fragments()
                        .unwrap(),
                ) {
                    num_signed_transactions += 1;
                } else {
                    let bundle_hash = bundle_to_sign.transactions()[i].bundle().unwrap();
                    let first_fragment = key[0..6561].to_vec();
                    let mut normalized_bundle_fragments = [[0; 27]; 3];
                    let normalized_bundle_hash = bundle::normalized_bundle(&bundle_hash);

                    for k in 0..3 {
                        let offset = k * 27;
                        normalized_bundle_fragments[k]
                            .copy_from_slice(&normalized_bundle_hash[offset..offset + 27]);
                    }

                    let first_bundle_fragment =
                        normalized_bundle_fragments[num_signed_transactions % 3];
                    let first_signed_fragment =
                        signing::signature_fragment(&first_bundle_fragment, &first_fragment);

                    *bundle_to_sign.transactions_mut()[i].signature_fragments_mut() =
                        Some(converter::trytes(&first_signed_fragment));

                    for j in 1..security {
                        let offset = j * 6561;
                        let next_fragment = key[offset..offset + 6561].to_vec();
                        let next_bundle_fragment =
                            normalized_bundle_fragments[(num_signed_transactions + j) % 3];
                        let next_signed_fragment =
                            signing::signature_fragment(&next_bundle_fragment, &next_fragment);
                        *bundle_to_sign.transactions_mut()[i + j].signature_fragments_mut() =
                            Some(converter::trytes(&next_signed_fragment));
                    }
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::bundle::Bundle;
    use crate::model::transfer::Transfer;
    use crate::utils::converter::*;
    use crate::iota_api;

    const TEST_SEED1: &str = "ABCDFG";
    const TEST_SEED2: &str = "FDSAG";
    const REMAINDER_ADDRESS: &str = "NZRALDYNVGJWUVLKDWFKJVNYLWQGCWYCURJIIZRLJIKSAIVZSGEYKTZRDBGJLOA9AWYJQB9IPWRAKUC9FBDRZJZXZG";
    const RECEIVE_ADDRESS: &str =
        "ZGHXPZYDKXPEOSQTAQOIXEEI9K9YKFKCWKYYTYAUWXK9QZAVMJXWAIZABOXHHNNBJIEBEUQRTBWGLYMTX";
    const TEST_TAG: &str = "JOTASPAM9999999999999999999";

    #[test]
    fn test_basic_multi_sig() {
        let digest_one = get_digest(TEST_SEED1, 3, 0);
        let initiated_multisig_digests = add_address_digest(&digest_one, "");
        let digest_two = get_digest(TEST_SEED2, 3, 0);
        let final_multisig_digests = add_address_digest(&digest_two, &initiated_multisig_digests);

        let multi_sig_address = finalize_address(&final_multisig_digests);

        let is_valid_multisig_address = validate_address(
            &multi_sig_address,
            &vec![
                converter::trits_from_string(&digest_one),
                converter::trits_from_string(&digest_two),
            ],
        );

        assert!(is_valid_multisig_address, "Not a valid address");

        let mut tmp_transfer = Transfer::default();
        *tmp_transfer.address_mut() = RECEIVE_ADDRESS.to_string();
        *tmp_transfer.value_mut() = 999;
        *tmp_transfer.message_mut() = "".to_string();
        *tmp_transfer.tag_mut() = TEST_TAG.to_string();
        let mut transfers = vec![tmp_transfer];

        let transactions = iota_api::initiate_transfer(6, &multi_sig_address, REMAINDER_ADDRESS, &mut transfers, true).unwrap();
        let mut bundle = Bundle::new(&transactions, transactions.len());
        add_signature(&mut bundle, &multi_sig_address, &get_key(TEST_SEED1, 0, 3));
        add_signature(&mut bundle, &multi_sig_address, &get_key(TEST_SEED2, 0, 3));
        let is_valid_sig = signing::validate_bundle_signatures(&bundle, &multi_sig_address);
        assert!(is_valid_sig, "not valid");
    }
}
