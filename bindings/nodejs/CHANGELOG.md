# Changelog

## \[0.3.2]

- Fix hexToBech32 when bech32\_hrp is optional.
  - [f67b445d](https://github.com/iotaledger/iota.rs/commit/f67b445d848b4cc4120a68600cd84ef0bb84de45) nodejs/fix hexToBech32 and update versions ([#533](https://github.com/iotaledger/iota.rs/pull/533)) on 2021-05-03
- Update types and make account_index for GetUnspentAddressBuilder optional.
  - [30585801](https://github.com/iotaledger/iota.rs/commit/305858017cabff456619f5ef0034dfa1973c5117) update nodejs types, optional account_index ([#518](https://github.com/iotaledger/iota.rs/pull/518)) on 2021-04-27

## \[0.3.1]

- Update minPoWScore in nodeinfo.
  - [b578d23a](https://github.com/iotaledger/iota.rs/commit/b578d23a9c212bc3851d4c3c4a8292af1fbd34de) update nodejs docs on 2021-04-11

## \[0.3.0]

- Added functions to convert addresses from bech32 to hex and vice versa.
  - [115184a8](https://github.com/iotaledger/iota.rs/commit/115184a8c712e3432cc960273278780ddc1b768a) Added hex_to_bech32 and bech32\_to_hex methods ([#471](https://github.com/iotaledger/iota.rs/pull/471)) on 2021-04-07
- Add optional quorum, primaryNode, primaryPowNode and return url together with the nodeinfo when calling getInfo().
  - [7a9ef60f](https://github.com/iotaledger/iota.rs/commit/7a9ef60fea1c865d59c744e0f6cc54371a4cebda) Node manager ([#457](https://github.com/iotaledger/iota.rs/pull/457)) on 2021-04-08
- Return addresses bech32 encoded also for balance and output endpoints.
  - [a2e09d1a](https://github.com/iotaledger/iota.rs/commit/a2e09d1a329404cdaf74890eae562fe992483b10) return address bech32 encoded for balance and outputs on 2021-04-07

## \[0.2.0]

- Fix import and installation + example in readme.
  - [5c96ab33](https://github.com/iotaledger/iota.rs/commit/5c96ab3379d992343c11ba3d7ae35ecaa22b4a0a) Fix import and installation + example in readme. ([#431](https://github.com/iotaledger/iota.rs/pull/431)) on 2021-03-18

## \[0.1.0]

- Added optional gapLimit to getBalance.
  - [cf1405e5](https://github.com/iotaledger/iota.rs/commit/cf1405e54383d71fac84c421b1b945cbe4959259) add gap_limit to GetBalanceBuilder ([#427](https://github.com/iotaledger/iota.rs/pull/427)) on 2021-03-18
