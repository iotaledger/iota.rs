use failure::Error;
use iri_api;
use serde_json;
use std::time::Duration;
use utils::api_utils;
use utils::stopwatch::StopWatch;

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
}
