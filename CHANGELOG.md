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
