// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example mnemonic --release

use iota_client::{crypto::keys::bip39::mnemonic_to_seed, Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
// use crypto;

/// In this example we will create addresses from a mnemonic defined in .env

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // Generate a random mnemonic
    let mnemonic = Client::generate_mnemonic()?;
    println!("Generated mnemonic: {mnemonic:?}");

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let mut mnemonic_seed = [0u8; 64];
    mnemonic_to_seed(
        &env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC").unwrap(),
        "",
        &mut mnemonic_seed,
    );

    let seed = Seed::from_bytes(&mnemonic_seed);

    // Generate addresses with custom account index and range
    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await
        .unwrap();

    println!("List of generated public addresses:\n{addresses:?}\n");
    Ok(())
}
