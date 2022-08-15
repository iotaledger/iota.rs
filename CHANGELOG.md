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

## Unreleased - YYYY-MM-DD

### Changed

- Use NativeTokensBuilder::finish_vec to lift some unnecessary 64 Native Tokens limits;
- Use `chacha::{aead_encrypt, aead_decrypt}` from `crypto.rs` in stronghold's `db` module;

### Removed

- Stronghold's `encryption` module;

## 2.0.0-beta.1 - 2022-08-11

First beta release.