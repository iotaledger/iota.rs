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

## 1.0.0-rc.3 - 2023-MM-DD

### Added

- `OutputIdsResponse`;
- `Client::hashTransactionEssence()`;

### Changed

- Changes from the Rust library;
- Merged `IAuth::{username, password}` into `IAuth::basicAuthNamePwd`;
- `Burn` fields are now optional;
- `Burn::nativeTokens` is now an array;
- `Client::{aliasOutputIds, basicOutputIds, foundryOutputIds, nftOutputIds}` will not do automatic pagination if `QueryParameter::Cursor(_)` is provided and return type from `string[]` to `OutputIdsResponse`;

### Removed

- `IInputSigningData::bech32Address`;

## 1.0.0-alpha.2 - 2023-02-08

### Added

- `aliasIdToBech32()`;
- `nftIdToBech32()`;
- `computeAliasId()`;
- `computeNftId()`;
- `computeFoundryId()`;

### Changed

- Updated dependencies;
- Renamed `IInputSigningData::outputMetaData` to `IInputSigningData::outputMetadata`;
- Changed `ISegment::bs` from `Uint8Array` to `number[]` so that the serialization corresponds to what is expected;

### Fixed

- Returned JSON value for `IInputSigningData`;

## 1.0.0-alpha.1 - YYYY-MM-DD

Initial release of the wasm bindings.