use iota_conversion::Trinary;
use iota_conversion;
use iota_crypto::{self, Kerl, Sponge};
use iota_model::{Signature, Transaction};

use crate::Result;

use super::input_validator;

/// Validates that a slice of transactions is a valid bundle
pub fn is_bundle(bundle: &[Transaction]) -> Result<bool> {
    if !input_validator::is_slice_of_transactions(bundle) {
        return Ok(false);
    }
    let mut total_sum = 0;
    let bundle_hash = bundle[0].bundle().unwrap_or_default();

    let mut hash_from_txs = [0; 243];
    let mut kerl = Kerl::default();
    let mut signatures_to_validate: Vec<Signature> = Vec::new();

    for (index, tx) in bundle.iter().enumerate() {
        let tx_value = tx.value().unwrap_or_default();
        total_sum += tx_value;
        if index != tx.current_index().unwrap_or_default() {
            return Ok(false);
        }
        let tx_trytes: String = tx.to_trytes()?;
        let tx_trits = (&tx_trytes[2187..2187 + 162]).trits();
        kerl.absorb(&tx_trits)?;
        if tx_value < 0 {
            let this_address = tx.address().unwrap_or_default();
            let mut new_signature = Signature::default();
            for i in index..bundle.len() - 1 {
                let new_tx = &bundle[i + 1];
                if new_tx.address().unwrap_or_default() == this_address
                    && new_tx.value().unwrap_or_default() == 0
                {
                    new_signature.add_fragment(new_tx.signature_fragments().unwrap_or_default());
                }
            }
            signatures_to_validate.push(new_signature);
        }
    }
    if total_sum != 0 {
        return Ok(false);
    }
    kerl.squeeze(&mut hash_from_txs)?;
    let bundle_from_txs = hash_from_txs.trytes()?;
    if bundle_from_txs != bundle_hash {
        return Ok(false);
    }
    if bundle.last().unwrap().current_index().unwrap_or_default()
        != bundle.last().unwrap().last_index().unwrap_or_default()
    {
        return Ok(false);
    }
    // TODO
    // for sig in signatures_to_validate {
    //     if !crypto::signing::validate_signatures(
    //         sig.address(),
    //         sig.signature_fragments(),
    //         &bundle_hash,
    //     )? {
    //         return Ok(false);
    //     }
    // }
    Ok(true)
}
