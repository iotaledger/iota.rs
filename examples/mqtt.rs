// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example mqtt --release
use iota::{BrokerOptions, Client, Topic};
use std::sync::{mpsc::channel, Arc, Mutex};

fn main() {
    let mut iota = Client::build() // Crate a client instance builder
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        // to use tcp instead
        .with_mqtt_broker_options(BrokerOptions::new().use_websockets(false))
        .finish()
        .unwrap();

    let (tx, rx) = channel();
    let tx = Arc::new(Mutex::new(tx));

    iota.subscriber()
        .with_topics(vec![
            Topic::new("milestones/latest").unwrap(),
            Topic::new("messages").unwrap(),
        ])
        .subscribe(move |event| {
            println!("{:?}", event);
            tx.lock().unwrap().send(()).unwrap();
        })
        .unwrap();

    rx.recv().unwrap();
    iota.subscriber().disconnect().unwrap();
    // alternatively
    // iota.subscriber().unsubscribe().unwrap();
}
