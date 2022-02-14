// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example search_address --release

use iota_client::{api::search_address, constants::IOTA_COIN_TYPE, signing::mnemonic::MnemonicSigner, Client, Result};
extern crate dotenv;
use dotenv::dotenv;
use std::{convert::TryInto, env};

/// In this example we will try to find the index and address type of an address

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;

    let addresses = iota
        .get_addresses(&signer)
        .with_account_index(0)
        .with_range(9..10)
        .finish()
        .await
        .unwrap();

    println!("{:?}", addresses[0]);

    let res = search_address(
        &signer,
        &iota.get_bech32_hrp().await.unwrap(),
        IOTA_COIN_TYPE,
        0,
        0..10,
        &addresses[0].clone().try_into().unwrap(),
    )
    .await
    .unwrap();

    println!("Address index: {}\nIs internal address: {}", res.0, res.1);
    Ok(())
}
