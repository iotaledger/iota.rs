// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example ledger_nano --features=ledger_nano --release
//! In this example we will create addresses with a ledger nano hardware wallet
//! To use the ledger nano simulator clone https://github.com/iotaledger/ledger-shimmer-app, run `git submodule init && git submodule update --recursive`,
//! then `./build.sh -m nanos|nanox|nanosplus -s` and use `true` in `LedgerSecretManager::new(true)`.

use iota_client::{
    constants::{SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{ledger_nano::LedgerSecretManager, SecretManager},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    let ledger_nano = LedgerSecretManager::new(false);

    println!("{:?}", ledger_nano.get_ledger_nano_status().await);

    let secret_manager = SecretManager::LedgerNano(ledger_nano);

    // Generate addresses with custom account index and range
    let addresses = client
        .get_addresses(&secret_manager)
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await?;

    println!("List of generated public addresses:\n{addresses:?}\n");

    Ok(())
}
