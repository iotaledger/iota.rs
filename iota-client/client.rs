//! The Client module to connect through IRI with API usages
use crate::core::*;
use crate::error::*;
use crate::extended::*;
use crate::response::*;
use crate::util::tx_trytes;

use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use iota_bundle_preview::{Address, Hash, Transaction, TransactionField};
use iota_conversion::Trinary;
use iota_crypto_preview::Kerl;
use iota_signing_preview::IotaSeed;
use iota_ternary_preview::TryteBuf;
use once_cell::sync::Lazy;
use reqwest::Url;

macro_rules! response {
    ($body:ident) => {
        Client::get()
            .client
            .post(Client::get_node()?)
            .header("Content-Type", "application/json")
            .header("X-IOTA-API-Version", "1")
            .body($body.to_string())
            .send()
            .await?
            .json()
            .await?
    };
    ($body:ident, $node:ident) => {
        Client::get()
            .client
            .post($node)
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
pub struct Client {
    // Node pool of IOTA nodes
    pub(crate) pool: Arc<RwLock<HashSet<Url>>>,
    /// A reqwest Client to make Requests with
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Get the instance of IOTA Client. It will init the instance if it's not created yet.
    pub fn get() -> &'static Client {
        static CLIENT: Lazy<Client> = Lazy::new(|| Client {
            pool: Arc::new(RwLock::new(HashSet::new())),
            client: reqwest::Client::new(),
        });

        &CLIENT
    }

    /// Add a node to the node pool.
    pub fn add_node(uri: &str) -> Result<bool> {
        let url = Url::parse(uri).map_err(|_| Error::UrlError)?;
        let pool = Client::get().pool.clone();
        let mut set = pool.write().expect("Node pool write poisened");
        Ok(set.insert(url))
    }

    /// Remove a node from the node pool.
    pub fn remove_node(uri: &str) -> Result<bool> {
        let url = Url::parse(uri).map_err(|_| Error::UrlError)?;
        let pool = Client::get().pool.clone();
        let mut set = pool.write().expect("Node pool write poisened");
        Ok(set.remove(&url))
    }

    pub(crate) fn get_node() -> Result<Url> {
        Ok(Client::get()
            .pool
            .clone()
            .read()
            .expect("Node pool read poinsened")
            .iter()
            .next()
            .ok_or(Error::NodePoolEmpty)?
            .clone())
    }

    /// Add a list of neighbors to your node. It should be noted that
    /// this is only temporary, and the added neighbors will be removed
    /// from your set of neighbors after you relaunch IRI.
    /// # Parameters
    /// * [`uris`] - Slices of neighbor URIs(`&str`) to add
    ///
    /// [`uris`]: ../core/struct.AddNeighborsBuilder.html#method.uris
    pub async fn add_neighbors(uris: Vec<&str>) -> Result<AddNeighborsResponse> {
        for uri in &uris {
            match Url::parse(&uri).map_err(|_| Error::UrlError)?.scheme() {
                "tcp" | "udp" => (),
                _ => return Err(Error::UrlError),
            }
        }

        let body = json!({
            "command": "addNeighbors",
            "uris": uris,
        });

        Ok(response!(body))
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
    pub fn attach_to_tangle() -> AttachToTangleBuilder {
        AttachToTangleBuilder::new()
    }

    /// Re-broadcasts all transactions in a bundle given the tail transaction hash. It might be useful
    /// when transactions did not properly propagate, particularly in the case of large bundles.
    ///
    /// # Parameters
    /// * `hash` - Tail transaction hash (current_index == 0)
    pub async fn broadcast_bundle(hash: &Hash) -> Result<Vec<Transaction>> {
        let mut bundle = Client::get_bundle(hash).await?;
        bundle.reverse();

        Client::broadcast_transactions(&bundle).await?;
        Ok(bundle)
    }

    /// Sends transaction trytes to a node.
    /// The input trytes for this call are provided by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    ///
    /// [`trytes`]: ../core/struct.BroadcastTransactionsBuilder.html#method.trytes
    pub async fn broadcast_transactions(trytes: &[Transaction]) -> Result<()> {
        let trytes: Vec<String> = trytes.iter().map(|tx| tx_trytes(tx)).collect();
        let body = json!({
            "command": "broadcastTransactions",
            "trytes": trytes,
        });

        let res: ErrorResponseBuilder = response!(body);
        res.build().await
    }

    /// Checks the consistency of transactions. A consistent transaction is one where the following statements are true:
    /// * The node isn't missing the transaction's branch or trunk transactions
    /// * The transaction's bundle is valid
    /// * The transaction's branch and trunk transactions are valid
    /// # Parameters
    /// * [`tails`] - Transaction hashes to check
    ///
    /// [`tails`]: ../core/struct.ConsistencyBuilder.html#method.tails
    pub async fn check_consistency(tails: &[Hash]) -> Result<ConsistencyResponse> {
        let tails: Vec<String> = tails
            .iter()
            .map(|h| h.as_bytes().trytes().unwrap())
            .collect();
        let body = json!({
            "command": "checkConsistency",
            "tails": tails,
        });

        let res: ConsistencyResponseBuilder = response!(body);
        res.build().await
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
    pub fn find_transactions() -> FindTransactionsBuilder {
        FindTransactionsBuilder::new()
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
    pub fn get_balances() -> GetBalancesBuilder {
        GetBalancesBuilder::new()
    }

    /// Fetches and validates the bundle given a tail transaction hash, by calling [`traverse_bundle`]
    /// and traversing through trunk transaction.
    /// # Parameters
    /// * [`hash`] - Tail transaction hash (current_index == 0)
    ///
    /// [`traverse_bundle`]: #method.traverse_bundle
    pub async fn get_bundle(hash: &Hash) -> Result<Vec<Transaction>> {
        // TODO validate bundle once it's in iota_bundle_preview's bundle types
        let bundle = Client::traverse_bundle(hash).await?;
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
    pub fn get_inclusion_states() -> GetInclusionStatesBuilder {
        GetInclusionStatesBuilder::new()
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
    pub fn get_inputs(seed: &IotaSeed<Kerl>) -> GetInputsBuilder<'_> {
        GetInputsBuilder::new(seed)
    }

    /// Fetches inclusion states of the given transactions by calling GetInclusionStates
    /// using the latest solid subtangle milestone from GetNodeInfo.
    ///
    /// # Parameters
    /// * [`transactions`] - List of transaction hashes for which you want to get the inclusion state
    pub async fn get_latest_inclusion(transactions: &[Hash]) -> Result<Vec<bool>> {
        let milestone = Client::get_latest_solid_subtangle_milestone().await?;
        let states = Client::get_inclusion_states()
            .transactions(transactions)
            .tips(&[milestone])
            .send()
            .await?
            .states;
        Ok(states)
    }

    /// Gets latest solid subtangle milestone.
    pub async fn get_latest_solid_subtangle_milestone() -> Result<Hash> {
        Ok(Hash::from_inner_unchecked(
            // TODO missing impl error on Hash
            TryteBuf::try_from_str(
                &Client::get_node_info()
                    .await?
                    .latest_solid_subtangle_milestone,
            )
            .unwrap()
            .as_trits()
            .encode(),
        ))
    }

    /// Gets all transaction hashes that a node is currently requesting from its neighbors.
    pub async fn get_missing_transactions() -> Result<GetTipsResponse> {
        let body = json!( {
            "command": "getMissingTransactions",
        });

        let res = response!(body);

        Ok(res)
    }

    /// Gets a node's neighbors and their activity.
    pub async fn get_neighbors() -> Result<GetNeighborsResponse> {
        let body = json!( {
            "command": "getNeighbors",
        });

        let res: GetNeighborsResponseBuilder = response!(body);

        res.build().await
    }

    /// Generates and returns a new address by calling `find_transactions` until the first unused address is detected.
    /// This stops working after a snapshot.
    /// # Parameters
    /// * [`seed`] - An iota seed.
    /// * [`index`] - (Optional) Key index to start search at. Default is 0.
    /// * [`security`] - (Optional) Security level. Default is 2.
    ///
    /// [`seed`]: ../extended/struct.GetNewAddressBuilder.html#method.seed
    /// [`index`]: ../extended/struct.GetNewAddressBuilder.html#method.index
    /// [`security`]: ../extended/struct.GetNewAddressBuilder.html#method.security
    pub fn get_new_address(seed: &IotaSeed<Kerl>) -> GetNewAddressBuilder<'_> {
        GetNewAddressBuilder::new(seed)
    }

    /// Gets a node's API configuration settings.
    pub async fn get_node_api_configuration() -> Result<GetNodeAPIConfigurationResponse> {
        let body = json!( {
            "command": "getNodeAPIConfiguration",
        });

        let res = response!(body);

        Ok(res)
    }

    /// Gets information about a node.
    pub async fn get_node_info() -> Result<GetNodeInfoResponse> {
        let body = json!( {
            "command": "getNodeInfo",
        });

        let res = response!(body);

        Ok(res)
    }

    /// Gets tip transaction hashes from a node.
    pub async fn get_tips() -> Result<GetTipsResponse> {
        let body = json!( {
            "command": "getTips",
        });

        let res = response!(body);

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
    pub fn get_transactions_to_approve() -> GetTransactionsToApproveBuilder {
        GetTransactionsToApproveBuilder::new()
    }

    /// Gets a transaction's contents in trytes.
    /// # Parameters
    /// * `hashes` - Transaction hashes
    ///
    /// [`hashes`]: ../core/struct.GetTrytesBuilder.html#method.hashes
    pub async fn get_trytes(hashes: &[Hash]) -> Result<GetTrytesResponse> {
        let hashes: Vec<String> = hashes
            .iter()
            .map(|h| h.as_bytes().trytes().unwrap())
            .collect();
        let body = json!({
            "command": "getTrytes",
            "hashes": hashes,
        });

        let res: GetTrytesResponseBuilder = response!(body);
        res.build().await
    }

    /// Aborts the process that's started by the `attach_to_tangle` method.
    pub async fn interrupt_attaching_to_tangle() -> Result<()> {
        let body = json!( {
            "command": "interruptAttachingToTangle",
        });

        let _ = Client::get()
            .client
            .post(Client::get_node()?)
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
    pub async fn is_address_used(address: &Address) -> Result<bool> {
        let addresses = &[address.clone()];
        let spent = Client::were_addresses_spent_from(addresses).await?.states[0];

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
    pub async fn is_promotable(tail: &Hash) -> Result<bool> {
        let is_consistent = Client::check_consistency(&[*tail]).await?.state;

        let timestamp = *Client::get_trytes(&[*tail]).await?.trytes[0]
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
    pub fn prepare_transfers(seed: Option<&IotaSeed<Kerl>>) -> PrepareTransfersBuilder<'_> {
        PrepareTransfersBuilder::new(seed)
    }

    /// Removes a list of neighbors to your node.
    /// This is only temporary, and if you have your neighbors
    /// added via the command line, they will be retained after
    /// you restart your node.
    /// # Parameters
    /// * [`uris`] - Slice of neighbor URIs(`&str`) to remove
    ///
    /// [`uris`]: ../core/struct.RemoveNeighborsBuilder.html#method.uris
    pub async fn remove_neighbors(uris: Vec<&str>) -> Result<RemoveNeighborsResponse> {
        for uri in &uris {
            match Url::parse(&uri).map_err(|_| Error::UrlError)?.scheme() {
                "tcp" | "udp" => (),
                _ => return Err(Error::UrlError),
            }
        }

        let body = json!({
            "command": "removeNeighbors",
            "uris": uris,
        });

        Ok(response!(body))
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
    pub async fn replay_bundle(hash: &Hash) -> Result<SendTrytesBuilder> {
        let mut bundle = Client::get_bundle(hash).await?;
        bundle.reverse();
        Ok(SendTrytesBuilder::new().trytes(bundle))
    }

    /// Calls PrepareTransfers and then sends off the bundle via SendTrytes.
    /// * [`seed`] - An iota seed.
    /// * [`transfers`] - Transfer addresses to send data/value to.
    /// * [`inputs`] - (Optional, but recommended) Input addresses used for signing. Use `get_inputs` to get the valid inputs yourself.
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
    pub fn send_transfers(seed: Option<&IotaSeed<Kerl>>) -> SendTransfersBuilder<'_> {
        SendTransfersBuilder::new(seed)
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
    pub fn send_trytes() -> SendTrytesBuilder {
        SendTrytesBuilder::new()
    }

    /// Store and broadcast transactions to the node.
    /// The trytes to be used for this call are returned by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    pub async fn store_and_broadcast(trytes: &[Transaction]) -> Result<()> {
        Client::store_transactions(trytes).await?;
        Client::broadcast_transactions(trytes).await?;
        Ok(())
    }

    /// Store transactions into the local storage.
    /// The trytes to be used for this call are returned by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * [`trytes`] - Transaction trytes
    ///
    /// [`trytes`]: ../core/struct.StoreTransactionsBuilder.html#method.trytes
    pub async fn store_transactions(trytes: &[Transaction]) -> Result<()> {
        let trytes: Vec<String> = trytes.iter().map(|tx| tx_trytes(tx)).collect();
        let body = json!({
            "command": "storeTransactions",
            "trytes": trytes,
        });

        let res: ErrorResponseBuilder = response!(body);
        res.build().await
    }

    /// Fetches the bundle of a given the tail transaction hash, by traversing through trunk transaction.
    /// It does not validate the bundle. Use [`get_bundle`] instead to get validated bundle.
    ///
    /// # Parameters
    /// * [`hash`] - Tail transaction hash (current_index == 0)
    ///
    /// [`get_bundle`]: #method.get_bundle
    pub async fn traverse_bundle(hash: &Hash) -> Result<Vec<Transaction>> {
        let mut bundle = Vec::new();
        let mut hash = *hash;
        let mut tail = true;
        loop {
            let res = Client::get_trytes(&[hash]).await?.trytes.pop().unwrap();

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

    /// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
    /// If an address has a pending transaction, it's also considered 'spent'.
    /// # Parameters
    /// * `address` - addresses to check (do not include the checksum)
    pub async fn were_addresses_spent_from(
        addresses: &[Address],
    ) -> Result<WereAddressesSpentFromResponse> {
        let addresses: Vec<String> = addresses
            .iter()
            .map(|h| h.to_inner().as_i8_slice().trytes().unwrap())
            .collect();
        let body = json!({
            "command": "wereAddressesSpentFrom",
            "addresses": addresses,
        });

        let res: WereAddressesSpentFromResponseBuilder = response!(body);
        res.build().await
    }
}
