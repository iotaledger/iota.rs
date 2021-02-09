// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migration --release
use anyhow::Result;
use bee_signing_ext::Seed;
use iota::client::chrysalis2::*;
use iota::{
    client::extended::PrepareTransfersBuilder,
    client::Transfer,
    signing::ternary::seed::Seed as TernarySeed,
    ternary::{T1B1Buf, T3B1Buf, TryteBuf, TritBuf},
    transaction::bundled::{Address, BundledTransactionField, BundledTransaction},
};
use iota_bundle_miner::{MinerBuilder, RecovererBuilder, CrackabilityMinerEvent};
use std::collections::HashSet;
use std::io;

/// Migration example
#[tokio::main]
async fn main() -> Result<()> {
    let security_level: u8 = 2;
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
            .with_security(security_level as u8)
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
    let input_addresses: Vec<Address> = inputs.1.iter().map(|i| i.address.clone()).collect();
    let spent_status = iota.were_addresses_spent_from(&input_addresses[..]).await?;
    println!("spent_status {:?}", spent_status);
    let mut spent_addresses = Vec::new();
    for (index, spent) in spent_status.states.iter().enumerate() {
        //TODO change to *spent is ! because devnet is broken
        if !*spent {
            spent_addresses.push(input_addresses[index].clone());
        }
    }
    let tx_hashes_on_spent_addresses = iota
        .find_transactions()
        .addresses(&spent_addresses)
        .send()
        .await?
        .hashes;
    let txs_on_spent_addresses = iota.get_trytes(&tx_hashes_on_spent_addresses).await?.trytes;
    let mut known_bundle_hashes = HashSet::new();
    for tx in txs_on_spent_addresses {
        if *tx.value().to_inner() < 0 {
            known_bundle_hashes.insert(tx.bundle().clone());
        }
    }
    let known_bundle_hashes: Vec<String> = known_bundle_hashes.into_iter().map(|b| b.to_inner().encode::<T3B1Buf>()
    .iter_trytes()
    .map(char::from)
    .collect::<String>()).collect();
    println!("bundle_hashes {:?}", known_bundle_hashes);
    // TODO change to true, is false because devnet is broken
    if spent_status.states.contains(&false) {
        println!("Mining bundle because of spent addresses, this can take some time...");
    // let res = RecovererBuilder::new().with_security_level(2).with_known_bundle_hashes(bundle_hashes.into_iter().collect()).finish()?;
    } else {
        println!("No spent address as input");
    }
        // Create bundle
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
    let bundle = PrepareTransfersBuilder::new(&iota, Some(&tryte_seed)).security(security_level).transfers(transfer).inputs(inputs.1).build().await?;
    let mut txs = Vec::new();
    for i in 0..bundle.len(){
        txs.push(bundle.get(i).unwrap());
    }
    // println!("{:?}",txs);
    let essence_parts = get_bundle_essence_parts(&txs);
    println!("essence_parts: {:?}",essence_parts);


    let mined_iteration_expected: usize = 28925;
    let mined_crackability_expected: f64 = 9.2430303968744906557891424497679297e-16;
    let mut miner = MinerBuilder::new()
        .with_offset(0)
        .with_essences_from_unsigned_bundle(
            essence_parts
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_security_level(security_level as usize)
        .with_known_bundle_hashes(
            known_bundle_hashes
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_worker_count(1)
        .with_core_thread_count(1)
        .with_mining_timeout(40)
        .finish()
        .unwrap();

    let mut recoverer = RecovererBuilder::new()
        .with_security_level(security_level as usize)
        .with_known_bundle_hashes(
            known_bundle_hashes
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_threshold(1e-15_f64)
        .miner(miner)
        .finish()
        .unwrap();

    if let CrackabilityMinerEvent::MinedCrackability(mined_info) = recoverer.recover() {
        println!("{:?}",mined_info);
        assert_eq!(mined_iteration_expected, mined_info.mined_iteration);
        assert_eq!(
            true,
            (mined_crackability_expected - mined_info.crackability).abs()
                < mined_crackability_expected * 1e-9
        );
    } else {
        panic!();
    }

    //todo replace obsolete_tag, sign and send with new tips

    // println!(
    //     "Bundle sent: {:?}",
    //     res[0]
    //         .bundle()
    //         .to_inner()
    //         .encode::<T3B1Buf>()
    //         .iter_trytes()
    //         .map(char::from)
    //         .collect::<String>()
    // );
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

fn get_bundle_essence_parts(txs: &Vec<&BundledTransaction>) -> Vec<String>{
    let mut essence_parts = Vec::new();
    for tx in txs{
        let essence = tx.essence();
        essence_parts.push(essence[0..243].encode::<T3B1Buf>()
        .iter_trytes()
        .map(char::from)
        .collect::<String>());
        essence_parts.push(essence[243..].encode::<T3B1Buf>()
        .iter_trytes()
        .map(char::from)
        .collect::<String>());
    }
    essence_parts
}