// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 04_get_balance --release

use iota_client::{signing::mnemonic::MnemonicSigner, Client, Result};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the account balance of a known seed and the balance and outputs of a known address

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;

    let seed_balance = iota.get_balance(&signer).finish().await?;
    println!("Account balance: {:?}i\n", seed_balance);

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let outputs = iota.get_address().outputs_response(address, Default::default()).await?;

    println!("The outputs of address {:?} are: {:?}", address, outputs);
    Ok(())
}
