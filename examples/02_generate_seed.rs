// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 02_generate_seed --release

use iota_client::crypto::signatures::ed25519::SecretKey;

/// In this example we will generate a seed

#[tokio::main]
async fn main() {
    let secret_key = SecretKey::generate().unwrap();
    println!("{}", hex::encode(secret_key.to_bytes()));
}
