// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Utility functions for IOTA

use std::collections::HashMap;

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{bip39::wordlist, slip10::Seed},
    utils,
};
use iota_types::block::{
    address::{Address, AliasAddress, Ed25519Address, NftAddress},
    output::{AliasId, NftId},
    payload::TaggedDataPayload,
};
use zeroize::Zeroize;

use super::Client;
use crate::error::{Error, Result};

/// Transforms bech32 to hex
pub fn bech32_to_hex(bech32: &str) -> Result<String> {
    let (_bech32_hrp, address) = Address::try_from_bech32(bech32)?;
    let hex_string = match address {
        Address::Ed25519(ed) => ed.to_string(),
        Address::Alias(alias) => alias.to_string(),
        Address::Nft(nft) => nft.to_string(),
    };
    Ok(hex_string)
}

/// Transforms a hex encoded address to a bech32 encoded address
pub fn hex_to_bech32(hex: &str, bech32_hrp: &str) -> Result<String> {
    let address: Ed25519Address = hex.parse::<Ed25519Address>()?;
    Ok(Address::Ed25519(address).to_bech32(bech32_hrp))
}

/// Transforms an alias id to a bech32 encoded address
pub fn alias_id_to_bech32(alias_id: AliasId, bech32_hrp: &str) -> String {
    Address::Alias(AliasAddress::new(alias_id)).to_bech32(bech32_hrp)
}

/// Transforms an nft id to a bech32 encoded address
pub fn nft_id_to_bech32(nft_id: NftId, bech32_hrp: &str) -> String {
    Address::Nft(NftAddress::new(nft_id)).to_bech32(bech32_hrp)
}

/// Transforms a prefix hex encoded public key to a bech32 encoded address
pub fn hex_public_key_to_bech32_address(hex: &str, bech32_hrp: &str) -> Result<String> {
    let public_key: [u8; Ed25519Address::LENGTH] = prefix_hex::decode(hex)?;

    let address = Blake2b256::digest(public_key)
        .try_into()
        .map_err(|_e| Error::Blake2b256("hashing the public key failed."))?;
    let address: Ed25519Address = Ed25519Address::new(address);
    Ok(Address::Ed25519(address).to_bech32(bech32_hrp))
}

/// Returns a valid Address parsed from a String.
pub fn parse_bech32_address(address: &str) -> Result<Address> {
    Ok(Address::try_from_bech32(address)?.1)
}

/// Checks if a String is a valid bech32 encoded address.
pub fn is_address_valid(address: &str) -> bool {
    Address::try_from_bech32(address).is_ok()
}

/// Generates a new mnemonic.
pub fn generate_mnemonic() -> Result<String> {
    let mut entropy = [0u8; 32];
    utils::rand::fill(&mut entropy)?;
    let mnemonic = wordlist::encode(&entropy, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| crate::Error::InvalidMnemonic(format!("{e:?}")))?;
    entropy.zeroize();
    Ok(mnemonic)
}

/// Returns a hex encoded seed for a mnemonic.
pub fn mnemonic_to_hex_seed(mnemonic: &str) -> Result<String> {
    // trim because empty spaces could create a different seed https://github.com/iotaledger/crypto.rs/issues/125
    let mnemonic = mnemonic.trim();
    // first we check if the mnemonic is valid to give meaningful errors
    crypto::keys::bip39::wordlist::verify(mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| crate::Error::InvalidMnemonic(format!("{e:?}")))?;
    let mut mnemonic_seed = [0u8; 64];
    crypto::keys::bip39::mnemonic_to_seed(mnemonic, "", &mut mnemonic_seed);
    Ok(prefix_hex::encode(mnemonic_seed))
}

/// Returns a seed for a mnemonic.
pub fn mnemonic_to_seed(mnemonic: &str) -> Result<Seed> {
    // trim because empty spaces could create a different seed https://github.com/iotaledger/crypto.rs/issues/125
    let mnemonic = mnemonic.trim();
    // first we check if the mnemonic is valid to give meaningful errors
    crypto::keys::bip39::wordlist::verify(mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| crate::Error::InvalidMnemonic(format!("{e:?}")))?;
    let mut mnemonic_seed = [0u8; 64];
    crypto::keys::bip39::mnemonic_to_seed(mnemonic, "", &mut mnemonic_seed);
    Ok(Seed::from_bytes(&mnemonic_seed))
}

