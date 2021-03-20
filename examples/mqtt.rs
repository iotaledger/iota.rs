// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example mqtt --release

use iota::{Client, Message, MqttEvent, Topic};
use std::sync::{mpsc::channel, Arc, Mutex};

// Connecting to a MQTT broker using raw ip doesn't work. This is a limitation of rustls.
#[tokio::main]
async fn main() {
    // Create a client instance
    let mut iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let (tx, rx) = channel();
    let tx = Arc::new(Mutex::new(tx));

    let mut event_rx = iota.mqtt_event_receiver();
    tokio::spawn(async move {
        while event_rx.changed().await.is_ok() {
            let event = event_rx.borrow();
            if *event == MqttEvent::Disconnected {
                println!("mqtt disconnected");
                std::process::exit(1);
            }
        }
    });

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
