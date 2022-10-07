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

## 1.0.0-rc.1 - 2022-XX-XX

First release based on `bee-api-types`, `bee-block` and `bee-ternary`.

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

Initial implementation of the `Block` related TIPs.

# bee-ternary

## 1.0.0 - 2022-09-26

### Changed

- Updated dependencies;

## 1.0.0-alpha.1 - 2022-07-15

First alpha release.

## 0.6.0 - 2022-04-12

### Changed

- Renamed feature `serde1` to `serde`;

## 0.5.2 - 2021-11-19

### Changed

- The crate has been made `no_std`;

## 0.5.1 - 2021-11-16

### Added

- `.capacity()` for `TritBuf`;
- Expose `TRITS_PER_BYTE` for `RawEncoding`s;

### Changed

- Preallocate `TritBuf` in `b1t6::encode` for better performance;

## 0.5.0 - 2021-09-27

### Added

- `{RawEncodingBuf, TritBuf, T1B1Buf, T2B1Buf, T3B1Buf, T4B1Buf, T5B1Buf}::clear`;

## 0.4.2-alpha - 2021-03-30

### Added

- `PartialOrd` and `Eq` implementations for `TritBuf`;
- `Eq` implementation for `Trits`;

## 0.4.1-alpha - 2021-03-15

### Fixed

- B1T6 decoding;

## 0.4.0-alpha - 2021-01-18

### Added

- B1T6 bytes-as-trits encoding and decoding support;

## 0.3.4-alpha - 2020-11-13

### Added

- Added proper `i128`/`u128` support detection;

## 0.3.3-alpha - 2020-11-06

### Fixed

- `TryFrom<Trits>` implemented for `u128` and `i128` only when `cfg(has_i128)`;

## 0.3.2-alpha - 2020-10-19

### Added

- `with_capacity` constructor for the buffers of every trit encoding;

## 0.3.1-alpha - 2020-07-23

### Added

- Conversions between `&[Trit]` and `&Trits<T1B1<T>>`;

### Removed

- A useless conversion to same type;

## 0.3.0-alpha - 2020-07-20

### Added

- Support for arbitrary trit to numeric type conversion;

## 0.2.0-alpha - 2020-07-17

### Added

- Binary/ternary numeric conversion;
- FromStr implementation for TryteBuf;
- TritBuf::from_i8s and TritBuf::from_u8s;

## 0.1.0-alpha - 2020-06-12

### Added

- Efficient manipulation of ternary buffers (trits and trytes);
- Multiple encoding schemes;
- Extensible design that allows it to sit on top of existing data structures, avoiding unnecessary allocation and copying;
- An array of utility functions to allow for easy manipulation of ternary data;
- Zero-cost conversion between trit and tryte formats (i.e: no slower than the equivalent code would be if hand-written);

-->
