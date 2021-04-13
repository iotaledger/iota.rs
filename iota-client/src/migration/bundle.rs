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
use iota_bundle_miner::{miner::MinerInfo, CrackabilityMinerEvent, MinerBuilder, RecovererBuilder};

/// Dust protection treshhold: minimum amount of iotas an address needs in Chrysalis
pub const DUST_THRESHOLD: u64 = 1_000_000;

/// Prepare migration bundle with address and inputs
pub async fn create_migration_bundle(
    client: &Client,
    address: Ed25519Address,
    inputs: Vec<InputData>,
) -> Result<OutgoingBundleBuilder> {
    let migration_address = encode_migration_address(address)?;

    if inputs.is_empty() {
        return Err(Error::MigrationError("No inputs provided"));
    }

    let security_level = inputs[0].security_lvl;
    let mut address_inputs = Vec::new();
    let mut total_value = 0;
    for input in inputs {
        if input.security_lvl != security_level {
            return Err(Error::MigrationError(
                "Not all inputs have the same security level",
            ));
        }
        let new_input = Input {
            address: input.address,
            balance: input.balance,
            index: input.index,
        };
        //Remove possible duplicates
        if address_inputs
            .binary_search_by(|input: &Input| input.index.cmp(&new_input.index))
            .is_err()
        {
            total_value += new_input.balance;
            address_inputs.push(new_input)
        }
    }

    // Check for dust protection value
    if total_value < DUST_THRESHOLD {
        return Err(Error::MigrationError(
            "Input value is < dust protection value (1_000_000 i)",
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
    if inputs.is_empty() {
        return Err(Error::MigrationError("No inputs provided"));
    }
    let security_level_u8 = inputs[0].security_lvl;
    let security_level = match security_level_u8 {
        1 => WotsSecurityLevel::Low,
        2 => WotsSecurityLevel::Medium,
        3 => WotsSecurityLevel::High,
        _ => return Err(Error::MigrationError("Invalid security level provided")),
    };
    // Validate that all inputs have the same security level
    let same_security_level = inputs.iter().all(|i| i.security_lvl == security_level_u8);
    if !same_security_level {
        return Err(Error::MigrationError(
            "Not all inputs have the same security level",
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
        .map(|i| (i.index as usize, i.address, security_level))
        .collect();
    // Sign
    let final_signed_bundle = prepared_bundle
        .seal()?
        .sign(&tryte_seed, &inputs[..])?
        .attach_local(Hash::zeros(), Hash::zeros())?
        .build()?;

    let mut trytes: Vec<BundledTransaction> = final_signed_bundle.into_iter().collect();
    let input_addresses: Vec<Address> = inputs.into_iter().map(|input| input.1).collect();
    let mut bundle_addresses: Vec<Address> = trytes.iter().map(|tx| tx.address().clone()).collect();
    bundle_addresses.dedup();
    // Check if all and only input addresses were used (-1 for the migration address)
    if input_addresses.len() != bundle_addresses.len() - 1 {
        return Err(Error::MigrationError(
            "Input address amount does't match created bundle",
        ));
    }
    for address in input_addresses {
        if !bundle_addresses.contains(&address) {
            return Err(Error::MigrationError("Not all input addresses in bundle"));
        }
    }
    // Reverse for correct order when doing PoW
    trytes.reverse();
    Ok(trytes)
}

/// mine a bundle essence to reveal as least new parts of the signature as possible
pub async fn mine(
    prepared_bundle: OutgoingBundleBuilder,
    security_level: u8,
    ledger: bool,
    spent_bundle_hashes: Vec<String>,
    timeout: u64,
    offset: i64,
) -> Result<(MinerInfo, OutgoingBundleBuilder)> {
    if spent_bundle_hashes.is_empty() {
        return Err(Error::MigrationError(
            "Can't mine without spent_bundle_hashes",
        ));
    }
    let bundle = prepared_bundle
        .seal()?
        .sign(&TernarySeed::rand(), &[])?
        .attach_local(Hash::zeros(), Hash::zeros())?
        .build()?;
    let mut txs = Vec::with_capacity(bundle.len());
    for tx in bundle {
        txs.push(tx)
    }

    let essence_parts = get_bundle_essence_parts(&txs);
    let mut miner_builder = MinerBuilder::new()
        .with_offset(offset)
        .with_essences_from_unsigned_bundle(
            essence_parts
                .iter()
                .map(|t| {
                    Ok(TryteBuf::try_from_str(&(*t).to_string())?
                        .as_trits()
                        .encode())
                })
                .collect::<Result<Vec<TritBuf<T1B1Buf>>>>()?,
        )
        .with_security_level(security_level as usize);
    // Ledger Nano App rejects bundles that contain a 13 anywhere in the signed fragments
    let with_num_13_free_fragments = match ledger {
        true => 81,
        false => (security_level * 27) as usize,
    };
    miner_builder = miner_builder.with_num_13_free_fragments(with_num_13_free_fragments);
    // Use one worker less than we have cores or 1 if there is only one core
    let mut worker_count = num_cpus::get();
    if worker_count > 1 {
        worker_count -= 1;
    } else {
        worker_count = 1;
    }
    let miner = miner_builder
        .with_known_bundle_hashes(
            spent_bundle_hashes
                .iter()
                .map(|t| {
                    Ok(TryteBuf::try_from_str(&(*t).to_string())?
                        .as_trits()
                        .encode())
                })
                .collect::<Result<Vec<TritBuf<T1B1Buf>>>>()?,
        )
        .with_worker_count(worker_count)
        .with_core_thread_count(worker_count)
        .with_mining_timeout(timeout)
        .finish()?;

    let mut recoverer = RecovererBuilder::new()
        .with_security_level(security_level as usize)
        .with_known_bundle_hashes(
            spent_bundle_hashes
                .iter()
                .map(|t| {
                    Ok(TryteBuf::try_from_str(&(*t).to_string())?
                        .as_trits()
                        .encode())
                })
                .collect::<Result<Vec<TritBuf<T1B1Buf>>>>()?,
        )
        .miner(miner)
        .finish()?;
    // Todo: decide which crackability value is good enough
    let mined_info = match recoverer.recover().await {
        CrackabilityMinerEvent::MinerInfo(mined_info) => mined_info,
    };
    let updated_bundle = update_essence_with_mined_essence(
        txs,
        mined_info
            .mined_essence
            .clone()
            .ok_or(Error::MigrationError("Couldn't get mined_essence"))?,
    )?;
    Ok((mined_info, updated_bundle))
}

/// Get Trytes from an OutgoingBundleBuilder
pub fn get_trytes_from_bundle(
    created_migration_bundle: OutgoingBundleBuilder,
) -> Result<Vec<String>> {
    let bundle = created_migration_bundle
        .seal()?
        .sign(&TernarySeed::rand(), &[])?
        .attach_local(Hash::zeros(), Hash::zeros())?
        .build()?;

    let mut trytes = Vec::with_capacity(bundle.len());
    for tx in bundle {
        let mut trits = TritBuf::<T1B1Buf>::zeros(8019);
        tx.as_trits_allocated(&mut trits);
        trytes.push(
            trits
                .encode::<T3B1Buf>()
                .iter_trytes()
                .map(char::from)
                .collect::<String>(),
        );
    }
    Ok(trytes)
}

// Update latest tx essence with mined essence part
fn update_essence_with_mined_essence(
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
    prepared_txs[tx_len - 1] = BundledTransaction::from_trits(&trits)?;

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
    Ok(bundle)
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
