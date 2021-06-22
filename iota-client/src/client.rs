//! The Client module to connect through IRI with API usages
use crate::core::*;
use crate::error::*;
use crate::extended::*;
use crate::response::*;
use crate::util::tx_trytes;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bee_ternary::{T1B1Buf, T3B1Buf, TryteBuf};
use bee_transaction::bundled::{
    Address, BundledTransaction as Transaction, BundledTransactionField,
};
use bee_transaction::Vertex;
use crypto::hashes::ternary::Hash;
use crypto::keys::ternary::seed::Seed;
use tokio::sync::RwLock;

pub(crate) const REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(20);

macro_rules! response {
    ($self:ident, $body:ident) => {
        ureq::AgentBuilder::new()
            .timeout_read($crate::client::REQUEST_TIMEOUT)
            .timeout_write($crate::client::REQUEST_TIMEOUT)
            .build()
            .post(&$self.get_node().await?)
            .set("Content-Type", "application/json")
            .set("X-IOTA-API-Version", "1")
            .send_json($body)?
            .into_json()?
    };
    ($self:ident, $body:ident, $node:ident) => {
        ureq::AgentBuilder::new()
            .timeout_read($crate::client::REQUEST_TIMEOUT)
            .timeout_write($crate::client::REQUEST_TIMEOUT)
            .build()
            .post($node)
            .set("Content-Type", "application/json")
            .set("X-IOTA-API-Version", "1")
            .send_json($body)?
            .into_json()?
    };
}

/// An instance of the client using IRI URI
#[derive(Debug, Clone)]
pub struct Client {
    /// Node pool of IOTA nodes
    pub(crate) pool: Arc<RwLock<HashSet<String>>>,
    pub(crate) sync: Arc<RwLock<HashSet<String>>>,
    pub(crate) permanode: Option<String>,
    pub(crate) mwm: u8,
    pub(crate) quorum: bool,
    pub(crate) quorum_size: u8,
    pub(crate) quorum_threshold: u8,
}

impl Client {
    // /// Create the instance of IOTA Client.
    // pub fn new() -> Self {
    //     Self {
    //         pool: Arc::new(RwLock::new(HashSet::new())),
    //         sync: Arc::new(RwLock::new(Vec::new())),
    //         client: reqwest::Client::new(),
    //         mwm: 14,
    //         quorum_size: 1,
    //         quorum_threshold: 100,
    //     }
    // }

    pub(crate) async fn sync(&mut self) {
        let mut sync_list: HashMap<u32, Vec<String>> = HashMap::new();
        for url in &*self.pool.read().await {
            if let Ok(milestone) = self.get_node_info(url).await {
                let set = sync_list
                    .entry(milestone.latest_solid_subtangle_milestone_index)
                    .or_insert_with(Vec::new);
                set.push(url.clone());
            };
        }
        if let Some(synced_nodes) = sync_list.into_iter().max_by_key(|(x, _)| *x) {
            *self.sync.write().await = synced_nodes.1.into_iter().collect();
        }
    }

    /// Add a node to the node pool.
    pub async fn add_node(&mut self, uri: &str) -> Result<bool> {
        Ok(self.pool.write().await.insert(uri.to_string()))
    }

    /// Remove a node from the node pool.
    pub async fn remove_node(&mut self, uri: &str) -> Result<bool> {
        Ok(self.pool.write().await.remove(uri))
    }

    pub(crate) async fn get_node(&self) -> Result<String> {
        // TODO getbalance, isconfirmed and were_addresses_spent_from should do quorum mode
        Ok(self
            .pool
            .read()
            .await
            .iter()
            .next()
            .ok_or(Error::NodePoolEmpty)?
            .clone())
    }

