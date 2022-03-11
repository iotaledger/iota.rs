// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example stronghold --features=stronghold --release

use iota_client::{signing::stronghold::StrongholdSigner, Client, Result};
extern crate dotenv;
use dotenv::dotenv;
use std::{env, path::Path};

/// In this example we will create addresses with a stronghold signer

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    let storage_path = Path::new("test.stronghold");
    let stronghold_signer =
        StrongholdSigner::try_new_signer_handle("some_hopefully_secure_password", storage_path).unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let mnemonic = env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap();
    // The mnemonic only needs to be stored the first time
    stronghold_signer
        .lock()
        .await
        .store_mnemonic(storage_path, mnemonic)
        .await
        .unwrap();

    // Generate addresses with custom account index and range
    let addresses = iota
        .get_addresses(&stronghold_signer)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await?;

    println!("List of generated public addresses:\n{:?}\n", addresses);

    Ok(())
}
