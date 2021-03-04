// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example mqtt --release

use iota::{BrokerOptions, Client, Message, Topic};
use std::sync::{mpsc::channel, Arc, Mutex};

// To run this example you'll need to add "mqtt" to the features of the iota-core import in Cargo.toml
// like this: iota-core = { path = "../iota-core", features = ["storage", "mqtt"] }. You'll also need to uncomment the
// serde_json dependency as well as the mqtt example beneath

#[tokio::main]
async fn main() {
    // Create a client instance
    let mut iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert your node URL here
        .unwrap()
        // to use tcp instead
        .with_mqtt_broker_options(BrokerOptions::new().use_websockets(false))
        .finish()
        .await
        .unwrap();

    let (tx, rx) = channel();
    let tx = Arc::new(Mutex::new(tx));

    iota.subscriber()
        .with_topics(vec![
            Topic::new("milestones/latest").unwrap(),
            Topic::new("messages").unwrap(),
        ])
        .subscribe(move |event| {
            match event.topic.as_str() {
                "messages" => {
                    let message: Message = serde_json::from_str(&event.payload).unwrap();
                    println!("{:?}", event);
                    println!("{:?}", message);
                }
                _ => println!("{:?}", event),
            }
            tx.lock().unwrap().send(()).unwrap();
        })
        .await
        .unwrap();

    for _ in 0..10 {
        rx.recv().unwrap();
    }

    iota.subscriber().disconnect().await.unwrap();
    // alternatively
    // iota.subscriber().unsubscribe().unwrap();
}
