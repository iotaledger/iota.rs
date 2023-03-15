# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## Unreleased - YYYY-MM-DD

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security -->

## 2.0.1-rc.8 - 2023-XX-XX

### Added

- `QueryParameters::empty()`;
- MQTT `Error`;
- `Message::HashTransactionEssence`;
- `Response::TransactionEssenceHash`;

### Changed

- `Client::{alias_output_ids, basic_output_ids, foundry_output_ids, nft_output_ids}` will not do automatic pagination if `QueryParameter::Cursor(_)` is provided and return `OutputIdsResponse`;
- `Message::{AliasOutputIds, BasicOutputIds, FoundryOutputIds, NftOutputIds}` return `OutputIdsResponse`;
- `Response::OutputIds` to `Response::OutputIdsResponse`;
- Renamed `Client::get_output_ids_with_pagination()` to `Client::get_output_ids()`;
- All MQTT related functions return an MQTT `Error`;
- Re-export `mqtt` module instead of all its symbols;

## 2.0.1-rc.7 - 2023-03-09

### Added

- `SecretManager::{try_from_mnemonic, try_from_hex_seed}`;
- Derive `Serialize` for `GetAddressesBuilderOptions`, `ClientBlockBuilderOutputAddress`, `ClientBlockBuilderOptions` and `Message`;

### Changed

- Update dependencies;
- `DatabaseProvider` renamed to `StorageProvider`;
- `StrongholdDatabaseProvider` renamed to `StrongholdStorageProvider`;
- `high_level` module moved to `api`;
- Added `#[serde(rename_all = "camelCase")]` to `NodeAuth`;
- `Topic::try_new` parameter from `String` to `impl Into<String>`;
- All `Error` variant related to ISA have been moved to a new ISA `Error` enum;
- All fields of `BurnDto` are now `Option`;
- Custom `Serialize` implementation for `Error`;
- `Message::BuildAliasOutput::state_metadata` from `Option<Vec<u8>>` to `Option<String>`;

### Removed

- `ClientBlockBuilder::get_output_amount_and_address()`;
- `{InputSigningData, InputSigningDataDto}::bech32_address`;
- Added time parameter to `SecretManageExt::sign_transaction_essence()` and `SecretManager::default_sign_transaction_essence()`;

### Fixed

- Input selection can find remainder addresses in expirations and alias state controller or governor;
- Set basic auth when provided;
- Input selection can select basic outputs without ed25519 address in the address unlock condition;
- Automatic input selection with sender and issuer features;
- Automatic alias governance transition does not change output amount in input selection;
- ISA does not select aliases it can't unlock for amount;
- Regex of several MQTT topics;
- ISA makes sure the address is owned on alias Ed25519 requirements;

## 2.0.1-rc.6 - 2023-02-08

### Added

- `TrackedParticipation::answers` field;
- `#[derive(Debug, Clone, Serialize, Deserialize)]` and `#[serde(untagged)]` to `ParticipationEventType`;
- `impl From<&OutputMetadata> for OutputMetadataResponse`;
- `Client::get_included_block_metadata` API endpoint;
- `Message::GetIncludedBlockMetadata` to message interface;
- `Serialize_repr`, `Deserialize_repr` and `#[repr(u8)]` to `ParticipationEventType`;
- `Error::StrongholdMnemonicMissing`;
- `ClientMessageHandler::listen()`;
- `Message::ClearListeners`;
- `#[derive(Eq, PartialEq, Deserialize)]` to `TopicEvent, MqttPayload`;
- `#[derive(Serialize)]` to ` Topic`;
- `impl<'de> Deserialize<'de> for Topic`;
- `Message::SignatureUnlock`;

### Changed

- Complete refactoring of Input Selection;
- Updated dependencies;
- Fields of `Error::InputAddressNotFound` are now named;
- `Event` renamed to `ParticipationEvent`;
- `EventId` renamed to `ParticipationEventId`;
- `EventData` renamed to `ParticipationEventData`;
- `EventStatus` renamed to `ParticipationEventStatus`;
- `EventPayload` renamed to `ParticipationEventPayload`;
- `Client::{subscriber(), subscribe(), unsubscribe()}` take now `&self` instead of `&mut self`;
- `{MqttManager, MqttTopicManager}::new()` take now `&'a Client` instead of `&'a mut Client`;
- `Error::Pow` to `Error::NonceNotFound`;
- `finish_nonce` takes a `F: Fn(&[u8]) -> Option<u64>` instead of a `F: Fn(&[u8]) -> Result<u64, PowError>`;
- Remove `Error` suffix on some `Error` variants;
- `ClientBlockBuilder::allow_burning: bool` to `ClientBlockBuilder::burn: Option<Burn>`;
- `ClientBlockBuilder::with_burning_allowed` to `ClientBlockBuilder::with_burn`;
- `ClientBlockBuilderOptions::allow_burning: Option<bool>` to `ClientBlockBuilderOptions::burn: Option<Burn>`;
- `InputSigningDataDto::output_metadata` from `OutputMetadata` to `OutputMetadataDto`;
- Renamed `MessageHandler::handle()` to `MessageHandler::send_message()`, removed the `response_tx` parameter and returned the `Response`;

