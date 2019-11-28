use chrono::prelude::*;

use iota_conversion::Trinary;
use iota_model::{Bundle, BundleEntry, Inputs, Transfer};

use std::cmp;
use std::convert::TryInto;

use crate::client::Client;
use crate::options::{GetBalancesOptions, GetInputsOptions, GetNewAddressOptions};
use crate::Result;

/// PrepareTransfersOptions
#[derive(Clone, Debug, PartialEq)]
pub struct PrepareTransfersOptions<'a, 'b> {
    /// Optional inputs to use if you're sending iota
    pub inputs: Option<Inputs>,
    /// Optional remainder address to use, if not provided, one will be generated
    pub remainder_address: Option<&'a str>,
    /// Security to use when generating addresses (1-3)
    pub security: usize,
    /// Optional key to use if you want to hmac the transfers
    pub hmac_key: Option<&'b str>,
}

impl<'a, 'b> Default for PrepareTransfersOptions<'a, 'b> {
    fn default() -> Self {
        PrepareTransfersOptions {
            inputs: None,
            remainder_address: None,
            security: 3,
            hmac_key: None,
        }
    }
}

/// AddRemainderOptions
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AddRemainderOptions<'a, 'b, 'c, 'd> {
    /// The tryte-encoded seed. It should be noted that this seed is not transferred.
    pub(crate) seed: &'a str,
    /// The tag to add to each bundle entry
    pub(crate) tag: &'b str,
    /// The address used for sending the remainder value (of the last input)
    pub(crate) remainder_address: Option<&'c str>,
    /// The signature fragments (message), used for signing. Should be 2187 characters long, can be padded with 9s.
    pub(crate) signature_fragments: Vec<String>,
    /// Check if hmac is added
    pub(crate) added_hmac: bool,
    /// Optional key to use if you want to hmac the transfers
    pub(crate) hmac_key: Option<&'d str>,
    /// Security to use when generating addresses (1-3)
    pub(crate) security: usize,
}

