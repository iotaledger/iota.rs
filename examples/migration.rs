// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migration --release
use anyhow::Result;
use bee_signing_ext::Seed;
use iota::client::chrysalis2::*;
use iota::{
    client::Transfer,
    signing::ternary::seed::Seed as TernarySeed,
    ternary::{T1B1Buf, T3B1Buf, TryteBuf},
    transaction::bundled::{Address, BundledTransactionField},
};
use std::io;

/// Migration example
#[tokio::main]
async fn main() -> Result<()> {
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.devnet.iota.org")?
        .build()?;
    let tryte_seed = TernarySeed::from_trits(
        TryteBuf::try_from_str(
            "TRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEED",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )?;
    let ed25519_seed = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";

    let mut inputs = (0, vec![]);
    let mut address_index = 0;
    let yes = vec!['Y', 'y'];
    let mut user_input = String::new();
    while !yes.contains(&user_input.chars().next().unwrap_or('N')) {
        println!("Searching for balance...");
        let more_inputs = iota
            .get_all_inputs()
            .with_seed(&tryte_seed)
            .with_start_index(address_index)
            .finish()
            .await?;
        inputs.0 += more_inputs.0;
        inputs.1.extend(more_inputs.1);
        println!("{:?}", inputs);
        println!(
            "Is {}i the correct balance? Type Y to continue or N to search for more balance",
            inputs.0
        );
        user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        address_index += 30;
    }
    // if inputs.0 < 1_000_000 {
    //     panic!("Balance needs to be > 1_000_000i to do transaction")
    // }
    println!("Preparing transaction...");
    // Get spent status of addresses, if spent do bundle mining
    // let input_addresses: Vec<Address> = inputs.1.iter().map(|i| i.address.clone()).collect();
    // let spent_status = iota.were_addresses_spent_from(&input_addresses[..]).await?;
    // if spent_status.states.contains(&true) {
    //     println!("Mining bundle because of spent addresses, this can take some time...");
    // }

    //Send final bundle
    let _migration_address = generate_migration_address(ed25519_seed);
    // placeholder to reuse tokens for testing
    let migration_address = Address::from_inner_unchecked(
        TryteBuf::try_from_str(
            "CHZHKFUCUMRHOFXB9SGEZVYUUXYKEIJ9VX9SLKATMLWQZUQXDWUKLYGZLMYYWHXKKTPQHIOHQMYARINLD",
        )
        .unwrap()
        .as_trits()
        .encode(),
    );

    let transfer = vec![Transfer {
        address: migration_address,
        value: inputs.0,
        message: None,
        tag: None,
    }];

    let res = iota
        .send(Some(&tryte_seed))
        .with_transfers(transfer)
        .with_inputs(inputs.1)
        .with_min_weight_magnitude(9)
        .finish()
        .await?;

    println!(
        "Bundle sent: {:?}",
        res[0]
            .bundle()
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
    );
    Ok(())
}

// Generate first ed25519 address and convert it to a migration tryte address
fn generate_migration_address(ed25519_seed: &str) -> Result<Address> {
    let seed = Seed::from_ed25519_bytes(&hex::decode(ed25519_seed)?).unwrap();

    let ed25519_address = GetAddressesBuilder::new(&seed)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .unwrap();
    Ok(encode_migration_address(ed25519_address[0]))
}
