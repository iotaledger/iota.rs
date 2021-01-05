// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{Client, Topic};
use std::sync::{mpsc::channel, Arc, Mutex};

fn main() {
    let mut iota = Client::build() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    let (tx, rx) = channel();
    let tx = Arc::new(Mutex::new(tx));

    iota.subscriber()
        .topic(Topic::new("milestones/latest").unwrap())
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
