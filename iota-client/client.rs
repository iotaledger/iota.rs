//! The Client module to connect through IRI with API usages
use crate::core::*;
use crate::extended::*;
use crate::response::*;
use crate::util::Bundle;
use anyhow::Result;
use bee_bundle::{Address, Hash, Transaction, TransactionField};

macro_rules! response {
    ($self:ident, $body:ident) => {
        $self
            .client
            .post($self.uri)
            .header("Content-Type", "application/json")
            .header("X-IOTA-API-Version", "1")
            .body($body.to_string())
            .send()
            .await?
            .json()
            .await?
    };
}

/// An instance of the client using IRI URI
#[derive(Debug)]
pub struct Client<'a> {
    /// URI of IRI connection
    pub(crate) uri: &'a str,
    /// A reqwest Client to make Requests with
    pub(crate) client: reqwest::Client,
}

impl Client<'_> {
    /// Create a new instance of IOTA Client
    pub fn new(uri: &str) -> Client<'_> {
        Client {
            uri,
            client: reqwest::Client::new(),
        }
    }

    /// Add a list of neighbors to your node. It should be noted that
    /// this is only temporary, and the added neighbors will be removed
    /// from your set of neighbors after you relaunch IRI.
    /// # Parameters
    /// * [`uris`] - Slices of neighbor URIs(`&str`) to add
    ///
    /// [`uris`]: ../core/struct.AddNeighborsBuilder.html#method.uris
    pub fn add_neighbors(&self) -> AddNeighborsBuilder<'_> {
        AddNeighborsBuilder::new(&self)
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
        AttachToTangleBuilder::new(&self)
    }

    /// Re-broadcasts all transactions in a bundle given the tail transaction hash. It might be useful
    /// when transactions did not properly propagate, particularly in the case of large bundles.
    ///
    /// # Parameters
    /// * `hash` - Tail transaction hash (current_index == 0)
    pub async fn broadcast_bundle(&self, hash: &Hash) -> Result<Bundle> {
        let mut bundle = self.get_bundle(hash).await?;
        bundle.reverse();

        self.broadcast_transactions().trytes(&bundle).send().await?;
        Ok(bundle)
    }

    /// Sends transaction trytes to a node.
    /// The input trytes for this call are provided by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    ///
    /// [`trytes`]: ../core/struct.BroadcastTransactionsBuilder.html#method.trytes
    pub fn broadcast_transactions(&self) -> BroadcastTransactionsBuilder<'_> {
        BroadcastTransactionsBuilder::new(&self)
    }

    /// Checks the consistency of transactions. A consistent transaction is one where the following statements are true:
    /// * The node isn't missing the transaction's branch or trunk transactions
    /// * The transaction's bundle is valid
    /// * The transaction's branch and trunk transactions are valid
    /// # Parameters
    /// * [`tails`] - Transaction hashes to check
    ///
    /// [`tails`]: ../core/struct.ConsistencyBuilder.html#method.tails
    pub fn check_consistency(&self) -> CheckConsistencyBuilder<'_> {
        CheckConsistencyBuilder::new(&self)
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
        FindTransactionsBuilder::new(&self)
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
        GetBalancesBuilder::new(&self)
    }

    /// Fetches and validates the bundle given a tail transaction hash, by calling [`traverse_bundle`]
    /// and traversing through trunk transaction.
    /// # Parameters
    /// * [`hash`] - Tail transaction hash (current_index == 0)
    ///
    /// [`traverse_bundle`]: #method.traverse_bundle
    pub async fn get_bundle(&self, hash: &Hash) -> Result<Bundle> {
        // TODO validate bundle once it's in bee_bundle's bundle types
        let bundle = self.traverse_bundle(hash).await?;
        Ok(bundle)
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
        GetInclusionStatesBuilder::new(&self)
    }

    /// Gets latest solid subtangle milestone.
    pub async fn get_latest_solid_subtangle_milestone(&self) -> Result<String> {
        Ok(self.get_node_info().await?.latest_solid_subtangle_milestone)
    }

    /// Gets all transaction hashes that a node is currently requesting from its neighbors.
    pub async fn get_missing_transactions(&self) -> Result<GetTipsResponse> {
        let body = json!( {
            "command": "getMissingTransactions",
        });

        let res = response!(self, body);

        Ok(res)
    }

    /// Gets a node's neighbors and their activity.
    pub async fn get_neighbors(&self) -> Result<GetNeighborsResponse> {
        let body = json!( {
            "command": "getNeighbors",
        });

        let res: GetNeighborsResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Generates and returns a new address by calling `find_transactions` until the first unused address is detected.
    /// This stops working after a snapshot.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`index`] - (Optional) Key index to start search at. Default is 0.
    /// * [`security`] - (Optional) Security level. Default is 2.
    /// Use this parameter to make sure the returned tip transaction hashes approve a given reference transaction
    ///
    /// [`seed`]: ../extended/struct.GetNewAddressBuilder.html#method.seed
    /// [`index`]: ../extended/struct.GetNewAddressBuilder.html#method.index
    /// [`security`]: ../extended/struct.GetNewAddressBuilder.html#method.security
    pub fn get_new_address(&self) -> GetNewAddressBuilder<'_> {
        GetNewAddressBuilder::new(&self)
    }

    /// Gets a node's API configuration settings.
    pub async fn get_node_api_configuration(&self) -> Result<GetNodeAPIConfigurationResponse> {
        let body = json!( {
            "command": "getNodeAPIConfiguration",
        });

        let res = response!(self, body);

        Ok(res)
    }

    /// Gets information about a node.
    pub async fn get_node_info(&self) -> Result<GetNodeInfoResponse> {
        let body = json!( {
            "command": "getNodeInfo",
        });

        let res = response!(self, body);

        Ok(res)
    }

    /// Gets tip transaction hashes from a node.
    pub async fn get_tips(&self) -> Result<GetTipsResponse> {
        let body = json!( {
            "command": "getTips",
        });

        let res = response!(self, body);

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
        GetTransactionsToApproveBuilder::new(&self)
    }

    /// Gets a transaction's contents in trytes.
    /// # Parameters
    /// * `hashes` - Transaction hashes
    ///
    /// [`hashes`]: ../core/struct.GetTrytesBuilder.html#method.hashes
    pub fn get_trytes(&self) -> GetTrytesBuilder<'_> {
        GetTrytesBuilder::new(&self)
    }

    /// Aborts the process that's started by the `attach_to_tangle` method.
    pub async fn interrupt_attaching_to_tangle(&self) -> Result<()> {
        let body = json!( {
            "command": "interruptAttachingToTangle",
        });

        let _ = self
            .client
            .post(self.uri)
            .header("Content-Type", "application/json")
            .header("X-IOTA-API-Version", "1")
            .body(body.to_string())
            .send()
            .await?;

        Ok(())
    }

    /// Checks whether an address is used via FindTransactions and WereAddressesSpentFrom.
    /// # Parameters
    /// * `address` - IOTA address
    pub async fn is_address_used(&self, address: &Address) -> Result<bool> {
        let addresses = &[address.clone()];
        let spent = self
            .were_addresses_spent_from()
            .address(addresses)
            .send()
            .await?
            .states[0];
        let len = self
            .find_transactions()
            .addresses(addresses)
            .send()
            .await?
            .hashes
            .len();

        if spent || len > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Removes a list of neighbors to your node.
    /// This is only temporary, and if you have your neighbors
    /// added via the command line, they will be retained after
    /// you restart your node.
    /// # Parameters
    /// * [`uris`] - Slice of neighbor URIs(`&str`) to remove
    ///
    /// [`uris`]: ../core/struct.RemoveNeighborsBuilder.html#method.uris
    pub fn remove_neighbors(&self) -> RemoveNeighborsBuilder<'_> {
        RemoveNeighborsBuilder::new(&self)
    }

    /// Reattaches a transfer to tangle by selecting tips & performing the Proof-of-Work again.
    /// Reattachments are usefull in case original transactions are pending, and can be done securely as many times as needed.
    /// # Parameters
    /// * `hash` - Tail transaction hash. Tail transaction is the transaction in the bundle with current_index == 0
    /// * [`depth`] - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * [`min_weight_magnitude`] - Difficulty of PoW
    ///
    /// [`depth`]: ../extended/struct.SendTrytesBuilder.html#method.depth
    /// [`min_weight_magnitude`]: ../extended/struct.SendTrytesBuilder.html#method.min_weight_magnitude
    pub async fn replay_bundle(&self, hash: &Hash) -> Result<SendTrytesBuilder<'_>> {
        let mut bundle = self.get_bundle(hash).await?;
        bundle.reverse();
        Ok(SendTrytesBuilder::new(&self).trytes(bundle))
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
        SendTrytesBuilder::new(&self)
    }

    /// Store and broadcast transactions to the node.
    /// The trytes to be used for this call are returned by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    pub async fn store_and_broadcast(&self, trytes: &[Transaction]) -> Result<()> {
        StoreTransactionsBuilder::new(&self)
            .trytes(trytes)
            .send()
            .await?;
        BroadcastTransactionsBuilder::new(&self)
            .trytes(trytes)
            .send()
            .await?;
        Ok(())
    }

    /// Store transactions into the local storage.
    /// The trytes to be used for this call are returned by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    ///
    /// [`trytes`]: ../core/struct.StoreTransactionsBuilder.html#method.trytes
    pub fn store_transactions(&self) -> StoreTransactionsBuilder<'_> {
        StoreTransactionsBuilder::new(&self)
    }

    /// Fetches the bundle of a given the tail transaction hash, by traversing through trunk transaction.
    /// It does not validate the bundle. Use [`get_bundle`] instead to get validated bundle.
    ///
    /// # Parameters
    /// * [`hash`] - Tail transaction hash (current_index == 0)
    ///
    /// [`get_bundle`]: #method.get_bundle
    pub async fn traverse_bundle(&self, hash: &Hash) -> Result<Bundle> {
        let mut bundle = Bundle::new();
        let mut hash = *hash;
        let mut tail = true;
        loop {
            let res = self
                .get_trytes()
                .hashes(&[hash])
                .send()
                .await?
                .trytes
                .pop()
                .unwrap();

            if tail {
                if *res.index().to_inner() != 0 {
                    break Err(anyhow!("Provided hash is not tail."));
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

    /// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
    /// If an address has a pending transaction, it's also considered 'spent'.
    /// # Parameters
    /// * [`address`] - addresses to check (do not include the checksum)
    ///
    /// [`address`]: ../core/struct.WereAddressesSpentFromBuilder.html#method.address
    pub fn were_addresses_spent_from(&self) -> WereAddressesSpentFromBuilder<'_> {
        WereAddressesSpentFromBuilder::new(&self)
    }
}
