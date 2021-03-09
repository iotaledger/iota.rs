// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migration --release
use anyhow::Result;
use iota::client::chrysalis2::*;
use iota::{
    client::extended::PrepareTransfersBuilder,
    client::response::{Input, InputData},
    client::Transfer,
    crypto::ternary::Hash,
    signing::ternary::{seed::Seed as TernarySeed, wots::WotsSecurityLevel},
    ternary::{T1B1Buf, T3B1Buf, TritBuf, TryteBuf},
    transaction::bundled::{
        Address, BundledTransaction, BundledTransactionBuilder, BundledTransactionField, Nonce,
        OutgoingBundleBuilder, Payload, Timestamp,
    },
};
use iota_bundle_miner::{
    miner::MinedCrackability, CrackabilityMinerEvent, MinerBuilder, RecovererBuilder,
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

    // Ideally split inputs to have one bundle for each spent address
    if account_input_data
        .1
        .iter()
        .map(|d| d.spent)
        .collect::<Vec<bool>>()
        .contains(&true)
    {
        println!("Mining bundle because of spent addresses, this can take some time...");
        // Provide random seed here, because we can't build a bundle without signed inputs, signature will be replaced later
        let bundle = PrepareTransfersBuilder::new(&iota, None)
            .security(security_level)
            .transfers(transfer)
            .inputs(address_inputs)
            .build_unsigned()
            .await?;

        // Mine bundle essence
        let mining_result = mine(bundle, security_level, ledger, spent_bundle_hashes, 40)?;
        println!("Mining info: {:?}", mining_result.0);
        // let latest_tx_essence_part = mined_info.mined_essence;

        // let trytes = sign_migration_bundle(tryte_seed, bundle, account_input_data)?;
        let trytes = sign_migration_bundle(tryte_seed, mining_result.1, account_input_data)?;

        // Send to Tangle
        let send_trytes = iota
            .send_trytes()
            .with_trytes(trytes)
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
    } else {
        println!("No spent address as input");
        // No need to do bundle mining, we can sign and send it right away
        let res = iota
            .send(Some(&tryte_seed))
            .with_transfers(transfer)
            .with_inputs(address_inputs)
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
    }
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

// Split each tx in two essence parts, first one is the address and the second one
// includes value, obsoleteTag, currentIndex, lastIndex and timestamp
fn get_bundle_essence_parts(txs: &Vec<BundledTransaction>) -> Vec<String> {
    let mut essence_parts = Vec::new();
    for tx in txs {
        let essence = tx.essence();
        // address
        essence_parts.push(
            essence[0..243]
                .encode::<T3B1Buf>()
                .iter_trytes()
                .map(char::from)
                .collect::<String>(),
        );
        // value, obsoleteTag, currentIndex, lastIndex and timestamp
        essence_parts.push(
            essence[243..]
                .encode::<T3B1Buf>()
                .iter_trytes()
                .map(char::from)
                .collect::<String>(),
        );
    }
    essence_parts
}

pub fn sign_migration_bundle(
    tryte_seed: TernarySeed,
    prepared_bundle: OutgoingBundleBuilder,
    account_input_data: (u64, Vec<InputData>),
) -> Result<Vec<BundledTransaction>> {
    let inputs: Vec<(usize, Address, WotsSecurityLevel)> = account_input_data
        .1
        .into_iter()
        .map(|i| {
            (
                i.index as usize,
                i.address,
                match i.security_lvl {
                    1 => WotsSecurityLevel::Low,
                    2 => WotsSecurityLevel::Medium,
                    3 => WotsSecurityLevel::High,
                    _ => panic!("Invalid scurity level"),
                },
            )
        })
        .collect();
    // Sign
    let final_signed_bundle = prepared_bundle
        .seal()
        .expect("Fail to seal bundle")
        .sign(&tryte_seed, &inputs[..])
        .expect("Fail to sign bundle")
        .attach_local(Hash::zeros(), Hash::zeros())
        .expect("Fail to attach bundle")
        .build()
        .expect("Fail to build bundle");

    //Reverse for correct order when doing PoW
    let mut trytes: Vec<BundledTransaction> = final_signed_bundle.into_iter().collect();
    trytes.reverse();
    Ok(trytes)
}

pub fn mine(
    prepared_bundle: OutgoingBundleBuilder,
    security_level: u8,
    ledger: bool,
    spent_bundle_hashes: Vec<String>,
    time: u64,
) -> Result<(MinedCrackability, OutgoingBundleBuilder)> {
    let bundle = prepared_bundle
        .seal()
        .expect("Fail to seal bundle")
        .sign(&TernarySeed::rand(), &[])
        .expect("Can't sign bundle")
        .attach_local(Hash::zeros(), Hash::zeros())
        .expect("Fail to attach bundle")
        .build()
        .expect("Fail to build bundle");
    let mut txs = Vec::new();
    for i in 0..bundle.len() {
        txs.push(bundle.get(i).unwrap().clone());
    }
    let essence_parts = get_bundle_essence_parts(&txs);
    let mut miner_builder = MinerBuilder::new()
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
        .with_security_level(security_level as usize);
    // Ledger Nano App rejects bundles that contain a 13 anywhere in the signed fragments
    miner_builder = match ledger {
        true => miner_builder.with_num_13_free_fragments(81),
        false => miner_builder.with_num_13_free_fragments((security_level * 27) as usize),
    };
    let miner = miner_builder
        .with_known_bundle_hashes(
            spent_bundle_hashes
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
        // use num_cpus::get()? Or not all, because it could lag?
        .with_worker_count(1)
        .with_core_thread_count(1)
        .with_mining_timeout(time)
        .finish()
        .unwrap();

    let mut recoverer = RecovererBuilder::new()
        .with_security_level(security_level as usize)
        .with_known_bundle_hashes(
            spent_bundle_hashes
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
        .miner(miner)
        .finish()
        .unwrap();
    // Todo: decide which crackability value is good enough
    let mined_info = match recoverer.recover() {
        CrackabilityMinerEvent::MinedCrackability(mined_info) => mined_info,
        CrackabilityMinerEvent::Timeout(mined_info) => mined_info,
    };
    let updated_bundle = update_essence_with_mined_essence(
        txs,
        mined_info.mined_essence.clone().expect("No essence mined"),
    )?;
    Ok((mined_info, updated_bundle))
}

pub fn update_essence_with_mined_essence(
    mut prepared_txs: Vec<BundledTransaction>,
    latest_tx_essence_part: TritBuf<T1B1Buf>,
) -> Result<OutgoingBundleBuilder> {
    // Replace obsolete tag of the last transaction with the mined obsolete_tag
    let mut trits = TritBuf::<T1B1Buf>::zeros(8019);
    prepared_txs[prepared_txs.len() - 1].as_trits_allocated(trits.as_slice_mut());
    trits
        .subslice_mut(6804..7047)
        .copy_from(&latest_tx_essence_part);
    let tx_len = prepared_txs.len();
    prepared_txs[tx_len - 1] = BundledTransaction::from_trits(&trits).unwrap();

    // Create final bundle with updated obsolet_tag
    let mut bundle = OutgoingBundleBuilder::default();
    for tx in prepared_txs.into_iter() {
        bundle.push(
            BundledTransactionBuilder::new()
                .with_payload(Payload::zeros())
                .with_address(tx.address().clone())
                .with_value(tx.value().clone())
                .with_obsolete_tag(tx.obsolete_tag().clone())
                .with_timestamp(tx.timestamp().clone())
                .with_index(tx.index().clone())
                .with_last_index(tx.last_index().clone())
                .with_tag(tx.tag().clone())
                .with_attachment_ts(tx.attachment_ts().clone())
                .with_bundle(Hash::zeros())
                .with_trunk(Hash::zeros())
                .with_branch(Hash::zeros())
                .with_attachment_lbts(Timestamp::from_inner_unchecked(std::u64::MIN))
                .with_attachment_ubts(Timestamp::from_inner_unchecked(std::u64::MAX))
                .with_nonce(Nonce::zeros()),
        )
    }
    Ok(bundle)
}
