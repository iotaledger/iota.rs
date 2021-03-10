// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The migration bundle module
use crate::{
    client::Client,
    error::{Error, Result},
    extended::PrepareTransfersBuilder,
    migration::encode_migration_address,
    response::{Input, InputData},
    Transfer,
};

use bee_crypto::ternary::Hash;
use bee_message::prelude::Ed25519Address;
use bee_signing::ternary::{seed::Seed as TernarySeed, wots::WotsSecurityLevel};
use bee_ternary::{T1B1Buf, T3B1Buf, TritBuf, TryteBuf};
use bee_transaction::bundled::{
    Address, BundledTransaction, BundledTransactionBuilder, BundledTransactionField, Nonce,
    OutgoingBundleBuilder, Payload, Timestamp,
};
use iota_bundle_miner::{
    miner::MinedCrackability, CrackabilityMinerEvent, MinerBuilder, RecovererBuilder,
};

/// Prepare migration bundle with address and inputs
pub async fn prepare_migration_bundle(
    client: &Client,
    address: Ed25519Address,
    inputs: Vec<InputData>,
) -> Result<OutgoingBundleBuilder> {
    let migration_address = encode_migration_address(address);

    let security_level = inputs[0].security_lvl;
    let same_security_level = inputs.iter().all(|i| i.security_lvl == security_level);
    if !same_security_level {
        return Err(Error::MigrationError(
            "Not all inputs have the same security level".into(),
        ));
    }

    let mut address_inputs: Vec<Input> = inputs
        .into_iter()
        .map(|i| Input {
            address: i.address,
            balance: i.balance,
            index: i.index,
        })
        .collect();
    //Remove possible duplicates
    address_inputs.dedup();

    let total_value = address_inputs.iter().map(|d| d.balance).sum();

    // Check for dust protection value
    if total_value > 1_000_000 {
        return Err(Error::MigrationError(
            "Input value is < dust protection value (1_000_000 i)".into(),
        ));
    }
    let transfer = vec![Transfer {
        address: migration_address,
        value: total_value,
        message: None,
        tag: None,
    }];

    PrepareTransfersBuilder::new(client, None)
        .security(security_level)
        .transfers(transfer)
        .inputs(address_inputs)
        .build_unsigned()
        .await
}

/// Sign a prepared bundle, inputs need to be the same as when it was prepared
pub fn sign_migration_bundle(
    tryte_seed: TernarySeed,
    prepared_bundle: OutgoingBundleBuilder,
    inputs: Vec<InputData>,
) -> Result<Vec<BundledTransaction>> {
    let security_level = inputs[0].security_lvl;
    let same_security_level = inputs.iter().all(|i| i.security_lvl == security_level);
    if !same_security_level {
        return Err(Error::MigrationError(
            "Not all inputs have the same security level".into(),
        ));
    }

    let mut address_inputs: Vec<Input> = inputs
        .into_iter()
        .map(|i| Input {
            address: i.address,
            balance: i.balance,
            index: i.index,
        })
        .collect();
    address_inputs.dedup();
    let inputs: Vec<(usize, Address, WotsSecurityLevel)> = address_inputs
        .into_iter()
        .map(|i| {
            (
                i.index as usize,
                i.address,
                match security_level {
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

/// mine a bundle essence to reveal as least new parts of the signature as possible
pub fn mine(
    prepared_bundle: OutgoingBundleBuilder,
    security_level: u8,
    ledger: bool,
    spent_bundle_hashes: Vec<String>,
    timeout: u64,
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
        .with_mining_timeout(timeout)
        .finish()
        .unwrap();

    let mut recoverer = RecovererBuilder::new()
        .with_security_level(security_level as usize)
        .with_known_bundle_hashes(
            spent_bundle_hashes
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
    );
    Ok((mined_info, updated_bundle))
}

// Update latest tx essence with mined essence part
fn update_essence_with_mined_essence(
    mut prepared_txs: Vec<BundledTransaction>,
    latest_tx_essence_part: TritBuf<T1B1Buf>,
) -> OutgoingBundleBuilder {
    // Replace obsolete tag of the last transaction with the mined obsolete_tag
    let mut trits = TritBuf::<T1B1Buf>::zeros(8019);
    prepared_txs[prepared_txs.len() - 1].as_trits_allocated(trits.as_slice_mut());
    trits
        .subslice_mut(6804..7047)
        .copy_from(&latest_tx_essence_part);
    let tx_len = prepared_txs.len();
    prepared_txs[tx_len - 1] = BundledTransaction::from_trits(&trits).unwrap();

    // Create final bundle with updated obsolet_tag(mined essence part)
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
    bundle
}

// Split each tx in two essence parts, first one is the address and the second one
// includes value, obsoleteTag, currentIndex, lastIndex and timestamp
fn get_bundle_essence_parts(txs: &[BundledTransaction]) -> Vec<String> {
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
