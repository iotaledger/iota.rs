// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migration --release
use anyhow::Result;
use iota::client::chrysalis2::*;
use iota::{
    client::extended::PrepareTransfersBuilder,
    client::migration::{encode_migration_address, mine, sign_migration_bundle},
    client::response::Input,
    client::Transfer,
    signing::ternary::seed::Seed as TernarySeed,
    ternary::{T1B1Buf, T3B1Buf, TryteBuf},
    transaction::bundled::{Address, BundledTransactionField},
};
use std::collections::HashMap;
use std::io;

/// Migration example
#[tokio::main]
async fn main() -> Result<()> {
    let security_level: u8 = 2;
    let min_weight_magnitude = 9;
    let ledger = false;
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

    // Get account data
    let mut account_input_data = (0, vec![]);
    let mut address_index = 0;
    let yes = vec!['Y', 'y'];
    let mut user_input = String::new();
    while !yes.contains(&user_input.chars().next().unwrap_or('N')) {
        println!("Searching for balance...");
        let more_inputs = iota
            .get_account_data_for_migration()
            .with_seed(&tryte_seed)
            .with_security(security_level as u8)
            .with_start_index(address_index)
            .finish()
            .await?;
        account_input_data.1.extend(more_inputs.1);
        // Filter duplicates because when it's called another time it could return duplicated entries
        let mut unique_address_data = HashMap::new();
        for data in account_input_data.1 {
            unique_address_data.insert(data.index, data);
        }
        account_input_data.1 = unique_address_data
            .into_iter()
            .map(|(_index, data)| data)
            .collect();
        // Get total available balance
        account_input_data.0 = account_input_data.1.iter().map(|d| d.balance).sum();
        println!("{:?}", account_input_data);
        println!(
            "Is {}i the correct balance? Type Y to continue or N to search for more balance",
            account_input_data.0
        );
        user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        address_index += 30;
    }
    // if account_input_data.0 < 1_000_000 {
    //     panic!("Balance needs to be > 1_000_000i to do the migration because of the dust protection")
    // }
    println!("Preparing transaction...");
    let mut spent_bundle_hashes = Vec::new();

    for input in &account_input_data.1 {
        if let Some(bundle_hashes) = input.spent_bundlehashes.clone() {
            spent_bundle_hashes.extend(bundle_hashes)
        }
    }
    println!("spent_bundle_hashes {:?}", spent_bundle_hashes);

    // Create bundle
    let _migration_address = generate_migration_address(ed25519_seed);
    // overwrite to reuse tokens for testing
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
        value: account_input_data.0,
        message: None,
        tag: None,
    }];

    let address_inputs = account_input_data
        .1
        .iter()
        .cloned()
        .map(|i| Input {
            address: i.address,
            balance: i.balance,
            index: i.index,
        })
        .collect();

    let mut prepared_bundle = PrepareTransfersBuilder::new(&iota, None)
        .security(security_level)
        .transfers(transfer)
        .inputs(address_inputs)
        .build_unsigned()
        .await?;

    // Ideally split inputs to have one bundle for each spent address
    if account_input_data
        .1
        .iter()
        .map(|d| d.spent)
        .collect::<Vec<bool>>()
        .contains(&true)
    {
        println!("Mining bundle because of spent addresses, this can take some time..."); //40 seconds in this case
                                                                                          // Mine bundle essence
        let mining_result = mine(
            prepared_bundle,
            security_level,
            ledger,
            spent_bundle_hashes,
            40,
        )?;
        println!("Mining info: {:?}", mining_result.0);
        prepared_bundle = mining_result.1;
    } else {
        println!("No spent address as input");
    }
    let signed_bundle_trytes =
        sign_migration_bundle(tryte_seed, prepared_bundle, account_input_data)?;

    // Send to Tangle
    let send_trytes = iota
        .send_trytes()
        .with_trytes(signed_bundle_trytes)
        .with_depth(2)
        .with_min_weight_magnitude(min_weight_magnitude)
        .finish()
        .await?;
    println!(
        "Bundle sent: {:?}",
        send_trytes[0]
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
    let seed = Seed::from_bytes(&hex::decode(ed25519_seed)?).unwrap();

    let ed25519_address = GetAddressesBuilder::new(&seed)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .unwrap();
    Ok(encode_migration_address(ed25519_address[0]))
}