impl<'a> Client<'a> {
    /// Prepares a slice of transfers and converts them into a
    /// slice of tryte-encoded strings
    ///
    /// * `seed` - The wallet seed to use
    /// * `transfers` - A slice of transfers to prepare
    /// * `options` - See `PrepareTransfersOptions`
    pub fn prepare_transfers(
        &mut self,
        seed: &str,
        transfers: impl Into<Vec<Transfer>>,
        options: PrepareTransfersOptions<'_, '_>,
    ) -> Result<Vec<String>> {
        let mut transfers = transfers.into();
        let mut add_hmac = false;
        let mut added_hmac = false;

        ensure!(iota_validation::is_trytes(&seed), "Invalid seed.");
        if let Some(hmac_key) = &options.hmac_key {
            ensure!(iota_validation::is_trytes(&hmac_key), "Invalid trytes.");
            add_hmac = true;
        }
        for transfer in &mut transfers {
            if add_hmac && transfer.value > 0 {
                transfer.message = "9".repeat(243) + &transfer.message;
                added_hmac = true;
            }
            if transfer.address.len() == 90 {
                ensure!(
                    iota_signing::checksum::is_valid_checksum(&transfer.address)?,
                    "Invalid address."
                );
            }
            transfer.address = iota_signing::checksum::remove_checksum(&transfer.address);
            if transfer.value > 0 {
                ensure!(
                    if transfer.address.trits()[242] == 0 {
                        true
                    } else {
                        false
                    },
                    "Invalid Kerl address."
                );
            }
        }
        ensure!(
            iota_validation::is_transfers_collection_valid(&transfers),
            "Invalid transfers."
        );
        let security = options.security;
        let mut bundle = Bundle::default();
        let mut total_value = 0;
        let mut signature_fragments: Vec<String> = Vec::new();
        let mut tag = String::new();

        for transfer in transfers {
            let mut signature_message_length = 1;
            if transfer.message.len() > iota_constants::MESSAGE_LENGTH {
                signature_message_length += (transfer.message.len() as f64
                    / iota_constants::MESSAGE_LENGTH as f64)
                    .floor() as usize;
                let mut msg_copy = transfer.message.to_string();
                while !msg_copy.is_empty() {
                    let mut fragment = msg_copy
                        .chars()
                        .take(iota_constants::MESSAGE_LENGTH)
                        .collect();
                    msg_copy = msg_copy
                        .chars()
                        .skip(iota_constants::MESSAGE_LENGTH)
                        .collect();
                    iota_model::right_pad_string(
                        &mut fragment,
                        iota_constants::MESSAGE_LENGTH,
                        '9',
                    );
                    signature_fragments.push(fragment);
                }
            } else {
                let mut fragment = if !transfer.message.is_empty() {
                    transfer.message.chars().take(2187).collect()
                } else {
                    String::new()
                };
                iota_model::right_pad_string(&mut fragment, iota_constants::MESSAGE_LENGTH, '9');
                signature_fragments.push(fragment);
            }
            tag = transfer.tag;
            iota_model::right_pad_string(&mut tag, iota_constants::TAG_LENGTH, '9');
            bundle.add_entry(BundleEntry {
                signature_message_length,
                address: &transfer.address,
                value: transfer.value,
                tag: &tag,
                timestamp: Utc::now().timestamp(),
            });
            total_value += transfer.value;
        }

        if total_value > 0 {
            match options.inputs {
                Some(inputs) => {
                    let input_addresses: Vec<String> = inputs
                        .inputs_list()
                        .iter()
                        .map(|input| input.address.to_string())
                        .collect();
                    let resp = self.get_balances(GetBalancesOptions {
                        addresses: input_addresses,
                        ..GetBalancesOptions::default()
                    })?;
                    let mut confirmed_inputs = Inputs::default();
                    let balances = resp.take_balances().unwrap_or_default();
                    for (i, balance) in balances.iter().enumerate() {
                        let b: i64 = balance.parse()?;
                        if b > 0 {
                            *confirmed_inputs.total_balance_mut() += b;
                            let mut confirmed_input = inputs.inputs_list()[i].clone();
                            confirmed_input.balance = b;
                            confirmed_inputs.add(confirmed_input);
                            if confirmed_inputs.total_balance() >= total_value {
                                break;
                            }
                        }
                    }
                    ensure!(
                        total_value <= confirmed_inputs.total_balance(),
                        "Not enough balance."
                    );
                    self.add_remainder(
                        &confirmed_inputs,
                        &mut bundle,
                        AddRemainderOptions {
                            seed,
                            tag: &tag,
                            remainder_address: options.remainder_address,
                            signature_fragments,
                            added_hmac,
                            hmac_key: options.hmac_key,
                            security,
                        },
                    )
                }
                None => {
                    let inputs = self.get_inputs(
                        &seed,
                        GetInputsOptions {
                            start: None,
                            end: None,
                            threshold: Some(total_value),
                            security: Some(security),
                        },
                    )?;
                    self.add_remainder(
                        &inputs,
                        &mut bundle,
                        AddRemainderOptions {
                            seed,
                            tag: &tag,
                            remainder_address: options.remainder_address,
                            signature_fragments,
                            added_hmac,
                            hmac_key: options.hmac_key,
                            security,
                        },
                    )
                }
            }
        } else {
            bundle.reset_indexes();
            bundle.finalize()?;
            bundle.add_trytes(&signature_fragments);
            let mut bundle_trytes: Vec<String> = Vec::new();
            for b in bundle.iter().rev() {
                bundle_trytes.push(b.try_into()?);
            }
            Ok(bundle_trytes)
        }
    }

