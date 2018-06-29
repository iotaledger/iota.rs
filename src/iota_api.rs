use super::iri_api;
use super::model::bundle::Bundle;
use super::model::transfer::Transfer;
use super::model::transaction::Transaction;
use super::utils::api_utils;
use super::utils::input_validator;
use super::utils::checksum;
use super::utils::stopwatch::StopWatch;
use super::utils::constants;
use failure::Error;
use serde_json;
use std::time::Duration;
use chrono::prelude::*;
use chrono::DateTime;

#[derive(Clone, Copy, Debug)]
pub struct API {
    uri: &'static str,
}

impl API {
    pub fn get_new_address(
        &self,
        seed: &str,
        security: usize,
        index: usize,
        checksum: bool,
        total: usize,
        return_all: bool,
    ) -> Result<(Vec<String>, Duration), Error> {
        let stopwatch = StopWatch::default();
        let mut all_addresses: Vec<String> = Vec::new();
        if total != 0 {
            for i in 0..index + total {
                all_addresses.push(api_utils::new_address(seed, security, i, checksum));
            }
            return Ok((all_addresses, stopwatch.elapsed_time()));
        }

        let mut i = index;
        loop {
            let new_address = api_utils::new_address(seed, security, i, checksum);
            let resp = iri_api::find_transactions(
                self.uri,
                None,
                Some(&[new_address.clone()]),
                None,
                None,
            )?;

            all_addresses.push(new_address);
            let hashes: Vec<String> = serde_json::from_value(resp["hashes"].clone())?;
            if hashes.is_empty() {
                break;
            }
            i += 1;
        }

        if !return_all {
            all_addresses = all_addresses[all_addresses.len() - 1..].to_vec();
        }

        Ok((all_addresses, stopwatch.elapsed_time()))
    }

    fn get_transfers(
        &self,
        seed: &str,
        security: usize,
        start: usize,
        end: usize,
        inclusion_states: bool,
    ) -> (Vec<Bundle>, Duration) {
        let stopwatch = StopWatch::default();
        if let Ok(resp) = self.get_new_address(seed, security, start, false, end, true) {
            return (
                bundles_from_addresses(&resp.0, inclusion_states),
                stopwatch.elapsed_time(),
            );
        }
        (vec![Bundle::default()], stopwatch.elapsed_time())
    }
}

pub fn bundles_from_addresses(_addresses: &[String], _inclusion_states: bool) -> Vec<Bundle> {
    vec![Bundle::default()]
}

pub fn initiate_transfer(security_sum: usize, input_address: &str, remainder_address: &str, transfers: &mut [Transfer], test_mode: bool) -> Result<Vec<Transaction>, Error> {
    ensure!(input_validator::is_address(input_address), "Invalid address [{}]", input_address);
    ensure!(input_validator::is_address(remainder_address), "Invalid address [{}]", remainder_address);
    ensure!(input_validator::is_transfers_collection_valid(transfers), "Invalid transfers [{:?}]", transfers);

    let mut bundle = Bundle::default();
    let mut total_value: i64 = 0;
    let mut signature_fragments: Vec<String> = Vec::new();
    let mut tag = "".to_string();
    for transfer in transfers.iter_mut() {
        if checksum::is_valid_checksum(transfer.address()) {
            *transfer.address_mut() = checksum::remove_checksum(transfer.address());
        }

        let mut signature_message_length = 1;

        if transfer.message().len() > constants::MESSAGE_LENGTH {
            signature_message_length += (transfer.message().len() as f64 / constants::MESSAGE_LENGTH as f64).floor() as usize;
            let mut msg_copy = transfer.message().to_string();
            while !msg_copy.is_empty() {
                let mut fragment: String = msg_copy.chars().take(constants::MESSAGE_LENGTH).collect();
                msg_copy = msg_copy.chars().skip(constants::MESSAGE_LENGTH).take(msg_copy.len()).collect();
                right_pad(&mut fragment, constants::MESSAGE_LENGTH, '9');
                signature_fragments.push(fragment);
            }
        } else {
            let mut fragment = transfer.message().to_string();
            right_pad(&mut fragment, constants::MESSAGE_LENGTH, '9');
            signature_fragments.push(fragment);
        }
        tag = transfer.tag().to_string();
        right_pad(&mut tag, constants::TAG_LENGTH, '9');
        let utc: DateTime<Utc> = Utc::now();
        let timestamp = utc.timestamp();
        bundle.add_entry(signature_message_length, transfer.address(), *transfer.value() as i64, &tag, timestamp);
        total_value += *transfer.value() as i64;
    }
    if total_value != 0 {
        let mut total_balance = 0;
        let utc: DateTime<Utc> = Utc::now();
        let timestamp = utc.timestamp();

        if test_mode {
            total_balance += 1000;
        }

        if total_balance > 0 {
        let to_subtract = 0 - total_balance;
        bundle.add_entry(security_sum, input_address, to_subtract, &tag, timestamp);
        }

        ensure!(total_balance >= total_value, "Not enough balance");

        if total_balance > total_value {
            let remainder = total_balance - total_value;
            bundle.add_entry(1, remainder_address, remainder, &tag, timestamp);
        }

        bundle.finalize();
        bundle.add_trytes(&signature_fragments);
        return Ok(bundle.transactions().to_vec());

    }

    Err(format_err!("Invalid value transfer"))
}

fn right_pad(x: &mut String, len: usize, pad: char) {
    if x.len() < len {
        for _ in x.len()..len {
            x.push(pad);
        }
    }
}