### Removed

- `participation`, `indexer` plugin API types and `OutputMetadata`, moved to `iota-types`;
- `message_handler::send_message()`;

### Fixed

- Possible infinite loop in pow::finish_multi_threaded_pow();

## 2.0.1-rc.5 - 2022-12-20

### Added

- `alias_id_to_bech32()` to utils;
- `nft_id_to_bech32()` to utils;
- `Response::Bech32Address()` to message interface;
- `Message::{AliasIdToBech32(), NftIdToBech32()}` to message interface;
- `Client::finish_block_builder()`;

### Changed

- Abort the sync nodes task through its handle instead of a oneshot channel;
- Update network info also from unhealthy nodes if ignore_node_health is set to true;
- `finish_pow()` and `do_pow()` now accept optional parents and always do PoW, independent of the local PoW setting in Client;
- `do_pow()` has been made private;
- Renamed participation `Answers` to `QuestionStatus`;

### Removed

- Sync nodes `select!`;
- `Response::{HexToBech32(), AliasIdToBech32()}` from message interface;
- `ClientMinerBuilder` and `ClientMiner`;
- `Client::get_pow_provider` method;

## 2.0.1-rc.4 - 2022-11-22

### Added

- `OutputMetadata::new()` and all field getters;
- `participation` feature with routes, responses and types;
- `Http client` allow setting User-Agent header for requests;

### Changed

- Use `OutputWithMetadataResponse` instead of `OutputResponse`;
- `ClientBlockBuilder::{with_output, with_output_hex, set_options}` made async;
- `Client::{hex_to_bech32, hex_public_key_to_bech32_address}` made async;
- `Client::{get_network_info, get_protocol_parameters, get_protocol_version, get_network_name, get_network_id, get_bech32_hrp, get_min_pow_score, get_below_max_depth, get_rent_structure, get_time_checked, get_token_supply}` made async;
- All fields of `OutputMetadata` are now private;
- `InputSigningData::output_id()` is not fallible anymore;
- Replace `ClientBuilder::with_node_sync_disabled()` by `ClientBuilder::with_ignore_node_health()`;
- Replace `NodeManagerBuilder.node_sync_enabled` by `NodeManagerBuilder.ignore_node_health`;
- Use new `RentStructureDto` and `ProtocolParametersDto` types;
- ClientBuilder no longer has `#[serde(deny_unknown_fields)]` for backwards compatibility;

### Removed

- `OutputMetadata::{transaction_id, output_index}` replaced with `OutputMetadata::output_id`;

### Fixed

- `get_time_checked()` in wasm;

## 2.0.1-rc.3 - 2022-10-25

### Added

- `GetProtocolParameters` to the message interface;
- Max length checks for transaction payload and essence, so the block size isn't exceeded, even with 8 parents;
- `{PreparedTransactionData, SignedTransactionData, RemainderData, InputSigningData}::try_from_dto_unverified`;
- `NetworkInfo::latest_milestone_timestamp` field;
- `Client::try_get_outputs_metadata`;

### Changed

- Update dependencies;
- Use new `iota-types` crate instead of `bee-api-types` and `bee-block`;
- Use new `iota-pow` crate instead of `bee-pow`;
- `Message::{SignTransaction, BlockId, TransactionId}` now uses unverified DTO conversions;
- `get_time_checked` has been made sync;
- `get_time_checked` doesn't call `get_info` anymore but returns the cached `latest_milestone_timestamp`;
- Syncing nodes also updates the `latest_milestone_timestamp`;
- `NODE_SYNC_INTERVAL` from 15s to 60s;
- `fn derive_key_from_password(password: &str) -> EncryptionKey` to `key_provider_from_password(password: &str) -> KeyProvider`;
- `generate_addresses()` to accept `Option<GenerateAddressOptions>`;
- Renamed `GenerateAddressOptions` to `GenerateAddressOptions` and replaced its `syncing` field with `ledger_nano_prompt`;

### Fixed

