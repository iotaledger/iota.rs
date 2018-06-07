# An unofficial implementation of the IOTA api in rust.

This is a port of the IOTA Java API into Rust. The plan is to do a straight port of the API initially, then improve it using all the greatness that Rust brings to the table.

Th0Bro's implementation of Kerl inspired my current Kerl implementation, but I plan on improving the implementation once the initial implemenation of the API is done.

Things that are done:

- [ ] PearlDiver
- [x] PoW
    - [x] Curl
    - [x] Kerl
- [ ] Model
    - [x] Bundle
    - [x] Input
    - [x] Inputs
    - [x] Neighbor
    - [x] Signature
    - [x] Transaction
    - [x] Transfer
- [ ] Utils
    - [ ] Checksum
    - [x] Constants
    - [x] Converter
    - [ ] InputValidator
    - [ ] IotaAPIUtils
    - [ ] IotaUnitConversion
    - [ ] IotaUnits
    - [ ] Multisig
    - [ ] SeedRandomGenerator
    - [ ] Signing
    - [ ] StopWatch
    - [x] TrytesConverter
- [ ] API
    - [ ] IotaAPI
    - [ ] IotaAPICommands
    - [ ] IotaAPICore
    - [ ] IotaAPIService
    - [ ] IotaLocalPoW
