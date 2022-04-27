// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 10_mqtt --features=mqtt --release

use std::sync::{mpsc::channel, Arc, Mutex};

use iota_client::{Client, MqttEvent, MqttPayload, Result, Topic};

// Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let mut client = Client::builder()
        // todo: replace url when we have a public testnet
        .with_node("https://chrysalis-nodes.iota.cafe/")?
        .with_node_sync_disabled()
        // .with_mqtt_broker_options(BrokerOptions::new().use_ws(false))
        .finish()
        .await?;

    let (tx, rx) = channel();
    let tx = Arc::new(Mutex::new(tx));

    let mut event_rx = client.mqtt_event_receiver();
    tokio::spawn(async move {
        while event_rx.changed().await.is_ok() {
            let event = event_rx.borrow();
            if *event == MqttEvent::Disconnected {
                println!("mqtt disconnected");
                std::process::exit(1);
            }
        }
    });

    client
        .subscriber()
        .with_topics(vec![
            Topic::try_from("milestones/latest".to_string())?,
            Topic::try_from("messages".to_string())?,
            Topic::try_from(
                "outputs/unlock/address/atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string(),
            )?,
        ])
        .subscribe(move |event| {
            println!("Topic: {}", event.topic);
            match &event.payload {
                MqttPayload::Json(val) => println!("{}", serde_json::to_string(&val).unwrap()),
                MqttPayload::Message(msg) => println!("{:?}", msg),
                MqttPayload::MilestonePayload(_) => todo!(),
                MqttPayload::Receipt(_) => todo!(),
            }
            tx.lock().unwrap().send(()).unwrap();
        })
        .await
        .unwrap();

    for i in 0..10 {
        rx.recv().unwrap();
        if i == 7 {
            // unsubscribe from topic "messages", will continue to receive events for "milestones/latest"
            client
                .subscriber()
                .with_topics(vec![Topic::try_from("messages".to_string())?])
                .unsubscribe()
                .await?;
        }
    }

    client.subscriber().disconnect().await?;
    // alternatively
    // client.subscriber().unsubscribe().await?;
    Ok(())
}
