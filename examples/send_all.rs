// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_all --release
use iota::{client::Result, Client, MessageId, Seed};
use std::time::Duration;
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example, we get the balance of the first account of the seed and send everything
// Todo: automatically detect amount of inputs and if > 127 create multiple transactions

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    // Insert your seed in the .env. Since the output amount cannot be zero. The seed must contain non-zero balance.
    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed_1 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?);

    let total_balance = iota.get_balance(&seed_1).with_initial_address_index(0).finish().await?;
    println!("total_balance {}", total_balance);
    let message = iota
        .message()
        .with_seed(&seed_1)
        .with_output(
            &"atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".into(),
            total_balance,
        )?
        .with_initial_address_index(0)
        .finish()
        .await?;
    println!(
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;
    Ok(())
}

async fn reattach_promote_until_confirmed(message_id: MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if let Some(state) = metadata.ledger_inclusion_state {
            println!("Leder inclusion state: {:?}", state);
            break;
        } else if let Ok(msg_id) = iota.reattach(&message_id).await {
            println!("Reattached or promoted {}", msg_id.0);
        }
        sleep(Duration::from_secs(5)).await;
    }
}
