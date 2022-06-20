// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! PoW functions

pub mod miner;

use bee_block::{parent::Parents, payload::Payload, Block, BlockBuilder, BlockId};
use packable::PackableExt;
#[cfg(not(target_family = "wasm"))]
use {
    crate::api::miner::ClientMinerBuilder,
    bee_pow::providers::{miner::MinerCancel, NonceProviderBuilder},
};
#[cfg(target_family = "wasm")]
use {bee_block::payload::OptionalPayload, packable::Packable};

use crate::{api::miner::ClientMiner, Client, Error, Result};

/// Does PoW with always new tips
#[cfg(not(target_family = "wasm"))]
pub async fn finish_pow(client: &Client, payload: Option<Payload>) -> Result<Block> {
    let local_pow = client.get_local_pow().await;
    let pow_worker_count = client.pow_worker_count;
    let min_pow_score = client.get_min_pow_score().await?;
    let tips_interval = client.get_tips_interval().await;
    loop {
        let cancel = MinerCancel::new();
        let cancel_2 = cancel.clone();
        let payload_ = payload.clone();
        let mut parent_blocks = client.get_tips().await?;
        parent_blocks.sort_unstable_by_key(|a| a.pack_to_vec());
        parent_blocks.dedup();
        let time_thread = std::thread::spawn(move || Ok(pow_timeout(tips_interval, cancel)));
        let pow_thread = std::thread::spawn(move || {
            let mut client_miner = ClientMinerBuilder::new()
                .with_local_pow(local_pow)
                .with_cancel(cancel_2);
            if let Some(worker_count) = pow_worker_count {
                client_miner = client_miner.with_worker_count(worker_count);
            }
            do_pow(client_miner.finish(), min_pow_score, payload_, parent_blocks)
        });

        let threads = vec![pow_thread, time_thread];
        for t in threads {
            match t.join().expect("Failed to join threads.") {
                Ok(res) => {
                    if res.0 != 0 || !local_pow {
                        if let Some(block) = res.1 {
                            return Ok(block);
                        }
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

// PoW timeout, if we reach this we will restart the PoW with new tips, so the final block will never be lazy
#[cfg(not(target_family = "wasm"))]
fn pow_timeout(after_seconds: u64, cancel: MinerCancel) -> (u64, Option<Block>) {
    std::thread::sleep(std::time::Duration::from_secs(after_seconds));
    cancel.trigger();
    (0, None)
}

/// Does PoW
pub fn do_pow(
    client_miner: ClientMiner,
    min_pow_score: f64,
    payload: Option<Payload>,
    parent_blocks: Vec<BlockId>,
) -> Result<(u64, Option<Block>)> {
    let mut block = BlockBuilder::<ClientMiner>::new(Parents::new(parent_blocks)?);
    if let Some(p) = payload {
        block = block.with_payload(p);
    }
    let block = block
        .with_nonce_provider(client_miner, min_pow_score)
        .finish()
        .map_err(Error::BlockError)?;
    Ok((block.nonce(), Some(block)))
}

// Single threaded PoW for wasm
#[cfg(target_family = "wasm")]
use {
    bee_ternary::{b1t6, Btrit, T1B1Buf, TritBuf},
    crypto::hashes::ternary::{
        curl_p::{CurlPBatchHasher, BATCH_SIZE},
        HASH_LENGTH,
    },
    crypto::hashes::{blake2b::Blake2b256, Digest},
};

// Precomputed natural logarithm of 3 for performance reasons.
// See https://oeis.org/A002391.
#[cfg(target_family = "wasm")]
const LN_3: f64 = 1.098_612_288_668_109;
#[cfg(target_family = "wasm")]
// should take around one second to reach on an average CPU, so shouldn't cause a noticeable delay on tips_interval
const POW_ROUNDS_BEFORE_INTERVAL_CHECK: usize = 3000;
#[cfg(target_family = "wasm")]
/// Single threaded PoW function for wasm
pub async fn finish_single_thread_pow(
    client: &Client,
    network_id: u64,
    parent_blocks: Option<Vec<BlockId>>,
    payload: Option<bee_block::payload::Payload>,
    target_score: f64,
) -> crate::Result<Block> {
    let mut parent_blocks = match parent_blocks {
        Some(parents) => parents,
        None => client.get_tips().await?,
    };

    // return with 0 as nonce if remote PoW should be used
    if !client.get_local_pow().await {
        let mut block_bytes: Vec<u8> = Vec::new();
        network_id.pack(&mut block_bytes).unwrap();
        Parents::new(parent_blocks.clone())?.pack(&mut block_bytes).unwrap();
        OptionalPayload::pack(&OptionalPayload::from(payload.clone()), &mut block_bytes)
            .map_err(|_| crate::Error::PackableError)?;
        (0_u64).pack(&mut block_bytes).unwrap();
        return Block::unpack_verified(&mut block_bytes.as_slice()).map_err(|_| crate::Error::PackableError);
    }

    let tips_interval = client.get_tips_interval().await;

    loop {
        let mut block_bytes: Vec<u8> = Vec::new();
        network_id.pack(&mut block_bytes).unwrap();
        Parents::new(parent_blocks.clone())?.pack(&mut block_bytes).unwrap();
        OptionalPayload::pack(&OptionalPayload::from(payload.clone()), &mut block_bytes)
            .map_err(|_| crate::Error::PackableError)?;

        let mut pow_digest = TritBuf::<T1B1Buf>::new();
        let target_zeros =
            (((block_bytes.len() + std::mem::size_of::<u64>()) as f64 * target_score).ln() / LN_3).ceil() as usize;

        if target_zeros > HASH_LENGTH {
            return Err(bee_pow::providers::miner::Error::InvalidPowScore(target_score, target_zeros).into());
        }

        let hash = Blake2b256::digest(&block_bytes);

        b1t6::encode::<T1B1Buf>(&hash).iter().for_each(|t| pow_digest.push(t));

        let mut nonce = 0;
        let mut hasher = CurlPBatchHasher::<T1B1Buf>::new(HASH_LENGTH);
        let mut buffers = Vec::<TritBuf<T1B1Buf>>::with_capacity(BATCH_SIZE);
        for _ in 0..BATCH_SIZE {
            let mut buffer = TritBuf::<T1B1Buf>::zeros(HASH_LENGTH);
            buffer[..pow_digest.len()].copy_from(&pow_digest);
            buffers.push(buffer);
        }
        let mining_start = instant::Instant::now();
        // counter to reduce amount of mining_start.elapsed() calls
        let mut counter = 0;
        loop {
            if counter % POW_ROUNDS_BEFORE_INTERVAL_CHECK == 0
                && mining_start.elapsed() > std::time::Duration::from_secs(tips_interval)
            {
                // update parents
                parent_blocks = client.get_tips().await?;
                break;
            }
            for (i, buffer) in buffers.iter_mut().enumerate() {
                let nonce_trits = b1t6::encode::<T1B1Buf>(&(nonce + i as u64).to_le_bytes());
                buffer[pow_digest.len()..pow_digest.len() + nonce_trits.len()].copy_from(&nonce_trits);
                hasher.add(buffer.clone());
            }
            for (i, hash) in hasher.hash().enumerate() {
                let trailing_zeros = hash.iter().rev().take_while(|t| *t == Btrit::Zero).count();
                if trailing_zeros >= target_zeros {
                    Box::new(nonce + i as u64).pack(&mut block_bytes).unwrap();
                    return Block::unpack_verified(&mut block_bytes.as_slice())
                        .map_err(|_| crate::Error::PackableError);
                }
            }
            nonce += BATCH_SIZE as u64;
            counter += 1;
        }
    }
}
