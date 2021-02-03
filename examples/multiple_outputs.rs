// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example multiple_outputs --release
use iota::{client::Result, Client, MessageId, Seed};
use std::{convert::TryInto, time::Duration};
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example, we send 8_800_000 tokens to the following 3 locations, respectively
///
/// Address Index (1..4)
///   output 0: 3_000_000 tokens atoi1q9nrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakc0ht47
///   output 1: 2_800_000 tokens atoi1qx4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337pw23ua
///   output 2: 3_000_000 tokens atoi1qxumqjtucwglfja746vvmr7n54ep88kcu2qvaquqrnx9qs2z8f4t6hpwr8h
///
///
/// These three addresses belong to first seed in .env.example

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder() // Crate a client instance builder
        // .with_node("http://0.0.0.0:14265") // Insert the node here
        // .unwrap()
        .finish()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed below is atoi1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxtmtev5
    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_ed25519_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap())
            .unwrap();

    let message = iota
        .send()
        .with_seed(&seed)
        .with_output(
            "iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6".try_into()?,
            3_000_000,
        )
        .unwrap()
        .with_output(
            "iot1qyg7l34etk4sdfrdt46vwt7a964avk9sfrxh8ecq2sgpezaktd55cyc76lc".try_into()?,
            2_800_000,
        )
        .unwrap()
        .with_output(
            "iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r".try_into()?,
            3_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

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
