// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example migration --release
use anyhow::Result;
use iota::client::chrysalis2::*;
use iota::{
    client::extended::PrepareTransfersBuilder,
    client::Transfer,
    crypto::ternary::Hash,
    signing::ternary::{seed::Seed as TernarySeed, wots::WotsSecurityLevel},
    ternary::{T1B1Buf, T3B1Buf, TritBuf, TryteBuf},
    transaction::bundled::{
        Address, BundledTransaction, BundledTransactionBuilder, BundledTransactionField, Nonce,
        OutgoingBundleBuilder, Payload, Timestamp,
    },
};
use iota_bundle_miner::{CrackabilityMinerEvent, MinerBuilder, RecovererBuilder};
use std::collections::HashSet;
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
    //     panic!("Balance needs to be > 1_000_000i to do the migration because of the dust protection")
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
    let known_bundle_hashes: Vec<String> = known_bundle_hashes
        .into_iter()
        .map(|b| {
            b.to_inner()
                .encode::<T3B1Buf>()
                .iter_trytes()
                .map(char::from)
                .collect::<String>()
        })
        .collect();
    println!("bundle_hashes {:?}", known_bundle_hashes);

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

    // TODO change to true, is false because devnet is broken
    if spent_status.states.contains(&false) {
        println!("Mining bundle because of spent addresses, this can take some time...");
        // Provide random seed here, because we can't build a bundle without signed inputs, signature will be replaced later
        let bundle = PrepareTransfersBuilder::new(&iota, Some(&TernarySeed::rand()))
            .security(security_level)
            .transfers(transfer)
            .inputs(inputs.1.clone())
            .build()
            .await?;
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
            // use num_cpus::get()? Or not all, because it could lag?
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
            .miner(miner)
            .finish()
            .unwrap();
        // Todo: decide which crackability value is good enough
        let mined_info = match recoverer.recover() {
            CrackabilityMinerEvent::MinedCrackability(mined_info) => mined_info,
            CrackabilityMinerEvent::Timeout(mined_info) => mined_info,
        };
        let latest_tx_essence_part = mined_info.mined_essence;

        // Replace obsolete tag of the last transaction with the mined obsolete_tag
        let mut trits = TritBuf::<T1B1Buf>::zeros(8019);
        txs[txs.len() - 1].as_trits_allocated(trits.as_slice_mut());
        trits
            .subslice_mut(6804..7047)
            .copy_from(&latest_tx_essence_part.unwrap());
        let tx_len = txs.len();
        txs[tx_len - 1] = BundledTransaction::from_trits(&trits).unwrap();

        // Create final bundle with updated obsolet_tag
        let mut bundle = OutgoingBundleBuilder::default();
        for tx in txs.into_iter() {
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

        let security = match security_level {
            1 => WotsSecurityLevel::Low,
            2 => WotsSecurityLevel::Medium,
            3 => WotsSecurityLevel::High,
            _ => panic!("Invalid scurity level"),
        };

        let inputs: Vec<(usize, Address, WotsSecurityLevel)> = inputs
            .1
            .into_iter()
            .map(|i| (i.index as usize, i.address, security))
            .collect();
        // Sign
        // Todo signing with a Ledger
        let final_signed_bundle = bundle
            .seal()
            .expect("Fail to seal bundle")
            .sign(&tryte_seed, &inputs[..])
            .expect("Fail to sign bundle")
            .attach_local(Hash::zeros(), Hash::zeros())
            .expect("Fail to attach bundle")
            .build()
            .expect("Fail to build bundle");

        // Send to Tangle
        let mut trytes: Vec<BundledTransaction> = final_signed_bundle.into_iter().collect();
        trytes.reverse();
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
