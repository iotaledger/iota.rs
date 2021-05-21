# Changelog

## \[0.5.1]

- Set git repo and rev to allow JS bindings to be built from source
  - [282de0a6](https://github.com/iotaledger/iota.rs/commit/282de0a6db2e8522b040c7aee1228840a6296cf1) fix(ci,bindings/nodejs): Set rev to allow building from source ([#573](https://github.com/iotaledger/iota.rs/pull/573)) on 2021-05-20
- Build bindings on Ubuntu 18.04 to support older versions of glibc
  - [9ee430ca](https://github.com/iotaledger/iota.rs/commit/9ee430cac5b21d61676c239f91414a00831be309) fix(ci): Build Node.js bindings on Ubuntu 18.04 ([#576](https://github.com/iotaledger/iota.rs/pull/576)) on 2021-05-20

## \[0.5.0]

- Add JWT support and fix default nodes.
  - [b94c0ae1](https://github.com/iotaledger/iota.rs/commit/b94c0ae150c935e3771d12061f534f301d39c33c) add changes file on 2021-05-14
- Validate mnemonic in mnemonicToHexSeed()
  - [e9c89e04](https://github.com/iotaledger/iota.rs/commit/e9c89e049d030fca17adfd63aa161b6911f846d1) add changes file on 2021-05-04
  - [cce6254f](https://github.com/iotaledger/iota.rs/commit/cce6254f37af65a08e5daf53dae6c3f3ba9f9abd) apply version updates ([#538](https://github.com/iotaledger/iota.rs/pull/538)) on 2021-05-09
  - [4b159da2](https://github.com/iotaledger/iota.rs/commit/4b159da25ea0f8db3eea5a6b2748eefb366d1f4d) validate mnemonic in mnemonic_to_hex_seed ([#568](https://github.com/iotaledger/iota.rs/pull/568)) on 2021-05-18
- MQTT uses websocket as default
  Indexation topic with non hex content will be converted to hex automatically
  - [98dad972](https://github.com/iotaledger/iota.rs/commit/98dad972549339d32fba6c06057a9df7582e0b51) Mqtt websocket first iteration ([#561](https://github.com/iotaledger/iota.rs/pull/561)) on 2021-05-17

## \[0.4.0]

- Use camelCase for attributes.
- Add consolidation function.
  - [7e54d183](https://github.com/iotaledger/iota.rs/commit/7e54d183e4e70172dca54b475676f2b1ddeb730f) add changes file on 2021-05-05
- Fix hexToBech32 when bech32\_hrp is optional.
  - [75a073e2](https://github.com/iotaledger/iota.rs/commit/75a073e2dc69a9d065c2bc50732c0b8e104743d9) update readme and use iota_client on 2021-05-03
- Don't overwrite custom provided input range.
  - [294920f7](https://github.com/iotaledger/iota.rs/commit/294920f791714df46ecd9ec09ab6a8f7947f3458) add changes file on 2021-05-04
- Add generateMnemonic and mnemonicToHexSeed.
  - [e9c89e04](https://github.com/iotaledger/iota.rs/commit/e9c89e049d030fca17adfd63aa161b6911f846d1) add changes file on 2021-05-04

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
