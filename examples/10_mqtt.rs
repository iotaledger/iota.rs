// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 10_mqtt --features=mqtt --release

use iota_client::{bee_message::Message, Client, MqttEvent, Result, Topic};
use std::sync::{mpsc::channel, Arc, Mutex};

// Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let mut iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

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
        .with_topics(vec![Topic::new("milestones/latest")?, Topic::new("messages")?])
        .subscribe(move |event| {
            match event.topic.as_str() {
                "messages" => {
                    let message: Message = serde_json::from_str(&event.payload).unwrap();
                    println!("{event:?}");
                    println!("{message:?}");
                }
                _ => println!("{event:?}"),
            }
            tx.lock().unwrap().send(()).unwrap();
        })
        .await
        .unwrap();

    for i in 0..10 {
        rx.recv().unwrap();
        if i == 7 {
            // unsubscribe from topic "messages", will continue to receive events for "milestones/latest"
            iota.subscriber()
                .with_topics(vec![Topic::new("messages")?])
                .unsubscribe()
                .await?;
        }
    }

    iota.subscriber().disconnect().await?;
    // alternatively
    // iota.subscriber().unsubscribe().await?;
    Ok(())
}
