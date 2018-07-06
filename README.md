# An unofficial implementation of the IOTA api in rust.

This is a port of the IOTA Java/JS API into Rust. It works, but I wouldn't trust it with real money yet. Having said that, please let me know if you have any suggestions or run into any issues.

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
```