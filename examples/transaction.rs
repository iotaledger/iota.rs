// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example transaction --release
use iota::{Client, MessageId, Seed};
use std::{num::NonZeroU64, time::Duration};
use tokio::time::delay_for;
/// In this example, we send 900 tokens to the following 3 locations, respectively
///
/// Address Index 0. Note that we can use the `address` example codes to know the addresses belong to the seed.
///   output 0: 300 tokens iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6
///   output 1: 300 tokens iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r
///   output 2: 300 tokens iot1q84egwx5gu4nme5cn6q3fxwe2j7qex6h66d2g6m5grshaxq07fntxufm9td
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
        .node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed below is iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .output(
            "iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6", // Insert the address to search for
            NonZeroU64::new(300).unwrap(),
        )
        .unwrap()
        .post()
        .await
        .unwrap();

    println!(
        "First transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message_id
    );
    reattach_promote_until_confirmed(message_id, &iota).await;

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .output(
            "iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r",
            NonZeroU64::new(300).unwrap(),
        )
        .unwrap()
        .post()
        .await
        .unwrap();

    println!(
        "Second transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message_id
    );
    reattach_promote_until_confirmed(message_id, &iota).await;

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .output(
            "iot1q84egwx5gu4nme5cn6q3fxwe2j7qex6h66d2g6m5grshaxq07fntxufm9td",
            NonZeroU64::new(300).unwrap(),
        )
        .unwrap()
        .post()
        .await
        .unwrap();
    println!(
        "Third transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message_id
    );
    reattach_promote_until_confirmed(message_id, &iota).await;

    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67369ece5bd9ae7eef5b0").unwrap(),
    )
    .unwrap(); // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        // Note that we can transfer to multiple outputs by using the `SendTransactionBuilder`
        .output(
            "iot1q95jpvtk7cf7c7l9ne50c684jl4n8ya0srm5clpak7qes9ratu0l76clafr",
            NonZeroU64::new(270).unwrap(),
        )
        .unwrap()
        .output(
            "iot1q9gtmpa58j9vp23hrsztckt5rquy26lrrv25nz4g0v9pr8nsnqetcjskw9m",
            NonZeroU64::new(280).unwrap(),
        )
        .unwrap()
        .post()
        .await
        .unwrap();

    println!(
        "Last transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message_id
    );
    reattach_promote_until_confirmed(message_id, &iota).await;
    let message_metadata = iota.get_message().metadata(&message_id).await;
    println!(
        "The ledgerInclusionState: {:?}",
        message_metadata.unwrap().ledger_inclusion_state
    );
}

async fn reattach_promote_until_confirmed(message_id: MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if let Some(state) = metadata.ledger_inclusion_state {
            println!("Leder inclusion state: {}", state);
            break;
        } else {
            if let Ok(msg_id) = iota.reattach(&message_id).await {
                println!("Reattached or promoted {}", msg_id.0);
            }
        }
        delay_for(Duration::from_secs(5)).await;
    }
}
