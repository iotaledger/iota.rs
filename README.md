# An unofficial implementation of the IOTA api in rust.

This is a port of the IOTA Java API into Rust. The plan is to do a straight port of the API initially, then improve it using all the greatness that Rust brings to the table.

Things that are done:

- [x] PoW
    - [x] Curl
    - [x] Kerl
    - [x] PearlDiver
    - [x] ISS
    - [x] HMAC
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
    - [x] Signing
    - [x] StopWatch
    - [x] TrytesConverter
- [ ] API
    - [ ] IotaAPI
    - [x] IotaAPICommands
    - [x] IotaAPICore
    - [x] IotaAPIService
    - [ ] IotaLocalPoW
