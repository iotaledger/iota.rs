use iota::{BIP32Path, Client, Ed25519Address, Seed};
use std::time::Duration;
use std::{convert::TryInto, num::NonZeroU64};
use tokio::time::delay_for;

/// In this example, we send 600 tokens to the following 6 locations, respectively
///
/// Address m/0 (5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8)
///   output 0: 100 tokens
///   output 1: 100 tokens
///   output 2: 100 tokens
///
/// Address m/1 (bcbe5e2ccd4ce942407a0fd8ccad1df33c68c9cb1078c043e95e486d8c6e0230)
///   output 0: 100 tokens
///   output 1: 100 tokens
///   output 2: 100 tokens
///
///
/// These two addresses belong to seed "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b1"
/// Then we send 550 tokens from seed "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b1"
/// to address "6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1fff", and check the ledger
/// inclusion state, which should be "included".

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();

    // Insert your account path. Note that index must be hardened(like 0', 123').
    let path = BIP32Path::from_str("m/").unwrap();
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);
    delay_for(Duration::from_millis(15000)).await;
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("bcbe5e2ccd4ce942407a0fd8ccad1df33c68c9cb1078c043e95e486d8c6e0230") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);

    delay_for(Duration::from_millis(15000)).await;
    // Insert your account path. Note that index must be hardened(like 0', 123').
    let path = BIP32Path::from_str("m/").unwrap();
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);
    delay_for(Duration::from_millis(15000)).await;
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("bcbe5e2ccd4ce942407a0fd8ccad1df33c68c9cb1078c043e95e486d8c6e0230") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);

    delay_for(Duration::from_millis(15000)).await;
    // Insert your account path. Note that index must be hardened(like 0', 123').
    let path = BIP32Path::from_str("m/").unwrap();
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);
    delay_for(Duration::from_millis(15000)).await;
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("bcbe5e2ccd4ce942407a0fd8ccad1df33c68c9cb1078c043e95e486d8c6e0230") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);

    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b1").unwrap(),
    )
    .unwrap(); // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.

    delay_for(Duration::from_millis(15000)).await;
    // Insert your account path. Note that index must be hardened(like 0', 123').
    let path = BIP32Path::from_str("m/").unwrap();
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1fff") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(550).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);
    delay_for(Duration::from_millis(15000)).await;
    let message_metadata = iota.get_message().metadata(&message_id.unwrap()).await;
    println!(
        "The ledgerInclusionState: {:?}",
        message_metadata.unwrap().ledger_inclusion_state
    );
}
