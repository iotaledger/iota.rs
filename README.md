# An unofficial implementation of the IOTA api in rust.

This is a port of the IOTA Java API into Rust. The plan is to do a straight port of the API initially, then improve it using all the greatness that Rust brings to the table.

Th0Bro's implementation of Kerl inspired my current Kerl implementation, but I plan on improving the implementation once the initial implemenation of the API is done.

Things that are done:

- [ ] PearlDiver
- [x] PoW
    - [x] Curl
    - [x] Kerl
- [x] Model
    - [x] Bundle
    - [x] Input
    - [x] Inputs
    - [x] Neighbor
    - [x] Signature
    - [x] Transaction
    - [x] Transfer
- [ ] Utils
    - [x] Checksum
    - [x] Constants
    - [x] Converter
    - [x] InputValidator
    - [ ] IotaAPIUtils
    - [x] IotaUnitConversion
    - [x] IotaUnits
    - [ ] Multisig
    - [x] SeedRandomGenerator
    - [x] Signing
    - [x] StopWatch
    - [x] TrytesConverter
- [ ] API
    - [ ] IotaAPI
    - [ ] IotaAPICommands
    - [ ] IotaAPICore
    - [ ] IotaAPIService
    - [ ] IotaLocalPoW