    fn add_remainder(
        &mut self,
        inputs: &Inputs,
        bundle: &mut Bundle,
        options: AddRemainderOptions<'_, '_, '_, '_>,
    ) -> Result<Vec<String>> {
        let mut total_transfer_value = inputs.total_balance();
        for input in inputs.inputs_list() {
            let this_balance = input.balance;
            let to_subtract = 0 - this_balance;
            let timestamp = Utc::now().timestamp();
            let address = iota_signing::checksum::remove_checksum(&input.address);

            bundle.add_entry(BundleEntry {
                signature_message_length: input.security,
                address: &address,
                value: to_subtract,
                tag: &options.tag,
                timestamp,
            });

            if this_balance >= total_transfer_value {
                let remainder = this_balance - total_transfer_value;
                if let Some(remainder_address) = &options.remainder_address {
                    if remainder > 0 {
                        bundle.add_entry(BundleEntry {
                            signature_message_length: 1,
                            address: &remainder_address,
                            value: remainder,
                            tag: &options.tag,
                            timestamp,
                        });
                        return self.sign_inputs_and_return(
                            &options.seed,
                            inputs,
                            bundle,
                            &options.signature_fragments,
                            options.added_hmac,
                            options.hmac_key,
                        );
                    }
                } else if remainder > 0 {
                    let mut start_index = 0;
                    for input in inputs.inputs_list() {
                        start_index = cmp::max(input.key_index, start_index);
                    }
                    start_index += 1;
                    let new_address = &self.get_new_address(
                        &options.seed,
                        false,
                        false,
                        GetNewAddressOptions {
                            security: Some(options.security),
                            index: Some(start_index),
                            total: None,
                        },
                    )?[0];
                    bundle.add_entry(BundleEntry {
                        signature_message_length: 1,
                        address: &new_address,
                        value: remainder,
                        tag: &options.tag,
                        timestamp: Utc::now().timestamp(),
                    });
                    return self.sign_inputs_and_return(
                        &options.seed,
                        inputs,
                        bundle,
                        &options.signature_fragments,
                        options.added_hmac,
                        options.hmac_key,
                    );
                } else {
                    return self.sign_inputs_and_return(
                        &options.seed,
                        inputs,
                        bundle,
                        &options.signature_fragments,
                        options.added_hmac,
                        options.hmac_key,
                    );
                }
            } else {
                total_transfer_value -= this_balance;
            }
        }
        Err(format_err!("Something wen't wrong..."))
    }

    fn sign_inputs_and_return<'b>(
        &mut self,
        seed: &str,
        inputs: &Inputs,
        bundle: &mut Bundle,
        signature_fragments: &[String],
        added_hmac: bool,
        hmac_key: Option<&'b str>,
    ) -> Result<Vec<String>> {
        bundle.reset_indexes();
        bundle.finalize()?;
        bundle.add_trytes(&signature_fragments);
        for i in 0..bundle.len() {
            if bundle[i].value < 0 {
                let this_address = bundle[i].address.clone();
                let mut key_index = 0;
                let mut key_security = 0;
                for input in inputs.inputs_list() {
                    if input.address == *this_address {
                        key_index = input.key_index;
                        key_security = input.security;
                        break;
                    }
                }
                let bundle_hash = &bundle[i].bundle;
                let key = iota_signing::key(&seed.trits(), key_index, key_security)?;
                let normalized_bundle_hash = Bundle::normalized_bundle(&bundle_hash).to_vec();
                let mut normalized_bundle_fragments = [[0; 27]; 3];
                for (j, c) in normalized_bundle_hash.chunks(27).enumerate() {
                    normalized_bundle_fragments[j].copy_from_slice(c);
                }
                let first_fragment = key[0..6561].to_vec();
                let first_bundle_fragment = normalized_bundle_fragments[0];
                let first_signed_fragment =
                    iota_signing::signature_fragment(&first_bundle_fragment, &first_fragment)?;
                bundle[i].signature_fragments = first_signed_fragment.trytes()?;
                for j in 1..key_security {
                    if bundle[i + j].address == *this_address && bundle[i + j].value == 0 {
                        let next_fragment = key[6561 * j..(j + 1) * 6561].to_vec();
                        let next_bundle_fragment = normalized_bundle_fragments[j];
                        let next_signed_fragment = iota_signing::signature_fragment(
                            &next_bundle_fragment,
                            &next_fragment,
                        )?;
                        bundle[i + j].signature_fragments = next_signed_fragment.trytes()?;
                    }
                }
            }
        }
        if added_hmac {
            let hmac = iota_signing::HMAC::new(&hmac_key.unwrap_or_default());
            hmac.add_hmac(bundle)?;
        }
        let mut bundle_trytes: Vec<String> = Vec::new();
        for tx in bundle.iter().rev() {
            let tx_trytes: String = tx.try_into()?;
            bundle_trytes.push(tx_trytes);
        }
        Ok(bundle_trytes)
    }
}
