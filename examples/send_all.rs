// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_all --release

use iota_client::{signing::mnemonic::MnemonicSigner, Client, Result};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the balance of the first account of the seed and send everything
/// Run the consolidation example first if there are more than 127 outputs

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();

    let seed = MnemonicSigner::new_from_seed(&env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?;
    let seed_2 = MnemonicSigner::new_from_seed(&env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?;

    let total_balance = iota.get_balance(&seed).with_initial_address_index(0).finish().await?;

    println!("Total balance: {}", total_balance);

    let message = iota
        .message()
        .with_signer(&seed)
        .with_output(
            &iota.get_addresses(&seed_2).with_range(0..1).finish().await?[0],
            total_balance,
        )?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );

    let _ = iota.retry_until_included(&message.id(), None, None).await.unwrap();
    Ok(())
}
