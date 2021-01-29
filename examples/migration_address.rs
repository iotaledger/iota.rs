// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migration_address --release
use bee_signing_ext::Seed;
use iota::client::chrysalis2::*;

/// In this example we create addresses from a seed
#[tokio::main]
async fn main() {
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();

    let ed25519_address = GetAddressesBuilder::new(&seed)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .unwrap();

    println!(
        "Generated migration address with checksum: {}",
        add_tryte_checksum(encode_migration_address(ed25519_address[0]))
    );
}
