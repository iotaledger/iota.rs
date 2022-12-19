// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example tagged_data_to_utf8 --release
//! In this example we will UTF-8 encode the tag and the data of an `TaggedDataPayload`.

use iota_client::{block::payload::TaggedDataPayload, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // `hello` in hexadecimal.
    let tag = prefix_hex::decode("0x68656c6c6f")?;
    // `world` in hexadecimal.
    let data = prefix_hex::decode("0x776f726c64")?;

    let (tag_utf8, data_utf8) = Client::tagged_data_to_utf8(&TaggedDataPayload::new(tag, data)?)?;

    println!("tag: {tag_utf8}\ndata: {data_utf8}");

    Ok(())
}
