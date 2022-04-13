// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example tagged_data_to_utf8 --release
//!
use bee_message::payload::TaggedDataPayload;
use iota_client::{Client, Result};

/// In this example we will UTF-8 encode the tag and the data of an `TaggedDataPayload`.

#[tokio::main]
async fn main() -> Result<()> {
    let tag = hex::decode("68656c6c6f").unwrap();
    let data = hex::decode("776f726c64").unwrap();

    let (tag_utf8, data_utf8) = Client::tagged_data_to_utf8(&TaggedDataPayload::new(tag, data).unwrap()).unwrap();

    println!("tag: {}\ndata: {}", tag_utf8, data_utf8);

    Ok(())
}
