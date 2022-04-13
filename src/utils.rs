// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Utility functions for IOTA

use std::collections::HashMap;

use bee_message::address::{Address, Ed25519Address};
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::{bip39::wordlist, slip10::Seed},
    utils,
};
use fern_logger::{logger_init, LoggerConfig, LoggerOutputConfigBuilder};
use log::LevelFilter;
use zeroize::Zeroize;

use crate::error::*;

/// Hash the network id str from the nodeinfo to an u64 (used in messages)
pub fn hash_network(network_id_string: &str) -> Result<u64> {
    let bytes = Blake2b256::digest(network_id_string.as_bytes())[0..8]
        .try_into()
        .map_err(|_e| Error::Blake2b256Error("Hashing the network id failed."))?;

    Ok(u64::from_le_bytes(bytes))
}

/// Transforms bech32 to hex
pub fn bech32_to_hex(bech32: &str) -> crate::Result<String> {
    let (_bech32_hrp, address) = Address::try_from_bech32(bech32)?;
    let hex_string = match address {
        Address::Ed25519(ed) => ed.to_string(),
        Address::Alias(alias) => alias.to_string(),
        Address::Nft(nft) => nft.to_string(),
    };
    Ok(hex_string)
}

/// Transforms a hex encoded address to a bech32 encoded address
pub fn hex_to_bech32(hex: &str, bech32_hrp: &str) -> crate::Result<String> {
    let address: Ed25519Address = hex.parse::<Ed25519Address>()?;
    Ok(Address::Ed25519(address).to_bech32(bech32_hrp))
}

/// Transforms a hex encoded public key to a bech32 encoded address
pub fn hex_public_key_to_bech32_address(hex: &str, bech32_hrp: &str) -> crate::Result<String> {
    let mut public_key = [0u8; Ed25519Address::LENGTH];
    hex::decode_to_slice(&hex, &mut public_key)?;

    let address = Blake2b256::digest(&public_key)
        .try_into()
        .map_err(|_e| Error::Blake2b256Error("Hashing the public key failed."))?;
    let address: Ed25519Address = Ed25519Address::new(address);
    Ok(Address::Ed25519(address).to_bech32(bech32_hrp))
}

/// Returns a valid Address parsed from a String.
pub fn parse_bech32_address(address: &str) -> crate::Result<Address> {
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
        .map_err(|e| crate::Error::InvalidMnemonic(format!("{:?}", e)))?;
    entropy.zeroize();
    Ok(mnemonic)
}

/// Returns a hex encoded seed for a mnemonic.
pub fn mnemonic_to_hex_seed(mnemonic: &str) -> Result<String> {
    // trim because empty spaces could create a different seed https://github.com/iotaledger/crypto.rs/issues/125
    let mnemonic = mnemonic.trim();
    // first we check if the mnemonic is valid to give meaningful errors
    crypto::keys::bip39::wordlist::verify(mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| crate::Error::InvalidMnemonic(format!("{:?}", e)))?;
    let mut mnemonic_seed = [0u8; 64];
    crypto::keys::bip39::mnemonic_to_seed(mnemonic, "", &mut mnemonic_seed);
    Ok(hex::encode(mnemonic_seed))
}

/// Returns a seed for a mnemonic.
pub fn mnemonic_to_seed(mnemonic: &str) -> Result<Seed> {
    // trim because empty spaces could create a different seed https://github.com/iotaledger/crypto.rs/issues/125
    let mnemonic = mnemonic.trim();
    // first we check if the mnemonic is valid to give meaningful errors
    crypto::keys::bip39::wordlist::verify(mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
        .map_err(|e| crate::Error::InvalidMnemonic(format!("{:?}", e)))?;
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

/// creates a file in which logs will be written in
pub fn init_logger(filename: &str, levelfilter: LevelFilter) -> crate::Result<()> {
    let output_config = LoggerOutputConfigBuilder::new()
        .name(filename)
        .level_filter(levelfilter);
    let config = LoggerConfig::build().with_output(output_config).finish();
    logger_init(config)?;
    Ok(())
}