/// Requests funds from a faucet
pub async fn request_funds_from_faucet(url: &str, bech32_address: &str) -> Result<String> {
    let mut map = HashMap::new();
    map.insert("address", bech32_address);

    let client = reqwest::Client::new();
    let faucet_response = client.post(url).json(&map).send().await?.text().await?;
    Ok(faucet_response)
}

impl Client {
    /// Transforms bech32 to hex
    pub fn bech32_to_hex(bech32: &str) -> crate::Result<String> {
        bech32_to_hex(bech32)
    }

    /// Transforms a hex encoded address to a bech32 encoded address
    pub async fn hex_to_bech32(&self, hex: &str, bech32_hrp: Option<&str>) -> crate::Result<String> {
        match bech32_hrp {
            Some(hrp) => Ok(hex_to_bech32(hex, hrp)?),
            None => Ok(hex_to_bech32(hex, &self.get_bech32_hrp().await?)?),
        }
    }

    /// Transforms an alias id to a bech32 encoded address
    pub async fn alias_id_to_bech32(&self, alias_id: AliasId, bech32_hrp: Option<&str>) -> crate::Result<String> {
        match bech32_hrp {
            Some(hrp) => Ok(alias_id_to_bech32(alias_id, hrp)),
            None => Ok(alias_id_to_bech32(alias_id, &self.get_bech32_hrp().await?)),
        }
    }

    /// Transforms an nft id to a bech32 encoded address
    pub async fn nft_id_to_bech32(&self, nft_id: NftId, bech32_hrp: Option<&str>) -> crate::Result<String> {
        match bech32_hrp {
            Some(hrp) => Ok(nft_id_to_bech32(nft_id, hrp)),
            None => Ok(nft_id_to_bech32(nft_id, &self.get_bech32_hrp().await?)),
        }
    }

    /// Transforms a hex encoded public key to a bech32 encoded address
    pub async fn hex_public_key_to_bech32_address(&self, hex: &str, bech32_hrp: Option<&str>) -> crate::Result<String> {
        match bech32_hrp {
            Some(hrp) => Ok(hex_public_key_to_bech32_address(hex, hrp)?),
            None => Ok(hex_public_key_to_bech32_address(hex, &self.get_bech32_hrp().await?)?),
        }
    }

    /// Returns a valid Address parsed from a String.
    pub fn parse_bech32_address(address: &str) -> crate::Result<Address> {
        parse_bech32_address(address)
    }

    /// Checks if a String is a valid bech32 encoded address.
    #[must_use]
    pub fn is_address_valid(address: &str) -> bool {
        is_address_valid(address)
    }

    /// Generates a new mnemonic.
    pub fn generate_mnemonic() -> Result<String> {
        generate_mnemonic()
    }

    /// Returns a seed for a mnemonic.
    pub fn mnemonic_to_seed(mnemonic: &str) -> Result<Seed> {
        mnemonic_to_seed(mnemonic)
    }

    /// Returns a hex encoded seed for a mnemonic.
    pub fn mnemonic_to_hex_seed(mnemonic: &str) -> Result<String> {
        mnemonic_to_hex_seed(mnemonic)
    }

    /// UTF-8 encodes the `tag` of a given TaggedDataPayload.
    pub fn tag_to_utf8(payload: &TaggedDataPayload) -> Result<String> {
        String::from_utf8(payload.tag().to_vec()).map_err(|_| Error::TaggedData("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes the `data` of a given TaggedDataPayload.
    pub fn data_to_utf8(payload: &TaggedDataPayload) -> Result<String> {
        String::from_utf8(payload.data().to_vec()).map_err(|_| Error::TaggedData("found invalid UTF-8".to_string()))
    }

    /// UTF-8 encodes both the `tag` and `data` of a given TaggedDataPayload.
    pub fn tagged_data_to_utf8(payload: &TaggedDataPayload) -> Result<(String, String)> {
        Ok((Self::tag_to_utf8(payload)?, Self::data_to_utf8(payload)?))
    }
}
