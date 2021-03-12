// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::constant::{HASH_CHUNK_LEN, MAX_TRYTE_VALUE, TRITS82_BE_U32};
use crate::error::{Error, Result};
use crate::helper::{get_max_normalized_bundle_hash, get_the_max_tryte_values};
use crate::success;
use bee_crypto::ternary::{
    bigint::{binary_representation::U32Repr, endianness::BigEndian, I384, T242, T243},
    sponge::{Kerl, Sponge},
};
use bee_signing::ternary::wots::normalize;
use bee_ternary::{t3b1::T3B1Buf, Btrit, T1B1Buf, TritBuf};
use bee_transaction::bundled::TAG_TRIT_LEN;
use futures::future::abortable;
use std::{
    convert::TryFrom,
    sync::{Arc, Mutex},
};
use tokio::{runtime::Builder, sync::mpsc, task, time};

/// TODO: Remove this when they are explosed to public in bee_transaction
#[derive(Copy, Clone)]
pub struct Offset {
    pub start: usize,
    pub length: usize,
}

/// TODO: Remove this when they are explosed to public in bee_transaction
#[derive(Copy, Clone)]
pub struct Field {
    pub trit_offset: Offset,
    pub tryte_offset: Offset,
}

/// TODO: Remove this when they are explosed to public in bee_transaction
impl Field {
    pub fn byte_start(&self) -> usize {
        self.trit_offset.start / 5
    }

    pub fn byte_length(&self) -> usize {
        if self.trit_offset.length % 5 == 0 {
            self.trit_offset.length / 5
        } else {
            self.trit_offset.length / 5 + 1
        }
    }
}

#[derive(Debug)]
pub enum MinerEvent {
    MinedEssence(TritBuf<T1B1Buf>),
    Timeout,
}

#[derive(Debug)]
pub struct MinedCrackability {
    pub crackability: f64,
    pub mined_essence: Option<TritBuf<T1B1Buf>>,
    pub mined_iteration: usize,
}

#[derive(Debug)]
pub enum CrackabilityMinerEvent {
    MinedCrackability(MinedCrackability),
    Timeout(MinedCrackability),
}

/// TODO: Remove this when they are explosed to public in bee_transaction
const HASH_TRYTES_COUNT: usize = 81;
const RESERVED_NONCE_TRYTES_COUNT: usize = 42;

/// Builder for a miner.
pub struct MinerBuilder {
    /// Bundle hashes from previous spends.
    known_bundle_hashes: Option<Vec<TritBuf<T1B1Buf>>>,

    /// Obsolete tag offset from which to start mining. The miner will begin from this offset.
    /// Existing obsolete tags in the unsignedBundle should be discarded.
    offset: i64,

    /// The essences from transactions of the unsigned bundle for mining.
    essences_from_unsigned_bundle: Option<Vec<TritBuf<T1B1Buf>>>,

    /// Bundle security level (usually 2).
    security_level: usize,

    /// The number of bundle fragments that should not contain a 13 (starting from bundle fragment at index 0).
    num_13_free_fragments: Option<usize>,

    /// The number of  concurrent mining workers.
    worker_count: usize,

    /// The number of used core threads in the user’s device.
    core_thread_count: usize,

    /// The seconds for mining.
    mining_timeout: u64,
}

impl Default for MinerBuilder {
    fn default() -> Self {
        Self {
            known_bundle_hashes: None,
            offset: 0,
            essences_from_unsigned_bundle: None,
            security_level: 2,
            num_13_free_fragments: None,
            worker_count: 1,
            core_thread_count: 1,
            mining_timeout: 600,
        }
    }
}

pub struct Miner {
    /// Bundle hashes from previous spends.
    known_bundle_hashes: Vec<TritBuf<T1B1Buf>>,

    /// Obsolete tag offset from which to start mining. The miner will begin from this offset.
    /// Existing obsolete tags in the unsignedBundle should be discarded.
    offset: i64,

    /// The essences from transactions of the unsigned bundle for mining.
    essences_from_unsigned_bundle: Vec<TritBuf<T1B1Buf>>,

