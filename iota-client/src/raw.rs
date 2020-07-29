//! Raw API of any Bee/IRI/Hornet node.

use crate::client::Client;
use crate::core::*;
use crate::error::*;
use crate::response::*;
use crate::util::tx_trytes;

use bee_crypto::ternary::Hash;
use bee_ternary::T3B1Buf;
use bee_transaction::bundled::{
    Address, BundledTransaction as Transaction, BundledTransactionField,
};
use reqwest::Url;

impl Client {
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
}
