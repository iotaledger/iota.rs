use super::iri_api;
use super::model::*;
use super::utils;
use super::utils::constants;
use super::utils::converter;
use super::utils::input_validator;
use chrono::prelude::*;
use crate::crypto;
use failure::Error;
use std::cmp;

#[derive(Clone, Copy, Debug)]
pub struct API {
    uri: &'static str,
}

impl API {
    pub fn new(uri: &'static str) -> API {
        API { uri }
    }

    pub fn new_address(
        &self,
        seed: &str,
        index: usize,
        security: usize,
        checksum: bool,
    ) -> Result<String, Error> {
        let key = crypto::signing::key(&converter::trits_from_string(seed), index, security)?;
        let digests = crypto::signing::digests(&key)?;
        let address_trits = crypto::signing::address(&digests)?;
        let mut address = converter::trytes(&address_trits);
        if checksum {
            address = utils::add_checksum(&address)?;
        }
        Ok(address)
    }

    pub fn get_new_address(
        &self,
        seed: &str,
        index: Option<usize>,
        security: Option<usize>,
        checksum: bool,
        total: Option<usize>,
        return_all: bool,
    ) -> Result<Vec<String>, Error> {
        let mut index = index.unwrap_or_default();
        let security = security.unwrap_or(2);
        ensure!(input_validator::is_trytes(seed), "Invalid seed.");
        ensure!(security > 0 && security < 4, "Invalid security.");

        let mut all_addresses: Vec<String> = Vec::new();

        match total {
            Some(total) => {
                ensure!(total > 0, "Invalid total.");
                for _ in 0..total {
                    let address = self.new_address(seed, index, security, checksum)?;
                    all_addresses.push(address);
                }
                Ok(all_addresses)
            }
            None => loop {
                let new_address = self.new_address(seed, index, security, checksum)?;
                if return_all {
                    all_addresses.push(new_address.clone());
                }
                index += 1;
                let new_address_vec = vec![new_address];
                let were_addr_spent =
                    iri_api::were_addresses_spent_from(self.uri, &new_address_vec)?;
                if !were_addr_spent.state(0) {
                    let resp = iri_api::find_transactions(
                        self.uri,
                        None,
                        Some(&new_address_vec),
                        None,
                        None,
                    )?;
                    if resp.hashes().unwrap_or_default().is_empty() {
                        if return_all {
                            return Ok(all_addresses);
                        } else {
                            return Ok(new_address_vec);
                        }
                    }
                }
            },
        }
    }

    pub fn send_trytes(
        &self,
        trytes: &[String],
        depth: usize,
        min_weight_magnitude: usize,
        local_pow: bool,
        reference: &Option<String>,
    ) -> Result<Vec<Transaction>, Error> {
        let to_approve = iri_api::get_transactions_to_approve(self.uri, depth, &reference)?;
        if let Some(error) = to_approve.error() {
            return Err(format_err!("{}", error));
        }
        let trytes_list = if local_pow {
            let res = iri_api::attach_to_tangle(
                None,
                &to_approve.trunk_transaction().unwrap(),
                &to_approve.branch_transaction().unwrap(),
                min_weight_magnitude,
                trytes)?;
            res.trytes().unwrap()
        } else {
            let attached = iri_api::attach_to_tangle(
                Some(self.uri.to_string()),
                &to_approve.trunk_transaction().unwrap(),
                &to_approve.branch_transaction().unwrap(),
                min_weight_magnitude,
                trytes)?;
            if let Some(error) = attached.error() {
                return Err(format_err!("{}", error));
            }
            attached.trytes().unwrap()
        };
        self.store_and_broadcast(&trytes_list)?;
        Ok(trytes_list
            .iter()
            .map(|trytes| trytes.parse().unwrap())
            .collect())
    }

    pub fn store_and_broadcast(
        &self,
        trytes: &[String],
    ) -> Result<(), Error> {
        iri_api::store_transactions(self.uri, trytes)?;
        iri_api::broadcast_transactions(self.uri, trytes)?;
        Ok(())
    }

