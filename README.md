# An unofficial implementation of the IOTA api in rust.

This is a port of the IOTA Java/JS API into Rust. It works, but I wouldn't trust it with real money yet. Having said that, please let me know if you have any suggestions or run into any issues.

Here are some reasons you might want to use this library:
1. It has a very fast implementation of local PoW (1-2s with MwM = 14, 4-6ms with MwM = 9 on my laptop)
2. You'll benefit from Rust's very nice type system
3. This library is more actively maintained than Jota
4. Now that the library is working, I'm going to be obsessively going over it to improve safety, performance, and usability
5. It would make me personally happy :)

(Documentation coming soon...)

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
```
extern crate iota_lib_rs;

use iota_lib_rs::iota_api;
use iota_lib_rs::utils::trytes_converter;
use iota_lib_rs::model::*;

fn main() {
    let trytes = "HELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDD";
    let message = trytes_converter::to_trytes("Hello World").unwrap();
    let mut transfer = Transfer::default();
    *transfer.value_mut() = 0;
    *transfer.address_mut() = trytes.to_string();
    *transfer.message_mut() = message;
    let transfers = [transfer];
    let api = iota_api::API::new("https://trinity.iota.fm");
    let tx = api.send_transfer(trytes, 3, 14, &transfers, true, None, &None, &None, None, None).unwrap();
    println!("{:?}", tx);
}
```

# Donations:
If you feel so inclined, you can find my address for donations at:

https://ecosystem.iota.org/projects/iota-lib-rs
