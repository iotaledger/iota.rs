//! Quorum module is a extension to iota client instance which can make sure result from API calls are verified by node
//! pool and guarantee to staisfy minimum quorum threshold.

use crate::error::*;
use crate::response::*;
use crate::Client;

use iota_bundle_preview::{Address, Hash, TransactionField};
use iota_conversion::Trinary;
use iota_ternary_preview::TryteBuf;

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use once_cell::sync::Lazy;
use reqwest::Url;

macro_rules! get_synced_nodes {
    () => {{
        let time = Quorum::get().time.clone();
        let mut time = time.write().expect("Timestamp read poinsened");
        if time.elapsed() >= Duration::from_secs(300) {
            refresh_synced_nodes().await?;
            *time = Instant::now();
        }
        Quorum::get()
            .pool
            .clone()
            .read()
            .expect("Node pool read poinsened")
    }};
}

/// An instance of the client using IRI URI
#[derive(Debug)]
pub struct Quorum {
    /// Synced node pool of IOTA nodes for quorum
    pub(crate) pool: Arc<RwLock<HashSet<Url>>>,
    /// Quorum threshold
    pub(crate) threshold: AtomicU8,
    /// Minimum nodes quired to satisfy quorum threshold
    pub(crate) min: AtomicUsize,
    /// Timestamp of last syncing process
    pub(crate) time: Arc<RwLock<Instant>>,
}

impl Quorum {
    /// Get the quorum instance. It will init the instance if it's not created yet.
    pub fn get() -> &'static Quorum {
        static QUORUM: Lazy<Quorum> = Lazy::new(|| Quorum {
            pool: Arc::new(RwLock::new(HashSet::new())),
            threshold: AtomicU8::new(66),
            min: AtomicUsize::new(0),
            time: Arc::new(RwLock::new(Instant::now())),
        });

        &QUORUM
    }
}

/// Refresh synced nodes in quorum. This will replace whole set in the quorum pool.
pub async fn refresh_synced_nodes() -> Result<()> {
    let quorum = Quorum::get();
    let body = json!( {
        "command": "getNodeInfo",
    });
    let mut result = HashMap::new();

    for ref_node in Client::get()
        .pool
        .clone()
        .read()
        .expect("Node pool read poinsened")
        .iter()
    {
        let node = ref_node.clone();
        let hash: GetNodeInfoResponse = response!(body, node);
        let hash = Hash::from_inner_unchecked(
            // TODO missing impl error on Hash
            TryteBuf::try_from_str(&hash.latest_solid_subtangle_milestone)
                .unwrap()
                .as_trits()
                .encode(),
        );
        // TODO Better scope
        let set = result.entry(hash).or_insert(HashSet::new());
        set.insert(ref_node.clone());
    }

    let pool = quorum.pool.clone();
    let mut set = pool.write().expect("Node pool write poisened");
    *set = result
        .into_iter()
        .max_by_key(|v| v.1.len())
        .ok_or(Error::QuorumError)?
        .1;

    let val = set.len() * quorum.threshold.load(Ordering::Acquire) as usize / 100;
    quorum.min.store(val, Ordering::Release);

    Ok(())
}

/// Gets the confirmed balance of an address.
/// If the tips parameter is missing, the returned balance is correct as of the latest confirmed milestone.
/// This endpoint returns data only if the node is synchronized.
/// # Parameters
/// * [`addresses`] - Address for which to get the balance (do not include the checksum)
/// * [`threshold`] - (Optional) Confirmation threshold between 0 and 100, default is 100.
/// * [`tips`] - (Optional) Tips whose history of transactions to traverse to find the balance
///
/// [`addresses`]: struct.GetBalancesBuilder.html#method.addresses
/// [`threshold`]: struct.GetBalancesBuilder.html#method.threshold
/// [`tips`]: struct.GetBalancesBuilder.html#method.tips
pub fn get_balances() -> GetBalancesBuilder {
    GetBalancesBuilder::new()
}

/// Builder to construct getBalances API
#[derive(Debug)]
pub struct GetBalancesBuilder {
    addresses: Vec<String>,
    tips: Option<Vec<String>>,
}

impl GetBalancesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            addresses: Default::default(),
            tips: Default::default(),
        }
    }

    /// Add address for which to get the balance (do not include the checksum)
    pub fn addresses(mut self, addresses: &[Address]) -> Self {
        self.addresses = addresses
            .iter()
            .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
            .collect();
        self
    }

    /// Add tips whose history of transactions to traverse to find the balance
    pub fn tips(mut self, tips: &[Hash]) -> Self {
        self.tips = Some(
            tips.iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
        );
        self
    }

    /// Send getBalances request
    pub async fn send(self) -> Result<GetBalancesResponse> {
        let quorum = Quorum::get();
        let mut body = json!({
            "command": "getBalances",
            "addresses": self.addresses,
        });

        if let Some(reference) = self.tips {
            body["tips"] = json!(reference);
        }

        let mut result = HashMap::new();
        for node in get_synced_nodes!().iter() {
            let node = node.clone();
            let res: GetBalancesResponseBuilder = response!(body, node);
            let res = res.build().await?;
            let counters = result.entry(res).or_insert(0);
            *counters += 1;
        }

        let res = result
            .into_iter()
            .max_by_key(|v| v.1)
            .ok_or(Error::QuorumError)?;

        if res.1 >= quorum.min.load(Ordering::Acquire) {
            Ok(res.0)
        } else {
            Err(Error::QuorumThreshold)
        }
    }
}