    /// Calls PrepareTransfers and then sends off the bundle via SendTrytes.
    /// * [`seed`] - An iota seed.
    /// * [`transfers`] - Transfer addresses to send data/value to.
    /// * [`inputs`] - (Optional, but recommended) Input addresses used for signing. Use `get_all_inputs` to get the valid inputs yourself.
    /// * [`remainder`] - (Optional) Custom remainder address.
    /// * [`security`] - (Optional) Security level. Default is 2.
    /// * [`depth`] - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * [`min_weight_magnitude`] - Difficulty of PoW
    /// * [`reference`] - (Optional) Transaction hash from which to start the weighted random walk.
    ///
    /// [`seed`]: ../extended/struct.SendTransfersBuilder.html#method.seed
    /// [`transfers`]: ../extended/struct.SendTransfersBuilder.html#method.transfers
    /// [`inputs`]: ../extended/struct.SendTransfersBuilder.html#method.inputs
    /// [`remainder`]: ../extended/struct.SendTransfersBuilder.html#method.remainder
    /// [`security`]: ../extended/struct.SendTransfersBuilder.html#method.security
    /// [`trytes`]: ../extended/struct.SendTransfersBuilder.html#method.trytes
    /// [`depth`]: ../extended/struct.SendTransfersBuilder.html#method.depth
    /// [`min_weight_magnitude`]: ../extended/struct.SendTransfersBuilder.html#method.min_weight_magnitude
    /// [`reference`]: ../extended/struct.SendTransfersBuilder.html#method.reference
    pub fn send<'a>(&'a self, seed: Option<&'a Seed>) -> SendBuilder<'a> {
        SendBuilder::new(self, seed)
    }

