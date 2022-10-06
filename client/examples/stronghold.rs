// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example stronghold --features=stronghold --release
//! In this example we will create an address with a stronghold secret manager.

use iota_client::{
    api::GetAddressesBuilder,
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{stronghold::StrongholdSecretManager, SecretManager},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut stronghold_secret_manager = StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password")
        .build("test.stronghold")?;

    // This example uses dotenv, which is not safe for use in production
    dotenv::dotenv().ok();
    let mnemonic = std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap();
    // The mnemonic only needs to be stored the first time
    stronghold_secret_manager.store_mnemonic(mnemonic).await?;

    // Generate addresses with custom account index and range
    let addresses = GetAddressesBuilder::new(&SecretManager::Stronghold(stronghold_secret_manager))
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await?;

    println!("First public address: {}", addresses[0]);

    Ok(())
}
