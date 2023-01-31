// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! PoW functions.

#[cfg(not(target_family = "wasm"))]
use iota_pow::miner::{Miner, MinerBuilder, MinerCancel};
#[cfg(target_family = "wasm")]
use iota_pow::wasm_miner::{SingleThreadedMiner, SingleThreadedMinerBuilder};
use iota_types::block::{parent::Parents, payload::Payload, Block, BlockBuilder, Error as BlockError};

use crate::{Client, Error, Result};

impl Client {
    /// Finishes the block with local PoW if needed.
    /// Without local PoW, it will finish the block with a 0 nonce.
    pub async fn finish_block_builder(&self, parents: Option<Parents>, payload: Option<Payload>) -> Result<Block> {
        if self.get_local_pow() {
            self.finish_pow(parents, payload).await
        } else {
            // Finish block without doing PoW.
            let parents = match parents {
                Some(parents) => parents,
                None => Parents::new(self.get_tips().await?)?,
            };
            let mut block_builder = BlockBuilder::new(parents);

            if let Some(p) = payload {
                block_builder = block_builder.with_payload(p);
            }

            Ok(block_builder.finish()?)
        }
    }

    /// Calls the appropriate PoW function depending whether the compilation is for wasm or not.
    pub async fn finish_pow(&self, parents: Option<Parents>, payload: Option<Payload>) -> Result<Block> {
        #[cfg(not(target_family = "wasm"))]
        let block = self.finish_multi_threaded_pow(parents, payload).await?;
        #[cfg(target_family = "wasm")]
        let block = self.finish_single_threaded_pow(parents, payload).await?;

        Ok(block)
    }

    /// Performs multi-threaded proof-of-work.
    ///
    /// Always fetches new tips after each tips interval elapses if no parents are provided.
    #[cfg(not(target_family = "wasm"))]
    async fn finish_multi_threaded_pow(&self, parents: Option<Parents>, payload: Option<Payload>) -> Result<Block> {
        let pow_worker_count = self.pow_worker_count;
        let min_pow_score = self.get_min_pow_score().await?;
        let tips_interval = self.get_tips_interval();

        loop {
            let cancel = MinerCancel::new();
            let cancel_2 = cancel.clone();
            let payload_ = payload.clone();
            let parents = match &parents {
                Some(parents) => parents.clone(),
                None => Parents::new(self.get_tips().await?)?,
            };
            let time_thread = std::thread::spawn(move || Ok(pow_timeout(tips_interval, cancel)));
            let pow_thread = std::thread::spawn(move || {
                let mut client_miner = MinerBuilder::new().with_cancel(cancel_2);
                if let Some(worker_count) = pow_worker_count {
                    client_miner = client_miner.with_num_workers(worker_count);
                }
                do_pow(client_miner.finish(), min_pow_score, payload_, parents).map(Some)
            });

            let threads = vec![pow_thread, time_thread];

            for t in threads {
                match t.join().expect("failed to join threads.") {
                    Ok(block) => {
                        if let Some(block) = block {
                            return Ok(block);
                        }
                    }
                    Err(Error::Block(BlockError::NonceNotFound)) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
    }

    /// Single threaded proof-of-work for Wasm, which cannot generally spawn the native threads used
    /// by the `ClientMiner`.
    ///
    /// Fetches new tips after each tips interval elapses if no parents are provided.
    #[cfg(target_family = "wasm")]
    async fn finish_single_threaded_pow(&self, parents: Option<Parents>, payload: Option<Payload>) -> Result<Block> {
        let min_pow_score: u32 = self.get_min_pow_score().await?;
        let tips_interval: u64 = self.get_tips_interval();

        loop {
            let parents = match &parents {
                Some(parents) => parents.clone(),
                None => Parents::new(self.get_tips().await?)?,
            };

            let single_threaded_miner = SingleThreadedMinerBuilder::new()
                .with_timeout_in_seconds(tips_interval)
                .finish();

            match do_pow(single_threaded_miner, min_pow_score, payload.clone(), parents) {
                Ok(block) => {
                    return Ok(block);
                }
                Err(Error::Block(BlockError::NonceNotFound)) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

/// Performs proof-of-work to construct a [`Block`].
fn do_pow(
    #[cfg(not(target_family = "wasm"))] miner: Miner,
    #[cfg(target_family = "wasm")] miner: SingleThreadedMiner,
    min_pow_score: u32,
    payload: Option<Payload>,
    parents: Parents,
) -> Result<Block> {
    let mut block = BlockBuilder::new(parents);

    if let Some(p) = payload {
        block = block.with_payload(p);
    }

    Ok(block.finish_nonce(|bytes| miner.nonce(bytes, min_pow_score))?)
}

// PoW timeout, if we reach this we will restart the PoW with new tips, so the final block will never be lazy.
#[cfg(not(target_family = "wasm"))]
fn pow_timeout(after_seconds: u64, cancel: MinerCancel) -> Option<Block> {
    std::thread::sleep(std::time::Duration::from_secs(after_seconds));

    cancel.trigger();

    None
}