- Add alias and nfts output in `try_select_input` to the inputs, when required for an unlock condition of an input;
- Healthy node list, when building with `.with_local_pow(false)`;
- Infinite PoW loop on wasm if min pow score == 0;
- Checks for unlock address;
- Missing options in `GetAddressesBuilder::set_options()`;

## 2.0.1-rc.2 - 2022-09-29

### Added

- `NetworkInfoDto`;

### Changed

- Update dependencies;
- Return `NetworkInfoDto` instead of `NetworkInfo` in message_interface;
- Use `#[cfg(target_family = "wasm")]` instead of `#[cfg(feature = "wasm")]`;

## 2.0.1-rc.1 - 2022-09-28

Re-release as RC.

### Fixed

- Check correct alias and nft addresses of new outputs in input selection;

## 2.0.0 - 2022-09-27

### Added

- `finish_pow` function to avoid the caller having to check for wasm family;
- Debug logs for GET requests;
- Added `#[serde(rename_all = "camelCase")]` to enums and aliases for `SecretManagerDto` and `LedgerDeviceType` fields;

### Changed

- Update dependencies;
- PoW node feature from `PoW` to `pow` to match TIP25;
- Made `finish_multi_threaded_pow` and `finish_single_threaded_pow` private;
- Cleaned up error enum;
- Replaced `force_use_all_inputs` in `try_select_inputs()` with `mandatory_inputs`;
- Rename `inputs` parameter in `try_select_inputs()` to `additional_inputs`;
- Adapt the whole codebase to bee's packable visitor changes;
- Made a lot of functions/methods async;
- Renamed `synced_nodes` to `healthy_nodes`;
- Renamed `unsynced_nodes()` to `unhealthy_nodes()`;
- `UrlAuthError(String)` to `UrlAuthError(&'static str)`;

### Removed

- `GetAddressBuilder` and `AddressBalance`;
- `offline` field in `ClientBuilder`;

### Fixed

- Check expiration for remainder address;
- Input selection when only native tokens are remaining;
- Fix cpufeatures advisory;
- Input selection for sender and issuer features features;

## 2.0.0-beta.3 - 2022-08-30

### Changed

- Update dependencies;
- `StrongholdAdapterBuilder::try_build` now takes a `snapshot_path: <P: AsRef<Path>>` instead of a `snapshot_path: PathBuf`;
- Rename `StrongholdAdapterBuilder::try_build` to `StrongholdAdapterBuilder::build`;
- `NetworkInfo::min_pow_score` from `Option<f64>` to `Option<u32>`;
- `do_pow`'s `min_pow_score` parameters from `f64` to `u32`;
- `ClientMiner::nonce`'s `target_score` parameters from `f64` to `u32`;
- `Client::get_min_pow_score`'s return from `Result<f64>` to `Result<u32>`;
- `Response::MinPowScore` from `f64` to `u32`;
- Pass a visitor to all `Packable::unpack*` methods;

### Fixed

- Fix Wasm compilation with iota-crypto curl-p feature;
- Fix missing reference unlocks with ledger nano secret manager;

## 2.0.0-beta.2 - 2022-08-22

### Added

- Add `SingleThreadedMiner`, `SingleThreadedMinerBuilder` for Wasm;
- Add `snapshot_path: Option<&Path>` parameters to `StrongholdAdapter::write_stronghold_snapshot`;

### Changed

- Update dependencies;
- Use `NativeTokensBuilder::finish_vec` to lift some unnecessary 64 Native Tokens limits;
- Change `Client::get_pow_provider()` to return `SingleThreadedMiner` on Wasm, otherwise `ClientMiner`, to fix `promote`, `reattach`, and `retry_until_included` for Wasm;
- Change `ClientMinerBuilder::default()` to match `new()` and default `local_pow` to `true`;
- Rename `finish_pow` to `finish_multi_threaded_pow`;
- Rename `finish_single_thread_pow` to `finish_single_threaded_pow`;
- Rename `minimum_storage_deposit` to `minimum_storage_deposit_basic_output`;
- Accept `GenerateAddressesOptions` in `consolidate_funds()` instead of `account_index` and `address_range`;
- Use `chacha::{aead_encrypt, aead_decrypt}` from `crypto.rs` in stronghold's `db` module;

### Removed

- Removed `snapshot_loaded` field from StrongholdAdapter;
- Removed `outputs()` field from GetAddressBuilder;
- Stronghold's `encryption` module;
- Primary node usage;

### Fixed

- Fix `Client::get_time_checked()` panic on Wasm;
- Fix `Client::get_pow_provider()` not using the `worker_count`;
- Fix `PoW` feature name in node syncing;
- Fix automatic input selection for recursively owned alias and nft outputs;

## 2.0.0-beta.1 - 2022-08-11

First beta release.
