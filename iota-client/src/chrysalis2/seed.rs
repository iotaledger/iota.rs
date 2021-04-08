// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The seed module
use crate::error::Result;
use bee_common_derive::{SecretDebug, SecretDisplay};
use crypto::signatures::ed25519::{SecretKey, SECRET_KEY_LENGTH};
use slip10::{derive_key_from_path, BIP32Path, Curve};
use std::convert::TryInto;

/// Binary `Ed25519`-based `Seed` to derive private keys, public keys and signatures from.
#[derive(SecretDebug, SecretDisplay)]
pub struct Seed(pub SecretKey);

impl Seed {
    /// Creates a new random `Seed`.
    pub fn generate() -> Result<Self> {
        Ok(Self(SecretKey::generate()?))
    }

    /// Generate private key with a BIP32Path
    pub fn generate_private_key(&self, path: &BIP32Path) -> Result<SecretKey> {
        let subseed = derive_key_from_path(&self.to_le_bytes(), Curve::Ed25519, path)?.key;

        Ok(SecretKey::from_le_bytes(subseed)?)
    }

    /// Convert this seed to a byte array.
    pub fn to_le_bytes(&self) -> [u8; SECRET_KEY_LENGTH] {
        self.0.to_le_bytes()
    }

    /// Convert this seed to a byte array.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self(SecretKey::from_le_bytes(
            bytes.try_into().expect("Invalid byte length"),
        )?))
    }
}