    /// Bundle security level (usually 2).
    security_level: usize,

    /// The number of bundle fragments that should not contain a 13 (starting from bundle fragment at index 0).
    num_13_free_fragments: usize,

    /// The number of  concurrent mining workers.
    worker_count: usize,

    /// The number of used core threads in the user’s device.
    core_thread_count: usize,

    /// The mining timeout.
    mining_timeout: u64,
}

impl MinerBuilder {
    /// Creates a new builder for a bundle miner.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the kown bundle hashes.
    pub fn with_known_bundle_hashes(mut self, known_bundle_hashes: Vec<TritBuf<T1B1Buf>>) -> Self {
        self.known_bundle_hashes = Some(known_bundle_hashes);
        self
    }

    /// Set the offset.
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    /// Set the essences from unsigned bundle.
    pub fn with_essences_from_unsigned_bundle(
        mut self,
        essences_from_unsigned_bundle: Vec<TritBuf<T1B1Buf>>,
    ) -> Self {
        self.essences_from_unsigned_bundle = Some(essences_from_unsigned_bundle);
        self
    }

    /// Set the security level.
    pub fn with_security_level(mut self, security_level: usize) -> Self {
        self.security_level = security_level;
        self
    }

    /// Set the num-13-free_fragments.
    pub fn with_num_13_free_fragments(mut self, num_13_free_fragments: usize) -> Self {
        self.num_13_free_fragments = Some(num_13_free_fragments);
        self
    }

    /// Set the worker count.
    pub fn with_worker_count(mut self, worker_count: usize) -> Self {
        self.worker_count = worker_count;
        self
    }

    /// Set the core-thread count.
    pub fn with_core_thread_count(mut self, core_thread_count: usize) -> Self {
        self.core_thread_count = core_thread_count;
        self
    }

    /// Set the mining timeout.
    pub fn with_mining_timeout(mut self, mining_timeout: u64) -> Self {
        self.mining_timeout = mining_timeout;
        self
    }

    /// Builds a bundler miner.
    pub fn finish(self) -> Result<Miner> {
        let miner = Miner {
            known_bundle_hashes: match self.known_bundle_hashes {
                Some(hashes) => hashes,
                None => return Err(Error::KownBundleHashesNotSet),
            },
            offset: self.offset,
            essences_from_unsigned_bundle: match self.essences_from_unsigned_bundle {
                Some(essences) => essences,
                None => return Err(Error::EssencesFromUnsignedBundleNotSet),
            },
            security_level: self.security_level,
            num_13_free_fragments: self
                .num_13_free_fragments
                .unwrap_or(self.security_level * HASH_CHUNK_LEN),
            worker_count: self.worker_count,
            core_thread_count: self.core_thread_count,
            mining_timeout: self.mining_timeout,
        };
        Ok(miner)
    }
}

/// Trait for defining the mining criteria.
pub trait StopMiningCriteria {
    /// Judgement function for the stop criterion.
    fn judge(
        &mut self,
        mined_hash: &TritBuf<T1B1Buf>,
        target_hash: &TritBuf<T1B1Buf>,
    ) -> Result<bool>;
}

/// Criteria of each tryte is less then the corresponding tryte in the max hash.
#[derive(Copy, Clone)]
pub struct LessThanMaxHash;

/// Criteria of each tryte equals to then the corresponding tryte in the target hash.
#[derive(Copy, Clone)]
pub struct EqualTargetHash;

pub(crate) struct CrackProbabilityLessThanThresholdBuilder {
    /// The target crack probability.
    target_crack_probability: f64,
    /// The mining will stop then the crack probability <= threshold.
    threshold: f64,
    /// The number of bundle fragments that should not contain a 13 (starting from bundle fragment at index 0).
    num_13_free_fragments: usize,
}

