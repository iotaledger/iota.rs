// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example crackability --release
use anyhow::Result;
use iota::{
    client::migration::{
        create_migration_bundle, mine, sign_migration_bundle, Address as ChrysalisAddress,
    },
    crypto::keys::ternary::seed::Seed as TernarySeed,
    ternary::{T1B1Buf, T3B1Buf, TritBuf, TryteBuf},
    transaction::bundled::BundledTransactionField,
};
use std::collections::HashMap;

/// Migration example
#[tokio::main]
async fn main() -> Result<()> {
    let security_level: u8 = 2;
    let mut iota = iota::ClientBuilder::new()
        .node("https://nodes-migration3-legacy.iota.cafe/")?
        .quorum(true)
        .permanode("http://permanode.migration3.iota.cafe:4000/api")?
        .build()?;
    let tryte_seed = TernarySeed::from_trits(
        TryteBuf::try_from_str(
            "VNRONDDCZLB9JZXFSCUPCMKBNDKJFLDTMYBURXWTG9RSDWZVYNUUEFSFNQZKSFRVJFNFNEMRFVZUSSVUW",
            // "LIIWD9FVIOLNUESFRVJNMJALLNWBZPDRB9QSGYKYFJCUDADSWEUIPCYPBBBCWDPNIISLHGJNZAYTCQYXW",
            // "STMILGEPNLFM9YPLGUM9RMPPZEJEMBKXUIDH9PVBTQDUPILKLNSPGXKUVDBIRPPBPUMWYBIUHEYNTZDUW",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )
    .unwrap();
    // Funds will be migrated to this address
    let bech32_address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    // Get account data
    let mut account_input_data = (0, vec![]);
    let address_index = 0;
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
        _ => panic!("Unsupported address type"),
    };

    // Create bundle
    let mut prepared_bundle =
        create_migration_bundle(&iota, new_converted_address, account_input_data.1.clone()).await?;

    // Ideally split inputs to have one bundle for each spent address
    let mut crackability: f64 = 0.0;
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
            spent_bundle_hashes.clone(),
            5,
            0,
        )
        .await?;
        println!("Mining info: {:?}", mining_result.0);
        crackability = mining_result.0.crackability;
        prepared_bundle = mining_result.1;
    } else {
        println!("No spent address as input");
    }
    let signed_bundle_trytes =
        sign_migration_bundle(tryte_seed, prepared_bundle, account_input_data.1)?;

    let bundlehash = signed_bundle_trytes[0]
        .bundle()
        .to_inner()
        .encode::<T3B1Buf>()
        .iter_trytes()
        .map(char::from)
        .collect::<String>();
    println!("Bundle sent: {:?}", bundlehash);
    let hashes_trit_i8_test = spent_bundle_hashes
        .clone()
        .iter()
        .map(|t| {
            TryteBuf::try_from_str(&(*t).to_string())
                .unwrap()
                .as_trits()
                .encode()
        })
        .collect::<Vec<TritBuf<T1B1Buf>>>();
    let security_level = 2;
    let p_actual =
        iota_bundle_miner::recoverer::get_crack_probability(security_level, &hashes_trit_i8_test);
    println!("without mined {}", p_actual);
    spent_bundle_hashes.push(bundlehash);
    let hashes_trit_i8_test = spent_bundle_hashes
        .clone()
        .iter()
        .map(|t| {
            TryteBuf::try_from_str(&(*t).to_string())
                .unwrap()
                .as_trits()
                .encode()
        })
        .collect::<Vec<TritBuf<T1B1Buf>>>();
    let p_actual =
        iota_bundle_miner::recoverer::get_crack_probability(security_level, &hashes_trit_i8_test);
    println!("with mined    {}", p_actual);
    assert_eq!(p_actual, crackability);
    Ok(())
}