    pub fn get_inputs(
        &self,
        seed: &str,
        start: Option<usize>,
        end: Option<usize>,
        threshold: Option<i64>,
        security: Option<usize>,
    ) -> Result<Inputs, Error> {
        ensure!(input_validator::is_trytes(seed), "Invalid seed.");
        let start = start.unwrap_or(0);
        let security = security.unwrap_or(2);

        let get_balance_and_format = |addresses: &[String]| -> Result<Inputs, Error> {
            let resp = iri_api::get_balances(self.uri, addresses, 100)?;
            let mut inputs = Inputs::default();

            let mut threshold_reached = match threshold {
                Some(_) => false,
                None => true,
            };

            for (i, address) in addresses.iter().enumerate() {
                let balance: i64 = resp.balances().unwrap_or_default()[i].clone().parse()?;
                if balance > 0 {
                    let new_entry = Input::new(address.clone(), balance, start + i, security);
                    inputs.add(new_entry);
                    *inputs.total_balance_mut() += balance;
                    if let Some(threshold) = threshold {
                        if inputs.total_balance() >= threshold {
                            threshold_reached = true;
                        }
                    }
                }
            }
            if threshold_reached {
                Ok(inputs)
            } else {
                Err(format_err!("Not enough balance."))
            }
        };

        if let Some(end) = end {
            ensure!(
                start <= end && end <= start + 500,
                "Invalid inputs provided."
            );
            let mut all_addresses: Vec<String> = Vec::new();
            for i in start..end {
                all_addresses.push(self.new_address(seed, i, security, false)?);
            }
            get_balance_and_format(&all_addresses)
        } else {
            let new_address =
                self.get_new_address(seed, Some(start), Some(security), false, None, true)?;
            get_balance_and_format(&new_address)
        }
    }

