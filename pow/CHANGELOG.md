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

## 1.0.0-rc.4 - 2023-03-09

### Changed

- Update dependencies;

## 1.0.0-rc.3 - 2023-02-08

### Changed

- Update dependencies;
- `{Miner, SingleThreadedMiner}::nonce` return an `Option<u64>` instead of a `Result<u64, Error>`;
- `get_miner` and `get_miner_num_workers` return an `Option<u64>` instead of a `Result<u64, Error>`;

### Removed

- `Error` enum;

## 1.0.0-rc.2 - 2022-12-20

### Added

- `SingleThreadedMinerBuilder` and `SingleThreadedMiner` for `target_family = "wasm"`;
- `get_miner` and `get_miner_num_workers` functions;
- `MinerBuilder::new` method;

### Changed

- `MinerBuilder` and `Miner` don't implement `NonceProvider` and `NonceProviderBuilder` traits anymore;
- Renamed `PoWScorer` to `PowScorer`;

### Removed

- `providers` module;
- `NonceProvider` and `NonceProviderBuilder` traits;
- Deprecated `pow_hash` and `compute_pow_score`;

## 1.0.0-rc.1 - 2022-10-25

First release based on `bee-pow`.

<!-- We include the past changelogs of `bee-pow` for reference as they have been merged into the client repository as a new crate.

## 1.0.0 - 2022-09-26

### Changed

- Update dependencies;

## 1.0.0-beta.1 - 2022-08-29

### Changed

- Type of `target_score` parameter of `NonceProvider::nonce` from `f64` to `u32` to better match TIP32;
- Updated dependencies;

### Fixed

- Clippy warning;

## 1.0.0-alpha.1 - 2022-07-15

First alpha release.

## 0.2.0 - 2021-11-19

### Changed

- Scoring of Proof of Work can now reuse hash functions;

## 0.1.0 - 2021-04-13

### Added

- Proof of Work scoring functions;
- NonceProviderBuilder/NonceProvider traits;
- MinerBuilder/Miner nonce provider;
- u64 nonce provider; -->
