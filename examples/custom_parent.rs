// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_parent --release

use std::{env, str::FromStr};

use dotenv::dotenv;
use iota_client::{bee_block::BlockId, Client};

/// In this example we will define a custom block parent which be used for promoting

#[tokio::main]
async fn main() {
    dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url) // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let custom_parent = BlockId::from_str("b5634e05a7c665d7f87330a53633f001a5d1d96b346dc98dc225c4d6c204f23b").unwrap();

    let block = client
        .block()
        .with_parents(vec![custom_parent])
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Empty block sent: https://explorer.iota.org/devnet/block/{}",
        block.id()
    );
}
