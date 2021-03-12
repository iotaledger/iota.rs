// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migration --release
use anyhow::Result;
use iota::{
    client::migration::{
        create_migration_bundle, get_trytes_from_bundle, mine, sign_migration_bundle,
        Address as ChrysalisAddress,
    },
    signing::ternary::seed::Seed as TernarySeed,
    ternary::{T1B1Buf, T3B1Buf, TryteBuf},
    transaction::bundled::BundledTransaction,
    transaction::bundled::BundledTransactionField,
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
        // .permanode("https://permanode.org")?
        .build()?;
    let tryte_seed = TernarySeed::from_trits(
        TryteBuf::try_from_str(
            "TRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEEDTRYTESEED",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )?;
    // Funds will be migrated to this address
    let bech32_address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

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

    //Convert migraton address
    let new_address = ChrysalisAddress::try_from_bech32(bech32_address)?;
    let new_converted_address = match new_address {
        ChrysalisAddress::Ed25519(a) => a,
        _ => panic!("Unsopported address type"),
    };

    // Create bundle
    let mut prepared_bundle =
        create_migration_bundle(&iota, new_converted_address, account_input_data.1.clone()).await?;

    // // Get Trytes as String
    // let bundle_for_trytes =
    //     create_migration_bundle(&iota, new_converted_address, account_input_data.1.clone())
    //         .await?;
    // let bundle_trytes = get_trytes_from_bundle(bundle_for_trytes);
    // println!("raw txs : {:?}",bundle_trytes);
    // // Sign trytes somewhere else here
    // // Convert Tryte Strings back to Transactions
    // let mut signed_bundle_trytes: Vec<BundledTransaction> = bundle_trytes.into_iter().map(|tx|{
    //     BundledTransaction::from_trits(&TryteBuf::try_from_str(
    //         &tx,
    //     )
    //     .unwrap()
    //     .as_trits()).expect("Can't build transaction from String")
    // }).collect();
    // // Reverse for correct attachment order
    // signed_bundle_trytes.reverse();

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
            5,
        )
        .await?;
        println!("Mining info: {:?}", mining_result.0);
        prepared_bundle = mining_result.1;
    } else {
        println!("No spent address as input");
    }
    let signed_bundle_trytes =
        sign_migration_bundle(tryte_seed, prepared_bundle, account_input_data.1)?;

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
