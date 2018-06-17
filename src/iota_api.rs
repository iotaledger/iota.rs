use failure::Error;
use iri_api;
use model::bundle::Bundle;
use serde_json;
use std::time::Duration;
use utils::api_utils;
use utils::stopwatch::StopWatch;

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

pub fn bundles_from_addresses(addresses: &[String], inclusion_states: bool) -> Vec<Bundle> {
    vec![Bundle::default()]
}
