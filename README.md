# An unofficial implementation of the IOTA api in rust.
[![Build Status](https://travis-ci.org/njaremko/iota-lib-rs.svg?branch=async)](https://travis-ci.org/njaremko/iota-lib-rs) 
[![Windows Build status](https://ci.appveyor.com/api/projects/status/m1g0ddlgxk8wq9es?svg=true)](https://ci.appveyor.com/project/njaremko/iota-lib-rs)
[![Version](https://img.shields.io/crates/v/iota-lib-rs-preview.svg)](https://crates.io/crates/iota-lib-rs-preview)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/njaremko/iota-lib-rs/master/LICENSE)


This is a port of the IOTA Java/JS API into Rust. It works, but I wouldn't trust it with real money yet. Having said that, please let me know if you have any suggestions or run into any issues.

Here are some reasons you might want to use this library:
1. It has a very fast implementation of local PoW (1-2s with MwM = 14, 4-6ms with MwM = 9 on my laptop)
2. You'll benefit from Rust's very nice type system
3. This library is more actively maintained than Jota
4. Now that the library is working, I'm going to be obsessively going over it to improve safety, performance, and usability
5. It would make me personally happy :)

# Documentation

https://docs.rs/iota-lib-rs-preview

This library currently requires nightly rust to build.

Things that are done:

- [x] Crypto
    - [x] Curl
    - [x] Kerl
    - [x] PearlDiver
    - [x] ISS
    - [x] HMAC
    - [x] Signing
- [x] Model
    - [x] Bundle
    - [x] Input
    - [x] Inputs
    - [x] Neighbor
    - [x] Signature
    - [x] Transaction
    - [x] Transfer
- [x] Utils
    - [x] Checksum
    - [x] Constants
    - [x] Converter
    - [x] InputValidator
    - [x] IotaAPIUtils
    - [x] IotaUnitConversion
    - [x] IotaUnits
    - [x] Multisig
    - [x] SeedRandomGenerator
    - [x] StopWatch
    - [x] TrytesConverter
- [ ] API
    - [x] IRI API calls and responses
        - [x] add neighbors
        - [x] attach_to_tangle
        - [x] find_transactions
        - [x] get_balances
        - [x] broadcastTransactions
        - [x] storeTransactions
        - [x] get_inclusion_states
        - [x] get_neighbors
        - [x] get_node_info
        - [x] get_tips
        - [x] get_transactions_to_approve
        - [x] get_trytes
        - [x] remove_neighbor
        - [x] were_addresses_spent_from
        - [x] check_consistency
    - [ ] Ease of use wrappers/helpers
        - [x] new_address
        - [x] get_new_address
        - [x] send_trytes
        - [x] store_and_broadcast
        - [x] get_inputs
        - [x] prepare_transfers
        - [x] traverse_bundle
        - [x] send_transfer
        - [x] get_bundle

Here's an example of how to send a transaction: (Note that we're using the address as the seed in `send_transfer()`...don't do this)
```rust
extern crate iota_lib_rs;
extern crate futures;

use iota_lib_rs::iota_api;
use iota_lib_rs::utils::trytes_converter;
use iota_lib_rs::model::*;
use futures::executor::block_on;

fn main() {
    let trytes = "HELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDD";
    let message = trytes_converter::to_trytes("Hello World").unwrap();
    let mut transfer = Transfer::default();
    *transfer.value_mut() = 0;
    *transfer.address_mut() = trytes.to_string();
    *transfer.message_mut() = message;
    let api = iota_api::API::new("https://trinity.iota.fm");
    let options = SendTransferOptions{
         seed: trytes.to_string(),
         depth: 3,
         min_weight_magnitude: 14,
         local_pow: true,
         threads: None,
         inputs: None,
         reference: None,
         remainder_address: None,
         security: None,
         hmac_key: None,
     };
    let tx = block_on(api.send_transfers(vec![transfer], options)).unwrap();
    println!("{:?}", tx);
}
```

# Donations:
If you feel so inclined, you can find my address for donations at:

https://ecosystem.iota.org/projects/iota-lib-rs
