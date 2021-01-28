// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example storage --release
use iota::Client;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .with_storage("my_messages".into(), PathBuf::from("testpath.db"))
        .await
        .unwrap()
        .finish()
        .unwrap();

    let r = iota
        .send()
        .with_index("Helloo")
        .with_data("Tangle".to_string().as_bytes().to_vec())
        .finish()
        .await
        .unwrap();

    println!("MessageId {}", r.id().0);
    println!(
        "Message from storage {:?}",
        iota.storage.clone().unwrap().list_messages(1, 0).await
    );
}
