//! Trivial API that users usually don't need to call.

use crate::error::*;
use crate::extended::*;
use crate::client::Client;

use bee_crypto::ternary::Hash;
use bee_signing::ternary::seed::Seed;
use bee_ternary::{T1B1Buf, TryteBuf};
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
