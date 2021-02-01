// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The secrets module for the seed
use crate::error::Result;
use bee_common_derive::{SecretDebug, SecretDisplay};
use crypto::ed25519::{PublicKey, SecretKey, Signature, SECRET_KEY_LENGTH};
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

/// Ed25519 private key.
#[derive(SecretDebug, SecretDisplay)]
pub struct Ed25519PrivateKey(SecretKey);

impl Ed25519PrivateKey {
    /// Deterministically generates and returns a private key from a seed and an index.
    ///
    /// # Arguments
    ///
    /// * `seed`    A seed to deterministically derive a private key from.
    pub fn generate_from_seed(seed: &Seed, path: &BIP32Path) -> Result<Self> {
        let subseed = derive_key_from_path(&seed.to_le_bytes(), Curve::Ed25519, path)?.key;

        Ok(Self(SecretKey::from_le_bytes(subseed)?))
    }

    /// Returns the public counterpart of a private key.
    pub fn generate_public_key(&self) -> PublicKey {
        self.0.public_key()
    }

    /// Convert this private key to a byte array.
    pub fn to_le_bytes(&self) -> [u8; SECRET_KEY_LENGTH] {
        self.0.to_le_bytes()
    }

    /// Convert this private key to a byte array.
    pub fn from_le_bytes(bytes: [u8; 32]) -> Result<Self> {
        Ok(Self(SecretKey::from_le_bytes(bytes)?))
    }

    /// Sign
    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.0.sign(msg)
    }
}
