// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 00_mnemonic --release

use iota_client::{
    api::GetAddressesBuilder,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

/// In this example we will generate a mnemonic and generate the first address with the Shimmer coin type, following
/// BIP-0044

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = Client::generate_mnemonic()?;
    println!("Mnemonic: {}", mnemonic);

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&mnemonic)?);

    // Generate addresses with custom account index and range
    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await?;

    println!("First public address: {}", addresses[0]);
    Ok(())
}