impl Default for CrackProbabilityLessThanThresholdBuilder {
    fn default() -> Self {
        Self {
            target_crack_probability: 1.0,
            threshold: 0.0,
            num_13_free_fragments: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct CrackProbabilityLessThanThreshold {
    /// The mined crack probability
    mined_crack_probability: f64,
    /// The mining will stop then the crack probability <= threshold.
    threshold: f64,
    /// The number of bundle fragments that should not contain a 13 (starting from bundle fragment at index 0).
    num_13_free_fragments: usize,
}

impl CrackProbabilityLessThanThresholdBuilder {
    /// Creates a new builder for a bundle miner.
    pub(crate) fn new() -> Self {
        Self::default()
    }
    /// Sets the target crack probability.
    pub(crate) fn with_target_crack_probability(mut self, target_crack_probability: f64) -> Self {
        self.target_crack_probability = target_crack_probability;
        self
    }
    /// Sets the threshold of the cracker.
    pub(crate) fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }
    /// Sets the num-13-free_fragments of the cracker.
    pub(crate) fn with_num_13_free_fragments(mut self, num_13_free_fragments: usize) -> Self {
        self.num_13_free_fragments = num_13_free_fragments;
        self
    }
    /// Builds a CrackProbabilityLessThanThreshold structure.
    pub(crate) fn finish(mut self) -> CrackProbabilityLessThanThreshold {
        if self.target_crack_probability >= self.threshold {
            self.threshold = 0.99_f64 * self.target_crack_probability;
        }
        CrackProbabilityLessThanThreshold {
            mined_crack_probability: std::f64::MAX, // Set the init crackability to be max value.
            threshold: self.threshold,
            num_13_free_fragments: self.num_13_free_fragments,
        }
    }
}

/// The constant of `LessThanMaxHash` criterion.
pub const LESS_THAN_MAX_HASH: LessThanMaxHash = LessThanMaxHash;

/// The constant of `EqualTargetHash` criterion.
pub const EQUAL_TRAGET_HASH: EqualTargetHash = EqualTargetHash;

/// For `LessThanMaxHash` criterion, each tryte in mined hash should be smaller than that in the max hash.
impl StopMiningCriteria for LessThanMaxHash {
    fn judge(
        &mut self,
        mined_hash: &TritBuf<T1B1Buf>,
        target_hash: &TritBuf<T1B1Buf>,
    ) -> Result<bool> {
        // Get the i8 slices from the mined bundle hash
        let mined_bundle_hash_i8 = TritBuf::<T3B1Buf>::from_i8s(mined_hash.as_i8_slice())?
            .as_i8_slice()
            .to_vec();

        // Get the i8 slices from the max bundle hash
        let max_bundle_hash_i8 = TritBuf::<T3B1Buf>::from_i8s(target_hash.as_i8_slice())?
            .as_i8_slice()
            .to_vec();

        // Check whether each tryte of mined hash is smaller than the corresponding tryte in the max hash
        let larger_than_max_count: i8 = max_bundle_hash_i8
            .iter()
            .zip(&mined_bundle_hash_i8[..max_bundle_hash_i8.len()])
            .map(|(&x, &y)| if x < y { 1 } else { 0 })
            .collect::<Vec<i8>>()
            .into_iter()
            .sum();

        // Return true if all of the trytes in the mined hash are smaller than those in the max hash
        Ok(larger_than_max_count == 0)
    }
}

/// For `EqualTargetHash` criterion, each tryte in mined hash should equal to that in the max hash.
impl StopMiningCriteria for EqualTargetHash {
    fn judge(
        &mut self,
        mined_hash: &TritBuf<T1B1Buf>,
        target_hash: &TritBuf<T1B1Buf>,
    ) -> Result<bool> {
        Ok(mined_hash == target_hash)
    }
}

/// For `CrackProbabilityLessThanThreshold` criterion, mining stops when mined_probability <= threshold
impl StopMiningCriteria for CrackProbabilityLessThanThreshold {
    fn judge(
        &mut self,
        mined_hash: &TritBuf<T1B1Buf>,
        target_hash: &TritBuf<T1B1Buf>,
    ) -> Result<bool> {
        let mined_hash_trit_t3b1 = TritBuf::<T3B1Buf>::from_i8s(mined_hash.as_i8_slice())?;
        let mined_hash_trit_t3b1_i8 = mined_hash_trit_t3b1.as_i8_slice();

        let target_hash_trit_t3b1 = TritBuf::<T3B1Buf>::from_i8s(target_hash.as_i8_slice())?;
        let target_hash_trit_t3b1_i8 = target_hash_trit_t3b1.as_i8_slice();

        // We are only interested in hashes not containing 'M'
        if mined_hash_trit_t3b1_i8[..self.num_13_free_fragments]
            .iter()
            .any(|&i| i == MAX_TRYTE_VALUE)
        {
            return Ok(false);
        }

        // Calculate the max hash
        let max_hash = get_the_max_tryte_values(
            mined_hash_trit_t3b1_i8.to_vec(),
            target_hash_trit_t3b1_i8.to_vec(),
        );

        // Terminate early if we found an optimal bundle
        // Return true when each tryte of max hash <= the corresponding tryte of the target hash
        if !max_hash[..self.num_13_free_fragments]
            .iter()
            .zip(&target_hash_trit_t3b1_i8[..self.num_13_free_fragments])
            .map(|(&x, &y)| x <= y)
            .any(|x| x)
        {
            return Ok(true);
        }
        let mut p = 1.0_f64;
        for i in 0..(self.num_13_free_fragments / HASH_CHUNK_LEN) {
            p *=
                success(&max_hash[i * HASH_CHUNK_LEN..i * HASH_CHUNK_LEN + HASH_CHUNK_LEN].to_vec())
        }
        if self.mined_crack_probability > p {
            self.mined_crack_probability = p;
        }
        Ok(p <= self.threshold)
    }
}

impl Miner {
    /// The minier which returns the best crackability and mined iteration.
    pub async fn run(
        &mut self,
        target_crack_probability: Option<f64>,
        threshold: Option<f64>,
    ) -> Result<CrackabilityMinerEvent> {
        let target_hash =
            get_max_normalized_bundle_hash(&self.known_bundle_hashes, self.security_level);
        let criterion = CrackProbabilityLessThanThresholdBuilder::new()
            .with_target_crack_probability(target_crack_probability.unwrap_or(0.0))
            .with_threshold(threshold.unwrap_or(0.0))
            .with_num_13_free_fragments(self.num_13_free_fragments)
            .finish();

        let (tx, mut rx) = mpsc::channel(self.worker_count);
        let counters = Arc::new(Mutex::new(vec![0; self.worker_count]));
        let crackability = Arc::new(Mutex::new(std::f64::MAX));
        // Use the dummy essence and update in the mining_worker function
        let best_essence = Arc::new(Mutex::new(self.essences_from_unsigned_bundle[0].clone()));
        let worker_count = self.worker_count;
        let offset = self.offset;
        let essences_from_unsigned_bundle = self.essences_from_unsigned_bundle[..].to_vec();
        let mining_timeout = self.mining_timeout;
        let runtime = Builder::new_multi_thread()
            .worker_threads(self.core_thread_count)
            .thread_name("bundle-miner")
            .thread_stack_size(3 * 1024 * 1024) // TODO: configurable by user
            .enable_time()
            .build()?;
        let mut abort_handles = Vec::new();
        let result = runtime
            .spawn(async move {
                for i in 0..worker_count {
                    let tx_cloned = tx.clone();
                    let (abortable_worker, abort_handle) = abortable(mining_worker(
                        offset,
                        i,
                        essences_from_unsigned_bundle.to_vec(),
                        target_hash.clone(),
                        Arc::clone(&counters),
                        Arc::clone(&crackability),
                        Arc::clone(&best_essence),
                        criterion,
                    ));
                    tokio::spawn(async move {
                        if let Ok(mined_essence) = abortable_worker.await {
                            tx_cloned
                                .send(MinerEvent::MinedEssence(
                                    mined_essence.expect("Cannot get the mined essence"),
                                ))
                                .await
                                .expect("Cannot send the MinedEssence event");
                        }
                    });
                    abort_handles.push(abort_handle);
                }
                let (abortable_worker, abort_handle) = abortable(timeout_worker(mining_timeout));
                let tx_cloned = tx.clone();
                tokio::spawn(async move {
                    if abortable_worker.await.is_ok() {
                        tx_cloned
                            .send(MinerEvent::Timeout)
                            .await
                            .expect("Cannot send Timeout event");
                    }
                });
                abort_handles.push(abort_handle);
                if let Some(event) = rx.recv().await {
                    let best_crackability =
                        *(crackability.lock().expect("Cannot get the crackability"));
                    let mined_iteration = *(counters.lock().expect("Cannot get the counters"))
                        .iter()
                        .max()
                        .expect("Cannot get the maximum iteration from counters");
                    let mined_best_essence = best_essence
                        .lock()
                        .expect("Cannot get the best essence")
                        .clone();
                    match event {
                        MinerEvent::MinedEssence(essence) => {
                            for i in abort_handles {
                                i.abort();
                            }
                            CrackabilityMinerEvent::MinedCrackability(MinedCrackability {
                                crackability: best_crackability,
                                mined_essence: Some(essence),
                                mined_iteration,
                            })
                        }
                        MinerEvent::Timeout => {
                            for i in abort_handles {
                                i.abort();
                            }
                            CrackabilityMinerEvent::Timeout(MinedCrackability {
                                crackability: best_crackability,
                                mined_essence: Some(mined_best_essence),
                                mined_iteration,
                            })
                        }
                    }
                } else {
                    unreachable!();
                }
            })
            .await
            .unwrap();
        runtime.shutdown_background();
        Ok(result)
    }

    /// Run the bundle miner with non-crack-probability stop criteria
    pub async fn run_with_with_non_crack_probability_stop_criteria(
        &mut self,
        target_hash: TritBuf<T1B1Buf>,
        criterion: impl StopMiningCriteria + std::marker::Send + 'static + Copy,
    ) -> MinerEvent {
        let (tx, mut rx) = mpsc::channel(self.worker_count);
        let runtime = Builder::new_multi_thread()
            .worker_threads(self.core_thread_count)
            .thread_name("miner")
            .thread_stack_size(3 * 1024 * 1024) // TODO: configurable by user
            .enable_time()
            .build()
            .unwrap();
        let mut abort_handles = Vec::new();
        let worker_count = self.worker_count;
        let essences_from_unsigned_bundle = self.essences_from_unsigned_bundle[..].to_vec();
        let mining_timeout = self.mining_timeout;
        let res = runtime
            .spawn(async move {
                for i in 0..worker_count {
                    let tx_cloned = tx.clone();
                    let (abortable_worker, abort_handle) =
                        abortable(mining_worker_with_non_crack_probability_stop_criteria(
                            0,
                            i,
                            essences_from_unsigned_bundle.to_vec(),
                            target_hash.clone(),
                            criterion,
                        ));
                    tokio::spawn(async move {
                        if let Ok(mined_essence) = abortable_worker.await {
                            tx_cloned
                                .send(MinerEvent::MinedEssence(mined_essence))
                                .await
                                .unwrap();
                        }
                    });
                    abort_handles.push(abort_handle);
                }
                let (abortable_worker, abort_handle) = abortable(timeout_worker(mining_timeout));
                let tx_cloned = tx.clone();
                tokio::spawn(async move {
                    if abortable_worker.await.is_ok() {
                        tx_cloned.send(MinerEvent::Timeout).await.unwrap();
                    }
                });
                abort_handles.push(abort_handle);
                if let Some(event) = rx.recv().await {
                    match event {
                        MinerEvent::MinedEssence(essence) => {
                            for i in abort_handles {
                                i.abort();
                            }
                            MinerEvent::MinedEssence(essence)
                        }
                        MinerEvent::Timeout => {
                            for i in abort_handles {
                                i.abort();
                            }
                            MinerEvent::Timeout
                        }
                    }
                } else {
                    unreachable!();
                }
            })
            .await
            .unwrap();
        runtime.shutdown_background();
        res
    }
}

/// The timeout worker to terminate the runtime in seconds
pub async fn timeout_worker(seconds: u64) {
    time::sleep(time::Duration::from_secs(seconds)).await;
}

/// The mining worker, stop when timeout or the criterion is met
/// Return the mined essence for the last transaction
pub async fn mining_worker(
    increment: i64,
    worker_id: usize,
    mut essences: Vec<TritBuf<T1B1Buf>>,
    target_hash: TritBuf<T1B1Buf>,
    counters: Arc<Mutex<Vec<usize>>>,
    crackability: Arc<Mutex<f64>>,
    best_essence: Arc<Mutex<TritBuf<T1B1Buf>>>,
    mut criterion: CrackProbabilityLessThanThreshold,
) -> Result<TritBuf<T1B1Buf>> {
    let mut last_essence: TritBuf<T1B1Buf> = match essences.pop() {
        Some(essence) => essence,
        None => return Err(Error::EmptyEssenceToMine),
    };
    let kerl = prepare_keccak_384(&essences).await;
    let obselete_tag = create_obsolete_tag(increment, worker_id as i32).await;
    last_essence = update_essense_with_new_obsolete_tag(last_essence, &obselete_tag).await;

    // Note that we check the last essence with `zero` incresement first
    // While in the go-lang version the first checked essence hash `one` incresement
    let mut mined_hash = last_essence.clone();
    // Update the counter
    {
        let mut num = match counters.lock() {
            Ok(num) => num,
            Err(_) => return Err(Error::CounterPoisonError),
        };
        (*num)[worker_id] += 1;
    }
    while !criterion.judge(&mined_hash, &target_hash)? {
        last_essence = increase_essense(last_essence).await?;
        mined_hash = absorb_and_get_normalized_bundle_hash(kerl.clone(), &last_essence).await;
        task::yield_now().await;
        // Update the counter
        {
            let mut num = match counters.lock() {
                Ok(num) => num,
                Err(_) => return Err(Error::CounterPoisonError),
            };
            (*num)[worker_id] += 1;
        }
        // Update current best crackability
        {
            let mut current_best_crackability = match crackability.lock() {
                Ok(crackability) => crackability,
                Err(_) => return Err(Error::CrackabilityPoisonError),
            };
            if criterion.mined_crack_probability < *current_best_crackability {
                *current_best_crackability = criterion.mined_crack_probability;
                let mut current_best_essence = best_essence.lock().unwrap();
                *current_best_essence = last_essence.clone();
            }
        }
    }
    // Finalize the best crackability
    {
        let mut current_best_crackability = crackability.lock().unwrap();
        if criterion.mined_crack_probability < *current_best_crackability {
            *current_best_crackability = criterion.mined_crack_probability;
            let mut current_best_essence = best_essence.lock().unwrap();
            *current_best_essence = last_essence.clone();
        }
    }
    Ok(last_essence)
}

/// The mining worker, stop when timeout or the criterion is met
/// Return the mined essence for the last transaction
pub async fn mining_worker_with_non_crack_probability_stop_criteria(
    increment: i64,
    worker_id: usize,
    mut essences: Vec<TritBuf<T1B1Buf>>,
    target_hash: TritBuf<T1B1Buf>,
    mut criterion: impl StopMiningCriteria,
) -> TritBuf<T1B1Buf> {
    let mut last_essence: TritBuf<T1B1Buf> = essences.pop().unwrap();
    let kerl = prepare_keccak_384(&essences).await;
    let obselete_tag = create_obsolete_tag(increment, worker_id as i32).await;
    last_essence = update_essense_with_new_obsolete_tag(last_essence, &obselete_tag).await;

    // Note that we check the last essence with `zero` incresement first
    // While in the go-lang version the first checked essence hash `one` incresement
    let mut mined_hash = last_essence.clone();
    while !criterion.judge(&mined_hash, &target_hash).unwrap() {
        last_essence = increase_essense(last_essence).await.unwrap();
        task::yield_now().await;
        mined_hash = absorb_and_get_normalized_bundle_hash(kerl.clone(), &last_essence).await;
        task::yield_now().await;
    }
    last_essence
}

/// Absorb the input essences and return the Kerl
pub async fn prepare_keccak_384(essences: &[TritBuf<T1B1Buf>]) -> Kerl {
    let mut kerl = Kerl::new();
    for essence in essences.iter() {
        async { kerl.absorb(essence.as_slice()).unwrap() }.await;
        task::yield_now().await;
    }
    kerl
}

/// Use Kerl to absorbe the last essence, sqeeze, and output the normalized hash
pub async fn absorb_and_get_normalized_bundle_hash(
    mut kerl: Kerl,
    last_essence: &TritBuf<T1B1Buf>,
) -> TritBuf<T1B1Buf> {
    async { kerl.absorb(last_essence.as_slice()).unwrap() }.await;
    task::yield_now().await;
    // Return hash
    async { normalize(&kerl.squeeze().unwrap()).unwrap() }.await
}

/// Increase the essence by 3^81, so the obselete is increased by 1
pub async fn increase_essense(essence: TritBuf<T1B1Buf>) -> Result<TritBuf<T1B1Buf>> {
    let mut essence_i384 = async {
        I384::<BigEndian, U32Repr>::try_from(T243::<Btrit>::new(essence).into_t242()).unwrap()
    }
    .await;
    async { essence_i384.add_inplace(TRITS82_BE_U32) }.await;
    // Return essence
    Ok(async {
        T242::<Btrit>::try_from(essence_i384)
            .unwrap()
            .into_t243()
            .into_inner()
    }
    .await)
}

/// Cast TritBuf to String for verification usage and ease of observation
pub async fn trit_buf_to_string(trit_buf: &TritBuf<T1B1Buf>) -> Result<String> {
    async {
        Ok(TritBuf::<T3B1Buf>::from_i8s(trit_buf.as_i8_slice())?
            .as_trytes()
            .iter()
            .map(|t| char::from(*t))
            .collect::<String>())
    }
    .await
}

/// Replace the obselete tag in the essence with a new one
pub async fn update_essense_with_new_obsolete_tag(
    mut essence: TritBuf<T1B1Buf>,
    obselete_tag: &TritBuf<T1B1Buf>,
) -> TritBuf<T1B1Buf> {
    let obselete_tag_i8s = obselete_tag.as_i8_slice();
    let essence_i8s = unsafe { essence.as_i8_slice_mut() };
    essence_i8s[TAG_TRIT_LEN..TAG_TRIT_LEN * 2].copy_from_slice(obselete_tag_i8s);
    async { TritBuf::<T1B1Buf>::from_i8s(essence_i8s).unwrap() }.await
}

/// Create the obsolete tag by the increment (the 43th-81th trits) and worker_id (first 42 trits)
pub async fn create_obsolete_tag(increment: i64, worker_id: i32) -> TritBuf<T1B1Buf> {
    let mut zero_tritbuf = TritBuf::<T1B1Buf>::zeros(TAG_TRIT_LEN);
    let reserved_nonce_tritbuf = async { TritBuf::<T1B1Buf>::from(increment) }.await;
    let reserved_nonce_trits = async { reserved_nonce_tritbuf.as_i8_slice() }.await;
    let other_essence_tritbuf = async { TritBuf::<T1B1Buf>::from(worker_id) }.await;
    let other_essence_trits = async { other_essence_tritbuf.as_i8_slice() }.await;
    let output = async { unsafe { zero_tritbuf.as_i8_slice_mut() } }.await;
    let mut reserved_nonce_trits_len = async { reserved_nonce_trits.len() }.await;
    if reserved_nonce_trits_len > RESERVED_NONCE_TRYTES_COUNT {
        reserved_nonce_trits_len = RESERVED_NONCE_TRYTES_COUNT;
    }
    async { output[..reserved_nonce_trits_len].clone_from_slice(reserved_nonce_trits) }.await;
    let mut other_trits_len = RESERVED_NONCE_TRYTES_COUNT + other_essence_trits.len();
    if other_trits_len > HASH_TRYTES_COUNT {
        other_trits_len = HASH_TRYTES_COUNT;
    }
    async {
        output[RESERVED_NONCE_TRYTES_COUNT..other_trits_len].clone_from_slice(other_essence_trits)
    }
    .await;
    async { TritBuf::<T1B1Buf>::from_i8s(output).unwrap() }.await
}
