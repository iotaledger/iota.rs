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

## 1.0.0-rc.4 - 20XX-XX-XX

### Added

- `Output::{as_treasury, as_basic, as_alias, as_foundry, as_nft}` methods;
- `Address::{as_ed25519, as_alias, as_nft}` methods;

## 1.0.0-rc.3 - 2022-11-22

### Added

- `NftAddress::into_nft_id` method;
- `AliasAddress::into_alias_id` method;
- `Output::{is_treasury, is_basic, is_alias, is_foundry, is_nft}` methods;
- `RentStructure::new` and getters;
- serde aliases to `ProtocolParameters` and `RentStructure` fields for backwards compatibility;

### Changed

- Replaced `RentStructureResponse` with `RentStructureDto`;
- Replaced `ProtocolResponse` with `ProtocolParametersDto`;
- `RentStructure` fields have been made private;
- Renamed `RentStructure::key_factor` to `RentStructure::byte_factor_key`;
- Renamed `RentStructure::data_factor` to `RentStructure::byte_factor_data`;

## 1.0.0-rc.2 - 2022-11-09

### Added

- `OutputResponse` enum with `Json` and `Raw` variants;
- `AliasOutput::alias_id_non_null` and `NftOutput::nft_id_non_null` methods;

### Changed

- Renamed `OutputResponse` to `OutputWithMetadataResponse`;
- `OutputId::hash` now takes a `&self`;
- `impl From<OutputId> for {AliasId, NftId}` now takes an `&OutputId`;
- `{AliasId, NftId, ChainId}::or_from_output_id` now takes an `&OutputId`;
- `AliasOutput::alias_address` now takes an `&OutputId`;
- `NftOutput::nft_address` now takes an `&OutputId`;

### Fixed

- Packing order of `v_byte_factor_data` and `v_byte_factor_key`;

## 1.0.0-rc.1 - 2022-10-25

First release based on `bee-api-types` and `bee-block`.

### Added

- `alias_address()` to `AliasOutput`;
- `nft_address()` to `NftOutput`;
- `required_and_unlocked_address()` to `Output`;
- `output_id()` to `OutputMetadataResponse`;
- Independent `From`/`TryFrom` implementations for all unlock conditions DTOs;
- `OutputId::null`;

### Changed

- Reduced `DATA_LENGTH_RANGE` of `TaggedDataPayload`;

<!-- We include the past changelogs of `bee-api-types` and `bee-block` for reference as they have been merged into the client repository as a new crate.

# bee-api-types

## 1.0.1 - 2022-09-28

### Changed

- Updated dependencies;

## 1.0.0 - 2022-09-27

### Changed

- Updated dependencies;
- Make `{LatestMilestoneResponse, ConfirmedMilestoneResponse}::{timestamp, milestone_id}` optional;

## 1.0.0-beta.7 - 2022-08-30

### Changed

- `ProtocolResponse::min_pow_score` from `f64` to `u32`;

## 1.0.0-beta.6 - 2022-08-30

### Changed

- Updated dependencies;

## 1.0.0-beta.5 - 2022-08-15

### Changed

- Updated dependencies;
- Added the `"axum"` feature to gate the `axum` dependency, `IntoResponse` impls;

## 1.0.0-beta.4 - 2022-07-26

### Changed

- Bump `bee-block` to `v1.0.0-beta.4`;

## 1.0.0-beta.3 - 2022-07-25

### Changed

- Renamed:
    - `bech32HRP` to `bech32Hrp`;
    - `minPoWScore` to `minPowScore`;

## 1.0.0-beta.2 - 2022-07-21

### Changed

- Bump `bee-block` dependency;

## 1.0.0-beta.1 - 2022-07-20

First beta release.

# bee-block

## 1.0.1 - 2022-09-28

### Added

- `#[serde(rename_all = "camelCase")]` to `ProtocolParameters` and `RentStructure`;
- `From` derivation on DTO enums;

### Changed

- `pub` expose `ReceiptMilestoneOptionDto` and `RegularTransactionEssenceDto`;

## 1.0.0 - 2022-09-27

### Added

- `helper` module with a `network_name_to_id` function;
- `Error::NetworkIdMismatch`;
- `ProtocolParameters::network_id` method;
- `BlockBuilder::with_protocol_version`;
- `Output::verify_storage_deposit` new parameter `token_supply: u64`;

### Changed

- Updated dependencies;
- `block` module is now public;
- `Packable::UnpackVisitor` from `()` to `ProtocolParameters` for a lot of types;
- `ProtocolParameters::version` renamed to `ProtocolParameters::protocol_version`;
- Some DTO `TryFrom` have been changed to functions as they needed another parameters;
- `Output`s amount is now simply an `u64`;
- `OutputBuilder`s `finish` now takes a `token_supply: u64` parameter; 
- Adapt the `rand` module to all these changes;
- All DTO conversion free functions have been made type methods;
- `DEFAULT_BYTE_COST` from 500 to 100;
- Implement `Default` for `ProtocolParameters` and `RentStructure`;
- Return `U256` instead of `&U256` for `NativeToken` amounts;

### Removed

- `constant` module;
- `OutputAmount`, `StorageDepositAmount`, `TreasuryOutputAmount` and `MigratedFundsAmount`;
- `target_score` parameters from `BlockBuilder::with_nonce_provider`;

## 1.0.0-beta.7 - 2022-08-30

### Changed

- Updated dependencies (including `packable`);
- Restrict constraint to unlock an Alias address to Alias state transitions;
- Use new packable version with `Packable::UnpackVisitor`;

## 1.0.0-beta.6 - 2022-08-11

### Added

- `NativeTokensBuilder::finish_vec`;

### Changed

- Updated dependencies;

## 1.0.0-beta.5 - 2022-07-27

### Fixed

- `rand` feature;

## 1.0.0-beta.4 - 2022-07-26

### Changed

- Bump `inx` to `v1.0.0-beta.3`;

## 1.0.0-beta.3 - 2022-07-21

### Added

- Added conversions for `inx` types;
- `ProtocolParameters::new` and getters;

## 1.0.0-beta.2 - 2022-07-20

### Added

- `ProtocolParameters` type;

### Changed

- Add "No Native Tokens" rule for storage deposit returns;
- Rename `ByteCost*` to `Rent*`;
- Moved random generation of types from `bee-test` to `rand` module within crate;

### Fixed

- Add expiration check for input storage deposit returns selection;

## 1.0.0-beta.1 - 2022-07-19

Initial implementation of the `Block` related TIPs. -->
