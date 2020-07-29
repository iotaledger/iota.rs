//! Trivial API that users usually don't need to call.

use crate::core::*;
use crate::error::*;
use crate::extended::*;
use crate::response::*;
use crate::util::tx_trytes;
use crate::client::Client;

use bee_crypto::ternary::Hash;
use bee_signing::ternary::seed::Seed;
use bee_ternary::{T1B1Buf, T3B1Buf, TryteBuf};
use bee_transaction::bundled::{
    Address, BundledTransaction as Transaction, BundledTransactionField,
};
use bee_transaction::TransactionVertex;
use reqwest::Url;

impl Client {
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

    /// Gets latest solid subtangle milestone.
    pub async fn get_latest_solid_subtangle_milestone(&self, url: Url) -> Result<Hash> {
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

    /// Prepares the transaction trytes by generating a bundle, filling in transfers and inputs,
    /// adding remainder and signing all input transactions.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`transfers`] - Transfer addresses to send data/value to.
    /// * [`inputs`] - (Optional, but recommended) Input addresses used for signing. Use `get_inputs` to get the valid inputs yourself.
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
    pub async fn get_node_info(&self, url: Url) -> Result<GetNodeInfoResponse> {
        let body = json!( {
            "command": "getNodeInfo",
        });

        let res = response!(self, body, url);

        Ok(res)
    }

    /// Gets a node's API configuration settings.
    pub async fn get_node_api_configuration(
        &self,
        url: Url,
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

        let res: GetTrytesResponseBuilder = response!(self, body);
        res.build().await
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

            hash = res.trunk().clone();
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
        let one_way_delay: i64 = 1 * 60 * 1000;
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
