# Changelog

## \[3.0.0-rc.10]

- IAliasOutputBuilderOptions::stateMetadata is now a HexEncodedString instead of Uint8Array.
  - [7a3f8b0](https://github.com/iotaledger/iota.rs/commit/7a3f8b03be42a581ea9af44f57f46304a93738a9) CI improvements ([#1633](https://github.com/iotaledger/iota.rs/pull/1633)) on 2023-03-20

- Removed `IInputSigningData::bech32Address`;
  - [7a3f8b0](https://github.com/iotaledger/iota.rs/commit/7a3f8b03be42a581ea9af44f57f46304a93738a9) CI improvements ([#1633](https://github.com/iotaledger/iota.rs/pull/1633)) on 2023-03-20

- `Burn` fields are now optional.
  `Burn::nativeTokens` is now an array.
  - [7a3f8b0](https://github.com/iotaledger/iota.rs/commit/7a3f8b03be42a581ea9af44f57f46304a93738a9) CI improvements ([#1633](https://github.com/iotaledger/iota.rs/pull/1633)) on 2023-03-20

- Add `Client::hashTransactionEssence()`;
  - [7a3f8b0](https://github.com/iotaledger/iota.rs/commit/7a3f8b03be42a581ea9af44f57f46304a93738a9) CI improvements ([#1633](https://github.com/iotaledger/iota.rs/pull/1633)) on 2023-03-20

- Fix install command;
  - [7a3f8b0](https://github.com/iotaledger/iota.rs/commit/7a3f8b03be42a581ea9af44f57f46304a93738a9) CI improvements ([#1633](https://github.com/iotaledger/iota.rs/pull/1633)) on 2023-03-20

- ### Added

- `OutputIdsResponse`;

### Changed

- `Client::{aliasOutputIds, basicOutputIds, foundryOutputIds, nftOutputIds}` will not do automatic pagination if `QueryParameter::Cursor(_)` is provided and return type from `string[]` to `OutputIdsResponse`;
- [7a3f8b0](https://github.com/iotaledger/iota.rs/commit/7a3f8b03be42a581ea9af44f57f46304a93738a9) CI improvements ([#1633](https://github.com/iotaledger/iota.rs/pull/1633)) on 2023-03-20

## \[3.0.0-rc.9]

- Merged `IAuth::{username, password}` into `IAuth::basicAuthNamePwd`;
  Set basic auth when provided;
  - [7c7b9b7](https://github.com/iotaledger/iota.rs/commit/7c7b9b7a6ccd3215edeecb6056e59db40e47e307) Fix regex of several MQTT topics ([#1594](https://github.com/iotaledger/iota.rs/pull/1594)) on 2023-02-21

## \[3.0.0-rc.8]

- Add `Burn` interface.
  Replace `IBuildBlockOptions::allowBurning` with `IBuildBlockOptions::burn`.
  - [2ba6566](https://github.com/iotaledger/iota.rs/commit/2ba65666264007514fe1ec319388725719d0830b) Prepare releases ([#1537](https://github.com/iotaledger/iota.rs/pull/1537)) on 2023-02-08
- Fixed returned JSON value for `IInputSigningData`;
  Renamed `IInputSigningData::outputMetaData` to `IInputSigningData::outputMetadata`;
  Changed `ISegment::bs` from `Uint8Array` to `number[]` so that the serialization corresponds to what is expected;
  - [2ba6566](https://github.com/iotaledger/iota.rs/commit/2ba65666264007514fe1ec319388725719d0830b) Prepare releases ([#1537](https://github.com/iotaledger/iota.rs/pull/1537)) on 2023-02-08
- Add `signatureUnlock()`.
  - [2ba6566](https://github.com/iotaledger/iota.rs/commit/2ba65666264007514fe1ec319388725719d0830b) Prepare releases ([#1537](https://github.com/iotaledger/iota.rs/pull/1537)) on 2023-02-08
- Move to N-API 6 builds.
  - [2ba6566](https://github.com/iotaledger/iota.rs/commit/2ba65666264007514fe1ec319388725719d0830b) Prepare releases ([#1537](https://github.com/iotaledger/iota.rs/pull/1537)) on 2023-02-08

## \[3.0.0-rc.7]

- Fix MQTT multiple events when .listen() is called multiple times.
  Made `Client::listen()` async.
  Added `Client::clearListeners()`.
  - [743a7c7](https://github.com/iotaledger/iota.rs/commit/743a7c70e59436ff09a2290a49538206d4a97509) Add listen and clearListeners for MQTT to message interface ([#1518](https://github.com/iotaledger/iota.rs/pull/1518)) on 2023-01-26

## \[3.0.0-rc.6]

- Add `u8` representation to serialization and deserialization for `ParticipationEventType`.
  - [3534a07](https://github.com/iotaledger/iota.rs/commit/3534a079afeefadd9f2879398d9e2b286209f8a6) Add change file ([#1494](https://github.com/iotaledger/iota.rs/pull/1494)) on 2023-01-17
- Add `Client::getIncludedBlockMetadata`.
  - [3534a07](https://github.com/iotaledger/iota.rs/commit/3534a079afeefadd9f2879398d9e2b286209f8a6) Add change file ([#1494](https://github.com/iotaledger/iota.rs/pull/1494)) on 2023-01-17
- Fix infinite loop when the minimum PoW score is 0 (often in private tangles).
  - [3534a07](https://github.com/iotaledger/iota.rs/commit/3534a079afeefadd9f2879398d9e2b286209f8a6) Add change file ([#1494](https://github.com/iotaledger/iota.rs/pull/1494)) on 2023-01-17

## \[3.0.0-rc.5]

- Add `aliasIdToBech32()`.
  - [4693449](https://github.com/iotaledger/iota.rs/commit/469344962cec1db0b94f0d0fc6fe6a514148001f) Add changelog ([#1448](https://github.com/iotaledger/iota.rs/pull/1448)) on 2022-12-14
- Add `computeAliasId()` and `computeNftId()` functions.
  - [4693449](https://github.com/iotaledger/iota.rs/commit/469344962cec1db0b94f0d0fc6fe6a514148001f) Add changelog ([#1448](https://github.com/iotaledger/iota.rs/pull/1448)) on 2022-12-14
- Add `computeFoundryId()`.
  - [4693449](https://github.com/iotaledger/iota.rs/commit/469344962cec1db0b94f0d0fc6fe6a514148001f) Add changelog ([#1448](https://github.com/iotaledger/iota.rs/pull/1448)) on 2022-12-14
- Add `nftIdToBech32()`.
  - [4693449](https://github.com/iotaledger/iota.rs/commit/469344962cec1db0b94f0d0fc6fe6a514148001f) Add changelog ([#1448](https://github.com/iotaledger/iota.rs/pull/1448)) on 2022-12-14
- Replaced `nodeSyncEnabled` by `ignoreNodeHealth`.
  - [4693449](https://github.com/iotaledger/iota.rs/commit/469344962cec1db0b94f0d0fc6fe6a514148001f) Add changelog ([#1448](https://github.com/iotaledger/iota.rs/pull/1448)) on 2022-12-14
- Update network info from unhealty nodes if ignoreNodeHealth is set to true.
  - [4693449](https://github.com/iotaledger/iota.rs/commit/469344962cec1db0b94f0d0fc6fe6a514148001f) Add changelog ([#1448](https://github.com/iotaledger/iota.rs/pull/1448)) on 2022-12-14

## \[3.0.0-rc.4]

- Fix rebuild script.
  - [6f3f9a7](https://github.com/iotaledger/iota.rs/commit/6f3f9a7f1b448bdf59bd0634ad822f43d5706fb1) Fix rebuild script ([#1344](https://github.com/iotaledger/iota.rs/pull/1344)) on 2022-10-31

## \[3.0.0-rc.3]

- Run tsc before publishing.
  - [f9efaf6](https://github.com/iotaledger/iota.rs/commit/f9efaf6384d93d9f08d81be6043923f2b13cd4dd) Fix nodejs package and path ([#1341](https://github.com/iotaledger/iota.rs/pull/1341)) on 2022-10-31

## \[3.0.0-rc.2]

- Fix build script filename.
  - [eef5530](https://github.com/iotaledger/iota.rs/commit/eef55302e5b75e298e734352d3ec64f601b42eee) Fix build script filename ([#1331](https://github.com/iotaledger/iota.rs/pull/1331)) on 2022-10-27

## \[3.0.0-rc.1]

- Add alias and nfts output in `try_select_input` to the inputs, when required for an unlock condition of an input.
  - [c18d9d7](https://github.com/iotaledger/iota.rs/commit/c18d9d77e8a18aef8e47c386ebe0736493bd134f) Rename `GenerateAddressMetadata` to `GenerateAddressOptions` ([#1330](https://github.com/iotaledger/iota.rs/pull/1330)) on 2022-10-25
- Rename `IGenerateAddressOptions` to `IGenerateAddressOptions` and replace its syncing field with ledgerNanoPrompt.
  - [c18d9d7](https://github.com/iotaledger/iota.rs/commit/c18d9d77e8a18aef8e47c386ebe0736493bd134f) Rename `GenerateAddressMetadata` to `GenerateAddressOptions` ([#1330](https://github.com/iotaledger/iota.rs/pull/1330)) on 2022-10-25
- Fix prebuild scripts.
  - [c18d9d7](https://github.com/iotaledger/iota.rs/commit/c18d9d77e8a18aef8e47c386ebe0736493bd134f) Rename `GenerateAddressMetadata` to `GenerateAddressOptions` ([#1330](https://github.com/iotaledger/iota.rs/pull/1330)) on 2022-10-25
- Add `getTokenSupply` and `getProtocolParameters`.
  - [c18d9d7](https://github.com/iotaledger/iota.rs/commit/c18d9d77e8a18aef8e47c386ebe0736493bd134f) Rename `GenerateAddressMetadata` to `GenerateAddressOptions` ([#1330](https://github.com/iotaledger/iota.rs/pull/1330)) on 2022-10-25

## \[3.0.0-rc.0]

- Fix types in networkInfo.
  - [061f773](https://github.com/iotaledger/iota.rs/commit/061f7737d899f5a3c746fb5b1e3ac36669b2580f) Fix types in networkInfo ([#1282](https://github.com/iotaledger/iota.rs/pull/1282)) on 2022-09-28

## \[3.0.0-alpha.11]

- Check correct alias and nft addresses of new outputs in input selection.
  - [ba592b7](https://github.com/iotaledger/iota.rs/commit/ba592b7bec186d922a68e9e72a63417a8298e8a4) Check correct alias and nft addresses for new outputs in input selection ([#1279](https://github.com/iotaledger/iota.rs/pull/1279)) on 2022-09-28

## \[3.0.0-alpha.10]

- Rename submitPayload() to postBlockPayload().
  - [509535f](https://github.com/iotaledger/iota.rs/commit/509535f6494f384e2fff863fb0637d808928a428) Fix dasel install ([#1276](https://github.com/iotaledger/iota.rs/pull/1276)) on 2022-09-27
- Remove "offline" field from client options.
  - [509535f](https://github.com/iotaledger/iota.rs/commit/509535f6494f384e2fff863fb0637d808928a428) Fix dasel install ([#1276](https://github.com/iotaledger/iota.rs/pull/1276)) on 2022-09-27
- Rename `unsyncedNodes()` to `unhealthyNodes()`.
  - [509535f](https://github.com/iotaledger/iota.rs/commit/509535f6494f384e2fff863fb0637d808928a428) Fix dasel install ([#1276](https://github.com/iotaledger/iota.rs/pull/1276)) on 2022-09-27

## \[3.0.0-alpha.9]

- Fix input selection when only native tokens are remaining.
  - [4c3b71b](https://github.com/iotaledger/iota.rs/commit/4c3b71b13d16ea543434ef2e652734f9d79c30b0) Docs - Develop - Organize examples ([#1228](https://github.com/iotaledger/iota.rs/pull/1228)) on 2022-09-21
- Add typescript to dependencies.
  - [4c3b71b](https://github.com/iotaledger/iota.rs/commit/4c3b71b13d16ea543434ef2e652734f9d79c30b0) Docs - Develop - Organize examples ([#1228](https://github.com/iotaledger/iota.rs/pull/1228)) on 2022-09-21
- Use `Uint8Array` over `number[]` in `IAliasOutputBuilderOptions` and other places to better reflect the type requirements.
  - [4c3b71b](https://github.com/iotaledger/iota.rs/commit/4c3b71b13d16ea543434ef2e652734f9d79c30b0) Docs - Develop - Organize examples ([#1228](https://github.com/iotaledger/iota.rs/pull/1228)) on 2022-09-21

## \[3.0.0-alpha.8]

- Fix missing reference unlocks with ledger nano secret manager.
  - [f9bc46f](https://github.com/iotaledger/iota.rs/commit/f9bc46f9dbf55dad1a6df771e9921646772d88a3) Bump ledger nano version ([#1224](https://github.com/iotaledger/iota.rs/pull/1224)) on 2022-09-02

## \[3.0.0-alpha.7]

- Accept `IGenerateAddressesOptions` in `consolidateFunds()` instead of `accountIndex` and `addressRange`.
  - [218352d](https://github.com/iotaledger/iota.rs/commit/218352de2a58de56c67892a0bfe2aab104793334) Fix primary node ([#1205](https://github.com/iotaledger/iota.rs/pull/1205)) on 2022-08-22
- Fix PoW feature name.
  - [218352d](https://github.com/iotaledger/iota.rs/commit/218352de2a58de56c67892a0bfe2aab104793334) Fix primary node ([#1205](https://github.com/iotaledger/iota.rs/pull/1205)) on 2022-08-22

## \[3.0.0-alpha.6]

- Rename getLedgerStatus to getLedgerNanoStatus.
  - [c744d77](https://github.com/iotaledger/iota.rs/commit/c744d777192e0d4dd67f374b0617ec8f6b35bac3) Rename get_ledger_status to get_ledger_nano_status ([#1185](https://github.com/iotaledger/iota.rs/pull/1185)) on 2022-08-09

- Rename indexer query parameters:

- "hasStorageReturnCondition" => "hasStorageDepositReturn"

- "storageReturnAddress" => "storageDepositReturnAddress"

- "hasExpirationCondition" => "hasExpiration"

- "hasTimelockCondition" => "hasTimelock"

- [c744d77](https://github.com/iotaledger/iota.rs/commit/c744d777192e0d4dd67f374b0617ec8f6b35bac3) Rename get_ledger_status to get_ledger_nano_status ([#1185](https://github.com/iotaledger/iota.rs/pull/1185)) on 2022-08-09

## \[3.0.0-alpha.5]

- Don't include tests in published package.
  - [78a29ec](https://github.com/iotaledger/iota.rs/commit/78a29ec506516a8263920ee65f356e7992c898aa) Don't include tests in published package ([#1180](https://github.com/iotaledger/iota.rs/pull/1180)) on 2022-08-08

## \[3.0.0-alpha.4]

- Fix IBuildBlockOptions.
  - [7a5c445](https://github.com/iotaledger/iota.rs/commit/7a5c4459650e8b9f0b551178474d885bc7aad57e) Add debug logs for message_interface ([#1177](https://github.com/iotaledger/iota.rs/pull/1177)) on 2022-08-05

## \[3.0.0-alpha.3]

- Update @iota/types to fix types.
  - [e86dfa2](https://github.com/iotaledger/iota.rs/commit/e86dfa22fc36e8e09e458a9147bf5fc5a2222860) Update @iota/types ([#1175](https://github.com/iotaledger/iota.rs/pull/1175)) on 2022-08-05
- Accept hex encoded strings for tag and data fields.
  - [e86dfa2](https://github.com/iotaledger/iota.rs/commit/e86dfa22fc36e8e09e458a9147bf5fc5a2222860) Update @iota/types ([#1175](https://github.com/iotaledger/iota.rs/pull/1175)) on 2022-08-05

## \[3.0.0-alpha.2]

- Improve handling for utxo chains in input selection.

## \[3.0.0-alpha.1]

- Rename generateBlock to buildAndPostBlock.
  - [4fcb685](https://github.com/iotaledger/iota.rs/commit/4fcb685082d6741a5e8c00ae44e1e70503b586d7) Pow hrp renaming ([#1163](https://github.com/iotaledger/iota.rs/pull/1163)) on 2022-07-27
- Rename PoW to Pow and HRP to Hrp.
  - [4fcb685](https://github.com/iotaledger/iota.rs/commit/4fcb685082d6741a5e8c00ae44e1e70503b586d7) Pow hrp renaming ([#1163](https://github.com/iotaledger/iota.rs/pull/1163)) on 2022-07-27

## \[3.0.0-alpha.0]

- Pre-release of the Stardust bindings of iota.rs for Node.JS
  - [bdb75aa](https://github.com/iotaledger/iota.rs/commit/bdb75aab24ccbde20c439559427d4c14437a3c96) Nodejs docs and examples ([#1128](https://github.com/iotaledger/iota.rs/pull/1128)) on 2022-07-18

## \[2.2.3]

- Fixes getIncludedMessage().
  - [070ea632](https://github.com/iotaledger/iota.rs/commit/070ea632c538108484958b052a2568afa73540a0) Fix getIncludedMessage() ([#780](https://github.com/iotaledger/iota.rs/pull/780)) on 2022-01-10

## \[2.2.2]

- Updated dependencies to fix compilation.
  - [0838d48e](https://github.com/iotaledger/iota.rs/commit/0838d48e4683ab8d10f685c444f4ed097cf76493) Update dependencies ([#767](https://github.com/iotaledger/iota.rs/pull/767)) on 2021-12-09

## \[2.2.1]

- Fixed binaries upload
  - [1b5044b1](https://github.com/iotaledger/iota.rs/commit/1b5044b17eb63cd33cf0cbbc72fb08286b64f3ef) fix workflow for binaries, clean wasm readme ([#738](https://github.com/iotaledger/iota.rs/pull/738)) on 2021-11-04

## \[2.2.0]

- Changed "discovered" to "autopeered" in the result from getPeers().
  - [f43b9dea](https://github.com/iotaledger/iota.rs/commit/f43b9deac867cc52bff92a6c400ff54cbd06add2) add change file and updated covector workflow on 2021-11-03

## \[2.1.1]

- Removed dependencies that failed to compile on a Raspberry Pi.
  - [f3692e13](https://github.com/iotaledger/iota.rs/commit/f3692e1385aa5e45b61e6222f738415af0a264ba) Lock dependencies to a specific version ([#719](https://github.com/iotaledger/iota.rs/pull/719)) on 2021-10-21

## \[2.1.0]

- Updated default testnet nodes
  - [4f060388](https://github.com/iotaledger/iota.rs/commit/4f060388a19ece1deee6b54748b13498078d0cef) Wasm binding ([#631](https://github.com/iotaledger/iota.rs/pull/631)) on 2021-09-27
- Added fallback to local PoW if no provided node has remote PoW enabled
  - [4f060388](https://github.com/iotaledger/iota.rs/commit/4f060388a19ece1deee6b54748b13498078d0cef) Wasm binding ([#631](https://github.com/iotaledger/iota.rs/pull/631)) on 2021-09-27

## \[2.0.0]

- Changed input() to accept the output id as string instead of the transaction id and the output index
  Add functionality for offline signing: offlineMode(), findInputs(), prepareTransaction(), signTransaction(), finishMessage()
  - [9489d319](https://github.com/iotaledger/iota.rs/commit/9489d319e971a18f44e4c88d38789f6b6b4d4d7e) Nodejs offline signing ([#674](https://github.com/iotaledger/iota.rs/pull/674)) on 2021-09-23

## \[1.0.1]

- Fix consolidation for higher start indexes
  - [3ca3a2d7](https://github.com/iotaledger/iota.rs/commit/3ca3a2d7ee9a25556ca7fe2a4eb1221bdb6accfe) Handle higher start indexes in consolidation ([#666](https://github.com/iotaledger/iota.rs/pull/666)) on 2021-09-08

## \[1.0.0]

- Fixed custom provided API timeouts
  - [5f8fd262](https://github.com/iotaledger/iota.rs/commit/5f8fd262526aa09e2f548b3711964ea8fc18bc0b) Fix API timeouts for node.js binding, update message format for MQTT and default reconnect attempts ([#652](https://github.com/iotaledger/iota.rs/pull/652)) on 2021-08-25
- Return messages in MQTT in the same format as from all other functions (byte arrays converted to hex strings)
  Changed default max reconnection attempts to be unlimited
  - [98dad972](https://github.com/iotaledger/iota.rs/commit/98dad972549339d32fba6c06057a9df7582e0b51) Mqtt websocket first iteration ([#561](https://github.com/iotaledger/iota.rs/pull/561)) on 2021-05-17
  - [24a4b3fb](https://github.com/iotaledger/iota.rs/commit/24a4b3fbffd2a5108eecbf43308fe8bbb9ddfe4a) apply version updates ([#563](https://github.com/iotaledger/iota.rs/pull/563)) on 2021-05-18
  - [5f8fd262](https://github.com/iotaledger/iota.rs/commit/5f8fd262526aa09e2f548b3711964ea8fc18bc0b) Fix API timeouts for node.js binding, update message format for MQTT and default reconnect attempts ([#652](https://github.com/iotaledger/iota.rs/pull/652)) on 2021-08-25

## \[0.7.0]

- Add hexPublicKeyToBech32Address method
  - [572bc72b](https://github.com/iotaledger/iota.rs/commit/572bc72b7a945528475132dbb9e86e0b21f08796) Add hexPublicKeyToBech32Address on 2021-08-13

## \[0.6.6]

- Import TextEncoder for older Node.js versions
  - [2c718365](https://github.com/iotaledger/iota.rs/commit/2c71836544ddd63571e6fd47cdf555a94afb322e) Import TextEncoder for older Node.js versions on 2021-08-06

## \[0.6.5]

- Order parent messages always, fix POST JSON request.
  - [1d6a84c4](https://github.com/iotaledger/iota.rs/commit/1d6a84c4fde2c1c0e46380509f9c66123842ca72) add change file on 2021-07-14

## \[0.6.4]

- Fixed receipt payload conversion.
  - [86857df1](https://github.com/iotaledger/iota.rs/commit/86857df1815dd26ea7d2d62b410efe1c76328e93) update dependencies on 2021-07-12

## \[0.6.3]

- Fixed ts interfaces
  - [49100701](https://github.com/iotaledger/iota.rs/commit/491007014eedce0cf8d90d4272b547ae83854094) add change file on 2021-07-07

## \[0.6.2]

- Fixed retryUntilIncluded.
  - [1703e299](https://github.com/iotaledger/iota.rs/commit/1703e299cc973d194ee0e82de87fec6c347afcf2) Fix retry_until_included ([#599](https://github.com/iotaledger/iota.rs/pull/599)) on 2021-07-01

## \[0.6.1]

- Add permanode option.
  - [8537f390](https://github.com/iotaledger/iota.rs/commit/8537f3901a3e9860df32aa982943de171670da3e) Add basic permanode support ([#586](https://github.com/iotaledger/iota.rs/pull/586)) on 2021-06-03

## \[0.6.0]

- Added getMessageId function.
  - [b1935f36](https://github.com/iotaledger/iota.rs/commit/b1935f36542ed805209c1ce684aa12fc1de8af7a) nodejs add getMessageId on 2021-05-25

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

- Update minPoWScore in node info.
  - [b578d23a](https://github.com/iotaledger/iota.rs/commit/b578d23a9c212bc3851d4c3c4a8292af1fbd34de) update nodejs docs on 2021-04-11

## \[0.3.0]

- Added functions to convert addresses from bech32 to hex and vice versa.
  - [115184a8](https://github.com/iotaledger/iota.rs/commit/115184a8c712e3432cc960273278780ddc1b768a) Added hex_to_bech32 and bech32\_to_hex methods ([#471](https://github.com/iotaledger/iota.rs/pull/471)) on 2021-04-07
- Add optional quorum, primaryNode, primaryPowNode and return url together with the node info when calling getInfo().
  - [7a9ef60f](https://github.com/iotaledger/iota.rs/commit/7a9ef60fea1c865d59c744e0f6cc54371a4cebda) Node manager ([#457](https://github.com/iotaledger/iota.rs/pull/457)) on 2021-04-08
- Return addresses bech32 encoded also for balance and output endpoints.
  - [a2e09d1a](https://github.com/iotaledger/iota.rs/commit/a2e09d1a329404cdaf74890eae562fe992483b10) return address bech32 encoded for balance and outputs on 2021-04-07

## \[0.2.0]

- Fix import and installation + example in readme.
  - [5c96ab33](https://github.com/iotaledger/iota.rs/commit/5c96ab3379d992343c11ba3d7ae35ecaa22b4a0a) Fix import and installation + example in readme. ([#431](https://github.com/iotaledger/iota.rs/pull/431)) on 2021-03-18

## \[0.1.0]

- Added optional gapLimit to getBalance.
  - [cf1405e5](https://github.com/iotaledger/iota.rs/commit/cf1405e54383d71fac84c421b1b945cbe4959259) add gap_limit to GetBalanceBuilder ([#427](https://github.com/iotaledger/iota.rs/pull/427)) on 2021-03-18