/// Gets the inclusion states of a set of transactions.
/// This endpoint determines if a transaction is confirmed by the network (referenced by a valid milestone).
/// You can search for multiple tips (and thus, milestones) to get past inclusion states of transactions.
/// This endpoint returns data only if the node is synchronized.
/// # Parameters
/// * [`transactions`] - List of transaction hashes for which you want to get the inclusion state
/// * [`tips`] - (Optional) List of tip transaction hashes (including milestones) you want to search for
///
/// [`transactions`]: ../core/struct.GetInclusionStatesBuilder.html#method.transactions
/// [`tips`]: ../core/struct.GetInclusionStatesBuilder.html#method.tips
pub fn get_inclusion_states() -> GetInclusionStatesBuilder {
    GetInclusionStatesBuilder::new()
}

/// Builder to construct getInclusionStates API
#[derive(Debug)]
pub struct GetInclusionStatesBuilder {
    transactions: Vec<String>,
}

impl GetInclusionStatesBuilder {
    pub(crate) fn new() -> Self {
        Self {
            transactions: Default::default(),
        }
    }

    /// Add list of transaction hashes for which you want to get the inclusion state
    pub fn transactions(mut self, transactions: &[Hash]) -> Self {
        self.transactions = transactions
            .iter()
            .map(|h| h.as_bytes().trytes().unwrap())
            .collect();
        self
    }

    /// Send getInclusionStates request
    pub async fn send(self) -> Result<GetInclusionStatesResponse> {
        let quorum = Quorum::get();
        let body = json!({
            "command": "getInclusionStates",
            "transactions": self.transactions,
        });

        let mut result = HashMap::new();
        for node in get_synced_nodes!().iter() {
            let node = node.clone();
            let res: GetInclusionStatesResponseBuilder = response!(body, node);
            let res = res.build().await?;
            let counters = result.entry(res).or_insert(0);
            *counters += 1;
        }

        let res = result
            .into_iter()
            .max_by_key(|v| v.1)
            .ok_or(Error::QuorumError)?;

        if res.1 >= quorum.min.load(Ordering::Acquire) {
            Ok(res.0)
        } else {
            Err(Error::QuorumThreshold)
        }
    }
}

/// Fetches inclusion states of the given transactions by calling GetInclusionStates
/// using the latest solid subtangle milestone from GetNodeInfo.
///
/// # Parameters
/// * [`transactions`] - List of transaction hashes for which you want to get the inclusion state
pub async fn get_latest_inclusion(transactions: &[Hash]) -> Result<Vec<bool>> {
    let states = get_inclusion_states()
        .transactions(transactions)
        .send()
        .await?
        .states;
    Ok(states)
}

/// Gets latest solid subtangle milestone.
pub async fn get_latest_solid_subtangle_milestone() -> Result<Hash> {
    let quorum = Quorum::get();
    let body = json!( {
        "command": "getNodeInfo",
    });

    let mut result = HashMap::new();
    for node in get_synced_nodes!().iter() {
        let node = node.clone();
        let hash: GetNodeInfoResponse = response!(body, node);
        let hash = Hash::from_inner_unchecked(
            // TODO missing impl error on Hash
            TryteBuf::try_from_str(&hash.latest_solid_subtangle_milestone)
                .unwrap()
                .as_trits()
                .encode(),
        );
        let counters = result.entry(hash).or_insert(0);
        *counters += 1;
    }

    let res = result
        .into_iter()
        .max_by_key(|v| v.1)
        .ok_or(Error::QuorumError)?;

    if res.1 >= quorum.min.load(Ordering::Acquire) {
        Ok(res.0)
    } else {
        Err(Error::QuorumThreshold)
    }
}

/// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
/// If an address has a pending transaction, it's also considered 'spent'.
/// # Parameters
/// * `address` - addresses to check (do not include the checksum)
pub async fn were_addresses_spent_from(
    addresses: &[Address],
) -> Result<WereAddressesSpentFromResponse> {
    let quorum = Quorum::get();
    let addresses: Vec<String> = addresses
        .iter()
        .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
        .collect();
    let body = json!({
        "command": "wereAddressesSpentFrom",
        "addresses": addresses,
    });

    let mut result = HashMap::new();
    for node in get_synced_nodes!().iter() {
        let node = node.clone();
        let res: WereAddressesSpentFromResponseBuilder = response!(body, node);
        let res = res.build().await?;
        let counters = result.entry(res).or_insert(0);
        *counters += 1;
    }

    let res = result
        .into_iter()
        .max_by_key(|v| v.1)
        .ok_or(Error::QuorumError)?;

    if res.1 >= quorum.min.load(Ordering::Acquire) {
        Ok(res.0)
    } else {
        Err(Error::QuorumThreshold)
    }
}
