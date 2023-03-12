// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example generate_addresses --release -- [NODE URL]

use iota_client::{api::GetAddressesBuilder, secret::SecretManager, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv::dotenv().ok();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| std::env::var("NODE_URL").unwrap());

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Generate addresses with default account index and range
    let addresses = client.get_addresses(&secret_manager).finish().await?;

    println!("List of generated public addresses:\n{addresses:#?}\n");

    // Generate addresses with custom account index and range
    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await?;

    println!("List of generated public addresses:\n{addresses:#?}\n");

    // Generate internal addresses with custom account index and range
    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(0..4)
        .with_internal_addresses(true)
        .finish()
        .await?;

    println!("List of generated internal addresses:\n{addresses:#?}\n");

    // Generating addresses with `client.get_addresses(&secret_manager)`, will by default get the bech32_hrp (Bech32
    // human readable part) from the node info, generating it "offline" requires setting it with
    // `with_bech32_hrp(bech32_hrp)`
    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_bech32_hrp(client.get_bech32_hrp().await?)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await?;

    println!("List of offline generated public addresses:\n{addresses:#?}\n");

    Ok(())
}
