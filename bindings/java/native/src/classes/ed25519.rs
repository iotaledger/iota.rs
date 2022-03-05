// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::crypto::signatures::ed25519::{
    PublicKey as RustPublicKey, SecretKey as RustSecretKey, Signature as RustSignature,
};
use std::fmt::{Display, Formatter};

use crate::Result;
use anyhow::anyhow;

const SECRET_KEY_LENGTH: usize = 32;
const SIGNATURE_LENGTH: usize = 64;

pub struct SecretKey(RustSecretKey);

impl SecretKey {
    pub fn generate() -> Result<Self> {
        match RustSecretKey::generate() {
            Ok(s) => Ok(Self(s)),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn from_bytes(bs: Vec<u8>) -> Self {
        let mut bs_arr: [u8; SECRET_KEY_LENGTH] = [0; SECRET_KEY_LENGTH];
        bs_arr.copy_from_slice(&bs[0..SECRET_KEY_LENGTH]);
        Self(RustSecretKey::from_bytes(bs_arr))
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn sign(&self, msg: Vec<u8>) -> Signature {
        Signature(self.0.sign(&msg))
    }
}

impl Display for SecretKey {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()))
    }
}

impl From<RustSecretKey> for SecretKey {
    fn from(key: RustSecretKey) -> Self {
        Self(key)
    }
}

pub struct PublicKey(RustPublicKey);

impl PublicKey {
    pub fn verify(&self, sig: Signature, msg: Vec<u8>) -> bool {
        self.0.verify(&sig.0, &msg)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn try_from_bytes(bs: Vec<u8>) -> Result<Self> {
        let mut bs_arr: [u8; SECRET_KEY_LENGTH] = [0; SECRET_KEY_LENGTH];
        bs_arr.copy_from_slice(&bs[0..SECRET_KEY_LENGTH]);
        match RustPublicKey::try_from_bytes(bs_arr) {
            Ok(bytes) => Ok(Self(bytes)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
}
impl core::convert::TryFrom<&[u8; 32]> for PublicKey {
    type Error = anyhow::Error;
    fn try_from(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        match RustPublicKey::try_from_bytes(*bytes) {
            Ok(k) => Ok(Self(k)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()))
    }
}

impl From<RustPublicKey> for PublicKey {
    fn from(output: RustPublicKey) -> Self {
        Self(output)
    }
}

pub struct Signature(RustSignature);

impl Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn from_bytes(bs: Vec<u8>) -> Self {
        let mut bs_arr: [u8; SIGNATURE_LENGTH] = [0; SIGNATURE_LENGTH];
        bs_arr.copy_from_slice(&bs[0..SIGNATURE_LENGTH]);
        Self(RustSignature::from_bytes(bs_arr))
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.to_bytes()))
    }
}

impl From<RustSignature> for Signature {
    fn from(output: RustSignature) -> Self {
        Self(output)
    }
}
