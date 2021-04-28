// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 08_data_message --release

use iota::{Client, Result};

/// In this example we will send a message without a payload

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    let message = iota
        .message()
        .with_index("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Message sent https://explorer.iota.org/testnet/message/{}\n",
        message.id().0
    );

    let fetched_message_ids = iota.get_message().index("Hello").await.unwrap();
    println!("Messages with Hello index: {:?}", fetched_message_ids);
    Ok(())
}
