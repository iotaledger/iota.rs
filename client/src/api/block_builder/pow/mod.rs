// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! PoW functions

#[cfg(target_family = "wasm")]
pub mod wasm_miner;

pub mod miner;

use bee_block::{parent::Parents, payload::Payload, Block, BlockBuilder, BlockId};
use bee_pow::providers::{NonceProvider, NonceProviderBuilder};
use packable::PackableExt;
#[cfg(target_family = "wasm")]
use wasm_miner::SingleThreadedMiner;
#[cfg(not(target_family = "wasm"))]
use {crate::api::miner::ClientMiner, bee_pow::providers::miner::MinerCancel};

use crate::{Client, Error, Result};

/// Performs proof-of-work to construct a [`Block`].
pub fn do_pow<P: NonceProvider>(
    miner: P,
    min_pow_score: u32,
    payload: Option<Payload>,
    parent_blocks: Vec<BlockId>,
) -> Result<Block> {
    let mut block = BlockBuilder::<P>::new(Parents::new(parent_blocks)?);
    if let Some(p) = payload {
        block = block.with_payload(p);
    }
    block
        .with_nonce_provider(miner)
        .finish(min_pow_score)
        .map_err(Error::BlockError)
}

/// Calls the appropriate PoW function depending whether the compilation is for wasm or not.
pub async fn finish_pow(client: &Client, payload: Option<Payload>) -> Result<Block> {
    #[cfg(not(target_family = "wasm"))]
    let block = crate::api::pow::finish_multi_threaded_pow(client, payload).await?;
    #[cfg(target_family = "wasm")]
    let block = crate::api::pow::finish_single_threaded_pow(client, payload).await?;

    Ok(block)
}

/// Performs multi-threaded proof-of-work.
///
/// Always fetches new tips after each tips interval elapses.
#[cfg(not(target_family = "wasm"))]
async fn finish_multi_threaded_pow(client: &Client, payload: Option<Payload>) -> Result<Block> {
    let local_pow = client.get_local_pow();
    let pow_worker_count = client.pow_worker_count;
    let min_pow_score = client.get_min_pow_score()?;
    let tips_interval = client.get_tips_interval();
    loop {
        let cancel = MinerCancel::new();
        let cancel_2 = cancel.clone();
        let payload_ = payload.clone();
        let mut parent_blocks = client.get_tips().await?;
        parent_blocks.sort_unstable_by_key(PackableExt::pack_to_vec);
        parent_blocks.dedup();
        let time_thread = std::thread::spawn(move || Ok(pow_timeout(tips_interval, cancel)));
        let pow_thread = std::thread::spawn(move || {
            let mut client_miner = ClientMiner::builder().with_local_pow(local_pow).with_cancel(cancel_2);
            if let Some(worker_count) = pow_worker_count {
                client_miner = client_miner.with_worker_count(worker_count);
            }
            do_pow(client_miner.finish(), min_pow_score, payload_, parent_blocks)
                .map(|block| (block.nonce(), Some(block)))
        });

        let threads = vec![pow_thread, time_thread];
        for t in threads {
            match t.join().expect("failed to join threads.") {
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

// PoW timeout, if we reach this we will restart the PoW with new tips, so the final block will never be lazy.
#[cfg(not(target_family = "wasm"))]
fn pow_timeout(after_seconds: u64, cancel: MinerCancel) -> (u64, Option<Block>) {
    std::thread::sleep(std::time::Duration::from_secs(after_seconds));
    cancel.trigger();
    (0, None)
}

/// Single threaded proof-of-work for Wasm, which cannot generally spawn the native threads used
/// by the `ClientMiner`.
///
/// Always fetches new tips after each tips interval elapses.
#[cfg(target_family = "wasm")]
async fn finish_single_threaded_pow(client: &Client, payload: Option<Payload>) -> Result<Block> {
    let min_pow_score: u32 = client.get_min_pow_score()?;
    let tips_interval: u64 = client.get_tips_interval();
    let local_pow: bool = client.get_local_pow();
    let mut parent_blocks = client.get_tips().await?;
    loop {
        parent_blocks.sort_unstable_by_key(PackableExt::pack_to_vec);
        parent_blocks.dedup();

        let single_threaded_miner = SingleThreadedMiner::builder()
            .tips_interval_secs(tips_interval)
            .local_pow(local_pow)
            .finish();
        let block: Block = do_pow(
            single_threaded_miner,
            min_pow_score,
            payload.clone(),
            parent_blocks.clone(),
        )?;

        // The nonce defaults to 0 on errors (from the tips interval elapsing),
        // we need to re-run proof-of-work with new parents.
        if block.nonce() == 0 && local_pow {
            parent_blocks = client.get_tips().await?;
        } else {
            return Ok(block);
        }
    }
}