    /// Finds transactions that contain the given values in their transaction fields.
    /// The parameters define the transaction fields to search for, including bundles, addresses, tags, and approvees.
    /// Using multiple transaction fields, returns transactions hashes at the intersection of those values.
    /// # Parameters
    /// * [`bundles`] - (Optional) Bundle hashes to search for
    /// * [`addresses`] - (Optional) Addresses to search for (do not include the checksum)
    /// * [`tags`] - (Optional) Tags to search for
    /// * [`approvees`] - (Optional) Child transactions to search for
    ///
    /// [`bundles`]: ../core/struct.FindTransactionsBuilder.html#method.bundles
    /// [`addresses`]: ../core/struct.FindTransactionsBuilder.html#method.addresses
    /// [`tags`]: ../core/struct.FindTransactionsBuilder.html#method.tags
    /// [`approvees`]: ../core/struct.FindTransactionsBuilder.html#method.approvees
    pub fn find_transactions(&self) -> FindTransactionsBuilder<'_> {
        FindTransactionsBuilder::new(self)
    }

    /// Creates and returns an Inputs object by generating addresses and fetching their latest balance.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`threshold`] - Minimum amount of balance required
    /// * [`index`] - (Optional) Key index to start search at. Default is 0.
    /// * [`security`] - (Optional) Security level. Default is 2.
    ///
    /// [`seed`]: ../extended/struct.GetInputsBuilder.html#method.seed
    /// [`threshold`]: ../extended/struct.GetInputsBuilder.html#method.threshold
    /// [`index`]: ../extended/struct.GetInputsBuilder.html#method.index
    /// [`security`]: ../extended/struct.GetInputsBuilder.html#method.security
    pub fn get_inputs<'a>(&'a self, seed: &'a Seed) -> GetInputsBuilder<'a> {
        GetInputsBuilder::new(self, seed)
    }

    /// Creates and returns an Inputs object by generating addresses and fetching their latest balance.
    /// Spent addresses will be included with the bundlehash(es) that were used to spent from them.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`gap_limit`] - (Optional) Amount of addresses to generate each round
    /// * [`start_index`] - (Optional) Key index to start search at. Default is 0.
    /// * [`security_lvl`] - (Optional) Security level. Default is 2.
    ///
    /// [`seed`]: ../extended/struct.GetAccountDataForMigrationBuilder.html#method.seed
    /// [`with_gap_limit`]: ../extended/struct.GetAccountDataForMigrationBuilder.html#method.with_gap_limit
    /// [`with_start_index`]: ../extended/struct.GetAccountDataForMigrationBuilder.html#method.with_start_index
    /// [`with_security_lvl`]: ../extended/struct.GetAccountDataForMigrationBuilder.html#method.with_security_lvl
    pub fn get_account_data_for_migration(&mut self) -> GetAccountDataForMigrationBuilder<'_> {
        GetAccountDataForMigrationBuilder::builder(self)
    }

    /// Creates and returns an Inputs object by fetching the latest balance of provided inputs.
    /// Spent addresses will be included with the bundlehash(es) that were used to spent from them.
    /// # Parameters
    /// * [`addresses`] - Vec<AddressInput> Addresses with index and security level.
    ///
    /// [`with_addresses`]: ../extended/struct.GetAccountDataForMigrationBuilder.html#method.with_gap_limit
    pub fn get_ledger_account_data_for_migration(
        &mut self,
    ) -> GetLedgerAccountDataForMigrationBuilder<'_> {
        GetLedgerAccountDataForMigrationBuilder::builder(self)
    }

    /// Fetch inclusion states of the given transactions to determine if the transactions are confirmed.
    ///
    /// # Parameters
    /// * [`transactions`] - List of transaction hashes for which you want to get the inclusion state
    pub async fn is_confirmed(&self, transactions: &[Hash]) -> Result<Vec<bool>> {
        let states = self
            .get_inclusion_states()
            .transactions(transactions)
            .send()
            .await?
            .states;
        Ok(states)
    }

    /// Gets latest solid subtangle milestone.
    pub async fn get_latest_solid_subtangle_milestone(&self, url: &str) -> Result<Hash> {
        let trits = TryteBuf::try_from_str(
            &self
                .get_node_info(url)
                .await?
                .latest_solid_subtangle_milestone,
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>();
        let hash = Hash::try_from_inner(trits).unwrap();
        Ok(hash)
    }

    /// Generates and returns a new address by calling `find_transactions` until the first unused address is detected.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`index`] - (Optional) Key index to start search at. Default is 0.
    /// * [`security`] - (Optional) Security level. Default is 2.
    ///
    /// [`seed`]: ../extended/struct.GenerateNewAddressBuilder.html#method.seed
    /// [`index`]: ../extended/struct.GenerateNewAddressBuilder.html#method.index
    /// [`security`]: ../extended/struct.GenerateNewAddressBuilder.html#method.security
    pub fn generate_new_address<'a>(&'a self, seed: &'a Seed) -> GenerateNewAddressBuilder<'a> {
        GenerateNewAddressBuilder::new(self, seed)
    }

    /// Prepares the transaction trytes by generating a bundle, filling in transfers and inputs,
    /// adding remainder and signing all input transactions.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`transfers`] - Transfer addresses to send data/value to.
    /// * [`inputs`] - (Optional, but recommended) Input addresses used for signing. Use `get_all_inputs` to get the valid inputs yourself.
    /// * [`remainder`] - (Optional) Custom remainder address.
    /// * [`security`] - (Optional) Security level. Default is 2.
    ///
    /// [`seed`]: ../extended/struct.PrepareTransfersBuilder.html#method.seed
    /// [`transfers`]: ../extended/struct.PrepareTransfersBuilder.html#method.transfers
    /// [`inputs`]: ../extended/struct.PrepareTransfersBuilder.html#method.inputs
    /// [`remainder`]: ../extended/struct.PrepareTransfersBuilder.html#method.remainder
    /// [`security`]: ../extended/struct.PrepareTransfersBuilder.html#method.security
    pub fn prepare_transfers<'a>(&'a self, seed: Option<&'a Seed>) -> PrepareTransfersBuilder<'a> {
        PrepareTransfersBuilder::new(self, seed)
    }

    /// Reattaches a transfer to tangle by selecting tips & performing the Proof-of-Work again.
    /// Reattachments are useful in case original transactions are pending and can be done securely as many times as needed.
    /// # Parameters
    /// * `hash` - Tail transaction hash. Tail transaction is the transaction in the bundle with current_index == 0
    /// * [`depth`] - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * [`min_weight_magnitude`] - Difficulty of PoW
    ///
    /// [`depth`]: ../extended/struct.SendTrytesBuilder.html#method.depth
    /// [`min_weight_magnitude`]: ../extended/struct.SendTrytesBuilder.html#method.min_weight_magnitude
    pub async fn reattach<'a>(&'a self, hash: &'a Hash) -> Result<SendTrytesBuilder<'a>> {
        let mut bundle = self.get_bundle(hash).await?;
        bundle.reverse();
        Ok(SendTrytesBuilder::new(self).with_trytes(bundle))
    }

    /// Store and broadcast transactions to the node.
    /// The trytes to be used for this call are returned by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    pub async fn store_and_broadcast(&self, trytes: &[Transaction]) -> Result<()> {
        self.store_transactions(trytes).await?;
        self.broadcast_transactions(trytes).await?;
        Ok(())
    }

    /// Does proof of work for the given transaction trytes.
    /// The `branch_transaction` and `trunk_transaction` parameters are returned
    /// from the `get_transactions_to_approve` method.
    /// # Parameters
    /// * [`trunk_transaction`] - trunk transaction hash
    /// * [`branch_transaction`] - branch transaction hash
    /// * [`min_weight_magnitude`] - Difficulty of PoW
    /// * [`trytes`] - Slice of transaction trytes. When sending transactions in a bundle,
    /// make sure that the trytes of the last transaction in the bundle are in index 0 of the array.
    ///
    /// [`trunk_transaction`]: ../core/struct.AttachToTangleBuilder.html#method.trunk_transaction
    /// [`branch_transaction`]: ../core/struct.AttachToTangleBuilder.html#method.branch_transaction
    /// [`min_weight_magnitude`]: ../core/struct.AttachToTangleBuilder.html#method.min_weight_magnitude
    /// [`trytes`]: ../core/struct.AttachToTangleBuilder.html#method.trytes
    pub fn attach_to_tangle(&self) -> AttachToTangleBuilder<'_> {
        AttachToTangleBuilder::new(self)
    }

    /// Gets the confirmed balance of an address.
    /// If the tips parameter is missing, the returned balance is correct as of the latest confirmed milestone.
    /// This endpoint returns data only if the node is synchronized.
    /// # Parameters
    /// * [`addresses`] - Address for which to get the balance (do not include the checksum)
    /// * [`threshold`] - (Optional) Confirmation threshold between 0 and 100, default is 100.
    /// * [`tips`] - (Optional) Tips whose history of transactions to traverse to find the balance
    ///
    /// [`addresses`]: ../core/struct.GetBalancesBuilder.html#method.addresses
    /// [`threshold`]: ../core/struct.GetBalancesBuilder.html#method.threshold
    /// [`tips`]: ../core/struct.GetBalancesBuilder.html#method.tips
    pub fn get_balances(&self) -> GetBalancesBuilder<'_> {
        GetBalancesBuilder::new(self)
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
    pub fn get_inclusion_states(&self) -> GetInclusionStatesBuilder<'_> {
        GetInclusionStatesBuilder::new(self)
    }

    /// Gets information about a node.
    pub async fn get_node_info(&self, url: &str) -> Result<GetNodeInfoResponse> {
        let body = json!( {
            "command": "getNodeInfo",
        });

        let res = response!(self, body, url);

        Ok(res)
    }

    /// Get tip info.
    pub async fn get_tip_info(&self, hash: &Hash) -> Result<GetTipInfoResponse> {
        let body = json!( {
            "command": "getTipInfo",
            "tailTransaction": hash.encode::<T3B1Buf>()
                    .iter_trytes()
                    .map(char::from)
                    .collect::<String>()
        });

        let res = response!(self, body);

        Ok(res)
    }

    /// Gets a node's API configuration settings.
    pub async fn get_node_api_configuration(
        &self,
        url: &str,
    ) -> Result<GetNodeAPIConfigurationResponse> {
        let body = json!( {
            "command": "getNodeAPIConfiguration",
        });

        let res = response!(self, body, url);

        Ok(res)
    }

    /// Gets two consistent tip transaction hashes to use as branch/trunk transactions.
    /// This endpoint returns data only if the node is synchronized.
    /// # Parameters
    /// * [`depth`] - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * [`reference`] - (Optional) Transaction hash from which to start the weighted random walk.
    /// Use this parameter to make sure the returned tip transaction hashes approve a given reference transaction
    ///
    /// [`depth`]: ../core/struct.GetTransactionsToApproveBuilder.html#method.depth
    /// [`reference`]: ../core/struct.GetTransactionsToApproveBuilder.html#method.reference
    pub fn get_transactions_to_approve(&self) -> GetTransactionsToApproveBuilder<'_> {
        GetTransactionsToApproveBuilder::new(self)
    }

    /// Gets a transaction's contents in trytes.
    /// # Parameters
    /// * `hashes` - Transaction hashes
    ///
    /// [`hashes`]: ../core/struct.GetTrytesBuilder.html#method.hashes
    pub async fn get_trytes(&self, hashes: &[Hash]) -> Result<GetTrytesResponse> {
        let hashes: Vec<String> = hashes
            .iter()
            .map(|h| {
                (*h).encode::<T3B1Buf>()
                    .iter_trytes()
                    .map(char::from)
                    .collect::<String>()
            })
            .collect();
        let body = json!({
            "command": "getTrytes",
            "hashes": hashes,
        });
        let res: GetTrytesResponseBuilder = match &self.permanode {
            Some(url) => {
                let body_ = body.clone();
                // Wrapper function so we can handle the result from the response! macro
                fn get_perma_response(
                    _client: &Client,
                    body: serde_json::Value,
                    url: &str,
                ) -> Result<GetTrytesResponseBuilder> {
                    let res: GetTrytesResponseBuilder = response!(_client, body, url);
                    Ok(res)
                }
                match get_perma_response(self, body, url) {
                    Ok(res) => res,
                    Err(_) => {
                        // Send request to a normal node in case the permanode failed
                        let res: GetTrytesResponseBuilder = response!(self, body_);
                        res
                    }
                }
            }
            None => response!(self, body),
        };
        res.build()
    }

    /// Sends transaction trytes to a node.
    /// The input trytes for this call are provided by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    ///
    /// [`trytes`]: ../core/struct.BroadcastTransactionsBuilder.html#method.trytes
    pub async fn broadcast_transactions(&self, trytes: &[Transaction]) -> Result<()> {
        let trytes: Vec<String> = trytes.iter().map(|tx| tx_trytes(tx)).collect();
        let body = json!({
            "command": "broadcastTransactions",
            "trytes": trytes,
        });

        let res: ErrorResponseBuilder = response!(self, body);
        res.build().await
    }

    /// Store transactions into the local storage.
    /// The trytes to be used for this call are returned by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    ///
    /// [`trytes`]: ../core/struct.StoreTransactionsBuilder.html#method.trytes
    pub async fn store_transactions(&self, trytes: &[Transaction]) -> Result<()> {
        let trytes: Vec<String> = trytes.iter().map(|tx| tx_trytes(tx)).collect();
        let body = json!({
            "command": "storeTransactions",
            "trytes": trytes,
        });

        let res: ErrorResponseBuilder = response!(self, body);
        res.build().await
    }

    /// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
    /// If an address has a pending transaction, it's also considered 'spent'.
    /// # Parameters
    /// * `address` - addresses to check (do not include the checksum)
    pub async fn were_addresses_spent_from(
        &self,
        addresses: &[Address],
    ) -> Result<WereAddressesSpentFromResponse> {
        let addresses: Vec<String> = addresses
            .iter()
            .map(|h| {
                h.to_inner()
                    .encode::<T3B1Buf>()
                    .iter_trytes()
                    .map(char::from)
                    .collect::<String>()
            })
            .collect();
        let body = json!({
            "command": "wereAddressesSpentFrom",
            "addresses": addresses,
        });

        let res: WereAddressesSpentFromResponseBuilder = response!(self, body);
        res.build().await
    }

    /// Re-broadcasts all transactions in a bundle given the tail transaction hash. It might be useful
    /// when transactions did not properly propagate, particularly in the case of large bundles.
    ///
    /// # Parameters
    /// * `hash` - Tail transaction hash (current_index == 0)
    pub async fn broadcast_bundle(&self, hash: &Hash) -> Result<Vec<Transaction>> {
        let mut bundle = self.get_bundle(hash).await?;
        bundle.reverse();

        self.broadcast_transactions(&bundle).await?;
        Ok(bundle)
    }

    /// Checks the consistency of transactions. A consistent transaction is one where the following statements are true:
    /// * The node isn't missing the transaction's branch or trunk transactions
    /// * The transaction's bundle is valid
    /// * The transaction's branch and trunk transactions are valid
    /// # Parameters
    /// * [`tails`] - Transaction hashes to check
    ///
    /// [`tails`]: ../core/struct.ConsistencyBuilder.html#method.tails
    pub async fn check_consistency(&self, tails: &[Hash]) -> Result<ConsistencyResponse> {
        let tails: Vec<String> = tails
            .iter()
            .map(|h| {
                (*h).encode::<T3B1Buf>()
                    .iter_trytes()
                    .map(char::from)
                    .collect::<String>()
            })
            .collect();
        let body = json!({
            "command": "checkConsistency",
            "tails": tails,
        });

        let res: ConsistencyResponseBuilder = response!(self, body);
        res.build().await
    }

    /// Fetches and validates the bundle given a tail transaction hash, by calling [`traverse_bundle`]
    /// and traversing through trunk transaction.
    /// # Parameters
    /// * [`hash`] - Tail transaction hash (current_index == 0)
    ///
    /// [`traverse_bundle`]: #method.traverse_bundle
    pub async fn get_bundle(&self, hash: &Hash) -> Result<Vec<Transaction>> {
        // TODO validate bundle once it's in iota_bundle_preview's bundle types
        let bundle = self.traverse_bundle(hash).await?;
        Ok(bundle)
    }

    /// Fetches the bundle of a given the tail transaction hash, by traversing through trunk transaction.
    /// It does not validate the bundle. Use [`get_bundle`] instead to get validated bundle.
    ///
    /// # Parameters
    /// * [`hash`] - Tail transaction hash (current_index == 0)
    ///
    /// [`get_bundle`]: #method.get_bundle
    pub async fn traverse_bundle(&self, hash: &Hash) -> Result<Vec<Transaction>> {
        let mut bundle = Vec::new();
        let mut hash = *hash;
        let mut tail = true;
        loop {
            let res = self.get_trytes(&[hash]).await?.trytes.pop().unwrap();

            if tail {
                if *res.index().to_inner() != 0 {
                    break Err(Error::NotTailHash);
                }
                tail = false;
            }

            hash = *res.trunk();
            if res.index() == res.last_index() {
                bundle.push(res);
                break Ok(bundle);
            } else {
                bundle.push(res);
            }
        }
    }

    /// Checks whether an address is used via FindTransactions and WereAddressesSpentFrom.
    /// # Parameters
    /// * `address` - IOTA address
    pub async fn is_address_used(&self, address: &Address) -> Result<bool> {
        let addresses = &[address.clone()];
        let spent = self.were_addresses_spent_from(addresses).await?.states[0];

        // TODO more address evaluations
        if spent {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// checks if a transaction is promotable by calling the checkConsistency IRI API command and
    /// verifying that attachmentTimestamp is above a lower bound. Lower bound is calculated based on the number of milestones issued
    /// since transaction attachment.
    /// # Parameters
    /// * `tail` - Tail Transaction Hash
    pub async fn is_promotable(&self, tail: &Hash) -> Result<bool> {
        let is_consistent = self.check_consistency(&[*tail]).await?.state;

        let timestamp = *self.get_trytes(&[*tail]).await?.trytes[0]
            .attachment_ts()
            .to_inner() as i64;

        let now = chrono::Utc::now().timestamp();
        let milestone_interval: i64 = 2 * 60 * 1000;
        let one_way_delay: i64 = 60 * 1000;
        let max_depth: i64 = 6;

        let is_above_max_depth =
            timestamp < now && (now - timestamp) < milestone_interval * one_way_delay * max_depth;
        Ok(is_consistent && is_above_max_depth)
    }

    /// Perform Attaches to tanlge, stores and broadcasts a vector of transaction trytes.
    /// # Parameters
    /// * [`trytes`] - Vector of trytes to attach, store & broadcast
    /// * [`depth`] - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * [`min_weight_magnitude`] - Difficulty of PoW
    /// * [`reference`] - (Optional) Transaction hash from which to start the weighted random walk.
    ///
    /// [`trytes`]: ../extended/struct.SendTrytesBuilder.html#method.trytes
    /// [`depth`]: ../extended/struct.SendTrytesBuilder.html#method.depth
    /// [`min_weight_magnitude`]: ../extended/struct.SendTrytesBuilder.html#method.min_weight_magnitude
    /// [`reference`]: ../extended/struct.SendTrytesBuilder.html#method.reference
    pub fn send_trytes(&self) -> SendTrytesBuilder<'_> {
        SendTrytesBuilder::new(self)
    }
}
