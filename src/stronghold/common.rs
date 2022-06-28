// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Commonly used constants and utilities.

use zeroize::Zeroizing;

/// Stronghold vault path to secrets.
///
/// The value has been hard-coded historically.
pub(super) const SECRET_VAULT_PATH: &[u8] = b"iota-wallet-secret";

/// Stronghold record path to a seed.
///
/// The value has been hard-coded historically.
pub(super) const SEED_RECORD_PATH: &[u8] = b"iota-wallet-seed";

/// Stronghold record path to a derived SLIP-10 private key.
///
/// The value has been hard-coded historically.
pub(super) const DERIVE_OUTPUT_RECORD_PATH: &[u8] = b"iota-wallet-derived";

/// The client path for the seed.
///
/// The value has been hard-coded historically.
pub(super) const PRIVATE_DATA_CLIENT_PATH: &[u8] = b"iota_seed";

const PBKDF_SALT: &[u8] = b"wallet.rs";
const PBKDF_ITER: usize = 100;

pub(crate) type EncryptionKey = Zeroizing<Vec<u8>>;

/// Hash a password, deriving a key, for accessing Stronghold.
pub(super) fn derive_key_from_password(password: &str) -> EncryptionKey {
    let mut buffer = Zeroizing::new([0u8; 64]);

    // Safe to unwrap because rounds > 0.
    crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(password.as_bytes(), PBKDF_SALT, PBKDF_ITER, buffer.as_mut()).unwrap();

    Zeroizing::new(buffer[..32].to_vec())
}
