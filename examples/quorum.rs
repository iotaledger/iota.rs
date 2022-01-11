// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example quorum --release

use iota_client::{
    signing::{mnemonic::MnemonicSigner, SignerHandle},
    Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the account balance of a known seed with quorum, which will compare the responses from
/// the nodes

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe/")?
        .with_node("http://localhost:14265")?
        .with_node("https://api.thin-hornet-1.h.chrysalis-devnet.iota.cafe/")?
        .with_quorum(true)
        .with_quorum_size(3)
        .with_quorum_threshold(66)
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let mnemonic_signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;
    let signer = SignerHandle::new(Box::new(mnemonic_signer));

    let seed_balance = iota.get_balance(&signer).finish().await.unwrap();
    println!("Account balance: {:?}i\n", seed_balance);

    Ok(())
}
