# An unofficial implementation of the IOTA api in rust.

This is a port of the IOTA Java/JS API into Rust. The plan is to do a straight port of the API initially, then improve it using all the greatness that Rust brings to the table.

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
        