    pub fn prepare_transfers(
        &self,
        seed: &str,
        transfers: &[Transfer],
        inputs: Option<Inputs>,
        remainder_address: &Option<String>,
        security: Option<usize>,
        hmac_key: Option<String>,
    ) -> Result<Vec<String>, Error> {
        let mut add_hmac = false;
        let mut added_hmac = false;
        let mut transfers = transfers.to_owned();
        ensure!(input_validator::is_trytes(seed), "Invalid seed.");
        if let Some(hmac_key) = &hmac_key {
            ensure!(input_validator::is_trytes(&hmac_key), "Invalid trytes.");
            add_hmac = true;
        }
        for transfer in &mut transfers {
            if add_hmac && *transfer.value() > 0 {
                *transfer.message_mut() = "9".repeat(243) + transfer.message();
                added_hmac = true;
            }
            if transfer.address().len() == 90 {
                ensure!(
                    utils::is_valid_checksum(transfer.address())?,
                    "Invalid address."
                );
            }
            *transfer.address_mut() = utils::remove_checksum(transfer.address());
        }
        ensure!(
            input_validator::is_transfers_collection_valid(&transfers),
            "Invalid transfers."
        );
        let security = security.unwrap_or_else(|| 2);
        let mut bundle = Bundle::default();
        let mut total_value = 0;
        let mut signature_fragments: Vec<String> = Vec::new();
        let mut tag = "".to_string();

        for transfer in transfers {
            let mut signature_message_length = 1;

            if transfer.message().len() > constants::MESSAGE_LENGTH {
                signature_message_length += (transfer.message().len() as f64
                    / constants::MESSAGE_LENGTH as f64)
                    .floor() as usize;
                let mut msg_copy = transfer.message().to_string();
                while !msg_copy.is_empty() {
                    let mut fragment = msg_copy.chars().take(constants::MESSAGE_LENGTH).collect();
                    msg_copy = msg_copy.chars().skip(constants::MESSAGE_LENGTH).collect();
                    utils::right_pad_string(&mut fragment, constants::MESSAGE_LENGTH, '9');
                    signature_fragments.push(fragment);
                }
            } else {
                let mut fragment = if !transfer.message().is_empty() { transfer.message().chars().take(2187).collect() } else { String::new() };
                utils::right_pad_string(&mut fragment, constants::MESSAGE_LENGTH, '9');
                signature_fragments.push(fragment);
            }
            tag = transfer.tag().unwrap_or_default();
            utils::right_pad_string(&mut tag, constants::TAG_LENGTH, '9');
            bundle.add_entry(
                signature_message_length,
                transfer.address(),
                *transfer.value() as i64,
                &tag,
                Utc::now().timestamp(),
            );
            total_value += *transfer.value();
        }

        if total_value > 0 {
            match inputs {
                Some(inputs) => {
                    let input_addresses: Vec<String> = inputs
                        .inputs_list()
                        .iter()
                        .map(|input| input.address().to_string())
                        .collect();
                    let resp = iri_api::get_balances(self.uri, &input_addresses, 100)?;
                    let mut confirmed_inputs = Inputs::default();
                    let balances = resp.balances().unwrap_or_default();
                    for (i, balance) in balances.iter().enumerate() {
                        let b: i64 = balance.parse()?;
                        if b > 0 {
                            *confirmed_inputs.total_balance_mut() += b;
                            let mut confirmed_input = inputs.inputs_list()[i].clone();
                            *confirmed_input.balance_mut() = b;
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
                        seed,
                        &confirmed_inputs,
                        &mut bundle,
                        &tag,
                        &remainder_address,
                        &signature_fragments,
                        added_hmac,
                        hmac_key,
                        security,
                    )
                }
                None => {
                    let inputs =
                        self.get_inputs(seed, None, None, Some(total_value), Some(security))?;
                    self.add_remainder(
                        seed,
                        &inputs,
                        &mut bundle,
                        &tag,
                        &remainder_address,
                        &signature_fragments,
                        added_hmac,
                        hmac_key,
                        security,
                    )
                }
            }
        } else {
            bundle.finalize()?;
            bundle.add_trytes(&signature_fragments);
            let mut bundle_trytes: Vec<String> = Vec::new();
            for b in bundle.bundle().iter().rev() {
                bundle_trytes.push(b.to_trytes()?);
            }
            Ok(bundle_trytes)
        }
    }

    pub fn send_transfer(
        &self,
        seed: &str,
        depth: usize,
        min_weight_magnitude: usize,
        transfers: &[Transfer],
        inputs: Option<Inputs>,
        reference: &Option<String>,
        remainder_address: &Option<String>,
        security: Option<usize>,
        hmac_key: Option<String>,
    ) -> Result<Vec<Transaction>, Error> {
        let trytes = self.prepare_transfers(
            seed,
            transfers,
            inputs,
            &remainder_address,
            security,
            hmac_key,
        )?;
        let t = self.send_trytes(&trytes, depth, min_weight_magnitude, false, &reference)?;
        Ok(t)
    }

    pub fn traverse_bundle(
        &self,
        trunk_tx: &str,
        bundle_hash: Option<String>,
        mut bundle: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, Error> {
        let tryte_list = iri_api::get_trytes(self.uri, &[trunk_tx.to_string()])?.take_trytes();
        ensure!(!tryte_list.is_empty(), "Bundle transactions not visible");
        let trytes = &tryte_list[0];
        let tx: Transaction = trytes.parse()?;
        let tx_bundle = tx.bundle().unwrap_or_default();
        ensure!(
            tx.current_index().unwrap_or_default() == 0,
            "Invalid tail transaction supplied."
        );
        let bundle_hash = bundle_hash.unwrap_or_else(|| tx_bundle.clone());
        if bundle_hash != tx_bundle {
            return Ok(bundle.to_vec());
        }

        if tx.last_index().unwrap_or_default() == 0 && tx.current_index().unwrap_or_default() == 0 {
            return Ok(vec![tx]);
        }

        let trunk_tx = tx.trunk_transaction().unwrap_or_default();
        bundle.push(tx);
        self.traverse_bundle(&trunk_tx, Some(bundle_hash), bundle)
    }

    pub fn get_bundle(&self, transaction: &str) -> Result<Vec<Transaction>, Error> {
        ensure!(
            input_validator::is_hash(transaction),
            "Invalid transaction."
        );
        let bundle = self.traverse_bundle(transaction, None, Vec::new())?;
        ensure!(utils::is_bundle(&bundle)?, "Invalid bundle provided.");
        Ok(bundle)
    }

    fn add_remainder(
        &self,
        seed: &str,
        inputs: &Inputs,
        bundle: &mut Bundle,
        tag: &str,
        remainder_address: &Option<String>,
        signature_fragments: &[String],
        added_hmac: bool,
        hmac_key: Option<String>,
        security: usize,
    ) -> Result<Vec<String>, Error> {
        let mut total_transfer_value = inputs.total_balance();
        for input in inputs.inputs_list() {
            let this_balance = input.balance();
            let to_subtract = 0 - this_balance;
            let timestamp = Utc::now().timestamp();
            let address = utils::remove_checksum(input.address());

            bundle.add_entry(input.security(), &address, to_subtract, tag, timestamp);

            if this_balance >= total_transfer_value {
                let remainder = this_balance - total_transfer_value;
                if let Some(remainder_address) = &remainder_address {
                    if remainder > 0 {
                        bundle.add_entry(1, remainder_address, remainder, tag, timestamp);
                        return self.sign_inputs_and_return(
                            seed,
                            inputs,
                            bundle,
                            signature_fragments,
                            added_hmac,
                            hmac_key,
                        );
                    }
                } else if remainder > 0 {
                    let mut start_index = 0;
                    for input in inputs.inputs_list() {
                        start_index = cmp::max(input.key_index(), start_index);
                    }
                    start_index += 1;
                    let new_address = self.get_new_address(
                        seed,
                        Some(start_index),
                        Some(security),
                        false,
                        None,
                        false,
                    )?[0]
                        .clone();
                    bundle.add_entry(1, &new_address, remainder, tag, Utc::now().timestamp());
                    return self.sign_inputs_and_return(
                        seed,
                        inputs,
                        bundle,
                        signature_fragments,
                        added_hmac,
                        hmac_key,
                    );
                } else {
                    return self.sign_inputs_and_return(
                        seed,
                        inputs,
                        bundle,
                        signature_fragments,
                        added_hmac,
                        hmac_key,
                    );
                }
            } else {
                total_transfer_value -= this_balance;
            }
        }
        Err(format_err!("Something wen't wrong..."))
    }

    fn sign_inputs_and_return(
        &self,
        seed: &str,
        inputs: &Inputs,
        bundle: &mut Bundle,
        signature_fragments: &[String],
        added_hmac: bool,
        hmac_key: Option<String>,
    ) -> Result<Vec<String>, Error> {
        bundle.finalize()?;
        bundle.add_trytes(&signature_fragments);
        for i in 0..bundle.bundle().len() {
            if bundle.bundle()[i].value().unwrap_or_default() < 0 {
                let this_address = bundle.bundle()[i].address().unwrap_or_default();
                let mut key_index = 0;
                let mut key_security = 0;
                for input in inputs.inputs_list() {
                    if input.address() == this_address {
                        key_index = input.key_index();
                        key_security = input.security();
                        break;
                    }
                }
                let bundle_hash = bundle.bundle()[i].bundle().unwrap_or_default();
                let key = crypto::signing::key(
                    &converter::trits_from_string(seed),
                    key_index,
                    key_security,
                )?;
                let normalized_bundle_hash = Bundle::normalized_bundle(&bundle_hash).to_vec();
                let mut normalized_bundle_fragments = [[0; 27]; 3];
                for (j, c) in normalized_bundle_hash.chunks(27).enumerate() {
                    normalized_bundle_fragments[j].copy_from_slice(c);
                }
                let first_fragment = key[0..6561].to_vec();
                let first_bundle_fragment = normalized_bundle_fragments[0];
                let first_signed_fragment =
                    crypto::signing::signature_fragment(&first_bundle_fragment, &first_fragment)?;
                *bundle.bundle_mut()[i].signature_fragments_mut() =
                    Some(converter::trytes(&first_signed_fragment));
                for j in 1..key_security {
                    if bundle.bundle()[i + j].address().unwrap_or_default() == this_address
                        && bundle.bundle()[i + j].value().unwrap_or_default() == 0
                    {
                        let next_fragment = key[6561 * j..(j + 1) * 6561].to_vec();
                        let next_bundle_fragment = normalized_bundle_fragments[j];
                        let next_signed_fragment = crypto::signing::signature_fragment(
                            &next_bundle_fragment,
                            &next_fragment,
                        )?;
                        *bundle.bundle_mut()[i + j].signature_fragments_mut() =
                            Some(converter::trytes(&next_signed_fragment));
                    }
                }
            }
        }
        if added_hmac {
            let hmac = crypto::HMAC::new(&hmac_key.unwrap_or_default());
            hmac.add_hmac(bundle)?;
        }
        let mut bundle_trytes: Vec<String> = Vec::new();
        for tx in bundle.bundle().iter().rev() {
            bundle_trytes.push(tx.to_trytes()?);
        }
        Ok(bundle_trytes)
    }
}
