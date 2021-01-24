// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example transaction --release
use iota::{Client, MessageId, Seed};
use std::time::Duration;
use tokio::time::sleep;
#[macro_use]
extern crate dotenv_codegen;

/// In this example, we send 900 tokens to the following 3 locations, respectively
///
/// Address Index 0. Note that we can use the `address` example codes to know the addresses belong to the seed.
///   output 0: 300 tokens iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6
///   output 1: 300 tokens iot1qyg7l34etk4sdfrdt46vwt7a964avk9sfrxh8ecq2sgpezaktd55cyc76lc
///   output 2: 300 tokens iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r
///
///
/// These two addresses belong to seed "256a818b2aac458941f7274985a410e57fb750f3a3a67369ece5bd9ae7eef5b0"
/// Then we send 550 tokens from seed "256a818b2aac458941f7274985a410e57fb750f3a3a67369ece5bd9ae7eef5b0"
/// to addresses "iot1q95jpvtk7cf7c7l9ne50c684jl4n8ya0srm5clpak7qes9ratu0l76clafr" and
/// "iot1q9gtmpa58j9vp23hrsztckt5rquy26lrrv25nz4g0v9pr8nsnqetcjskw9m", and check the ledger
/// inclusion state, which should be "included".

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    // Insert your seed in the .env. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed in the .env is iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4
    let seed = Seed::from_ed25519_bytes(&hex::decode(dotenv!("seed")).unwrap()).unwrap();

    let message = iota
        .send()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            &"iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6".into(),
            300,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "First transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;

    let message = iota
        .send()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            &"iot1qyg7l34etk4sdfrdt46vwt7a964avk9sfrxh8ecq2sgpezaktd55cyc76lc".into(),
            300,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Second transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;

    let message = iota
        .send()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            &"iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r".into(),
            300,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();
    println!(
        "Third transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;

    let seed = Seed::from_ed25519_bytes(&hex::decode(dotenv!("second_seed")).unwrap()).unwrap();

    let message = iota
        .send()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        // Note that we can transfer to multiple outputs by using the `SendTransactionBuilder`
        .with_output(
            &"iot1q95jpvtk7cf7c7l9ne50c684jl4n8ya0srm5clpak7qes9ratu0l76clafr".into(),
            270,
        )
        .unwrap()
        .with_output(
            &"iot1q9gtmpa58j9vp23hrsztckt5rquy26lrrv25nz4g0v9pr8nsnqetcjskw9m".into(),
            280,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Last transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;
    let message_metadata = iota.get_message().metadata(&message.id().0).await;
    println!(
        "The ledgerInclusionState: {:?}",
        message_metadata.unwrap().ledger_inclusion_state
    );
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
