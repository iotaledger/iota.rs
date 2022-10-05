// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example generate_mnemonic --release

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic = Client::generate_mnemonic()?;

    println!("Mnemonic: {mnemonic}");

    Ok(())
}
