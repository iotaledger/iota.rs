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
