// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 0_address_generation --release
use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::{env, fs::File, io::BufWriter, path::Path};

/// In this example we will generate addresses which will be used later to find inputs

const ADDRESS_FILE_NAME: &str = "examples/offline_signing/addresses.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota_offline = Client::builder().with_offline_mode().finish().await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    // Generate addresses offline
    let addresses = iota_offline
        .get_addresses(&seed)
        .with_range(0..10)
        .with_bech32_hrp("atoi".into())
        .finish()
        .await?;
    println!("{:?}", addresses);

    write_addresses_to_file(ADDRESS_FILE_NAME, addresses)?;
    Ok(())
}

pub fn write_addresses_to_file<P: AsRef<Path>>(path: P, addresses: Vec<String>) -> Result<()> {
    let jsonvalue = serde_json::to_value(&addresses)?;
    let file = File::create(path)?;
    let bw = BufWriter::new(file);
    serde_json::to_writer_pretty(bw, &jsonvalue)?;
    Ok(())
}
