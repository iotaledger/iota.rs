//! The Client module to connect through IRI with API usages
use crate::request::*;
use crate::response::*;
use crate::util::tx_trytes;
use anyhow::Result;
use bee_bundle::{Address, Hash, Tag, Transaction};
use iota_conversion::Trinary;
use reqwest::Url;

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
    uri: &'a str,
    /// A reqwest Client to make Requests with
    client: reqwest::Client,
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
    /// * `uris` - Slices of neighbor URIs(`&str`) to add
    pub async fn add_neighbors(&self, uris: &[&str]) -> Result<AddNeighborsResponse> {
        for uri in uris {
            match Url::parse(uri)?.scheme() {
                "tcp" | "udp" => (),
                _ => return Err(anyhow!("Uri scheme should be either tcp or udp")),
            }
        }
        let body = serde_json::to_string(&UrisRequest {
            command: "addNeighbors",
            uris,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Does proof of work for the given transaction trytes.
    /// The `branch_transaction` and `trunk_transaction` parameters are returned
    /// from the `get_transactions_to_approve` method.
    /// # Parameters
    /// * `trunk_transaction` - trunk transaction hash
    /// * `branch_transaction` - branch transaction hash
    /// * `min_weight_magnitude` - Difficulty of PoW
    /// * `trytes` - Slice of transaction trytes. When sending transactions in a bundle,
    /// make sure that the trytes of the last transaction in the bundle are in index 0 of the array.
    pub async fn attach_to_tangle(
        &self,
        trunk_transaction: &Hash,
        branch_transaction: &Hash,
        min_weight_magnitude: usize,
        trytes: &[Transaction],
    ) -> Result<AttachToTangleResponse> {
        let body = serde_json::to_string(&AttachToTangleRequest {
            command: "attachToTangle",
            trunk_transaction: &trunk_transaction.as_bytes().trytes().unwrap(),
            branch_transaction: &branch_transaction.as_bytes().trytes().unwrap(),
            min_weight_magnitude,
            trytes: trytes
                .iter()
                .map(|tx| tx_trytes(tx))
                .collect::<Vec<String>>(),
        })?;
        let res: AttachToTangleResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Sends transaction trytes to a node.
    /// The input trytes for this call are provided by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * `tryres` - Valid transaction trytes
    pub async fn broadcast_transactions(&self, trytes: &[Transaction]) -> Result<()> {
        let body = serde_json::to_string(&TrytesRequest {
            command: "broadcastTransactions",
            trytes: trytes
                .iter()
                .map(|tx| tx_trytes(tx))
                .collect::<Vec<String>>(),
        })?;
        let res: ErrorResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Checks the consistency of transactions. A consistent transaction is one where the following statements are true:
    /// * The node isn't missing the transaction's branch or trunk transactions
    /// * The transaction's bundle is valid
    /// * The transaction's branch and trunk transactions are valid
    /// # Parameters
    /// * `tails` - Transaction hashes to check
    pub async fn check_consistency(&self, tails: &[Hash]) -> Result<ConsistencyResponse> {
        let body = serde_json::to_string(&TailsRequest {
            command: "checkConsistency",
            tails: &tails
                .iter()
                .map(|x| x.as_bytes().trytes().unwrap())
                .collect::<Vec<String>>(),
        })?;
        let res: ConsistencyResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Finds transactions that contain the given values in their transaction fields.
    /// The parameters define the transaction fields to search for, including bundles, addresses, tags, and approvees.
    /// Using multiple transaction fields, returns transactions hashes at the intersection of those values.
    /// # Parameters
    /// * `bundles` - (Optional) Bundle hashes to search for
    /// * `addresses` - (Optional) Addresses to search for (do not include the checksum)
    /// * `tags` - (Optional) Tags to search for
    /// * `approvves` - (Optional) Child transactions to search for
    pub async fn find_transactions(
        &self,
        bundles: Option<&[Hash]>,
        addresses: Option<&[Address]>,
        tags: Option<&[Tag]>,
        approvees: Option<&[Hash]>,
    ) -> Result<FindTransactionsResponse> {
        let bundles = match bundles {
            Some(b) => b.iter().map(|h| h.as_bytes().trytes().unwrap()).collect(),
            None => Vec::new(),
        };
        let addresses = match addresses {
            Some(a) => a.iter().map(|h| h.as_bytes().trytes().unwrap()).collect(),
            None => Vec::new(),
        };
        let tags = match tags {
            Some(t) => t.iter().map(|h| h.as_bytes().trytes().unwrap()).collect(),
            None => Vec::new(),
        };
        let approvees = match approvees {
            Some(a) => a.iter().map(|h| h.as_bytes().trytes().unwrap()).collect(),
            None => Vec::new(),
        };
        let body = serde_json::to_string(&FindTransactionsRequest {
            command: "findTransactions",
            bundles,
            addresses,
            tags,
            approvees,
        })?;
        let res: FindTransactionsResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Gets the confirmed balance of an address.
    /// If the tips parameter is missing, the returned balance is correct as of the latest confirmed milestone.
    /// This endpoint returns data only if the node is synchronized.
    /// # Parameters
    /// * `addresses` - Address for which to get the balance (do not include the checksum)
    /// * `threshold` - (Optional) Confirmation threshold between 0 and 100, default is 100.
    /// * `tips` - (Optional) Tips whose history of transactions to traverse to find the balance
    pub async fn get_balances(
        &self,
        addresses: &[Address],
        threshold: Option<u8>,
        tips: Option<&[Hash]>,
    ) -> Result<GetBalancesResponse> {
        let threshold = match threshold {
            Some(i) => i,
            None => 100,
        };
        let tips = match tips {
            Some(t) => t.iter().map(|h| h.as_bytes().trytes().unwrap()).collect(),
            None => Vec::new(),
        };
        let body = serde_json::to_string(&GetBalancesRequest {
            command: "getBalances",
            addresses: addresses
                .iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
            threshold,
            tips,
        })?;
        let res: GetBalancesResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Gets the inclusion states of a set of transactions.
    /// This endpoint determines if a transaction is confirmed by the network (referenced by a valid milestone).
    /// You can search for multiple tips (and thus, milestones) to get past inclusion states of transactions.
    /// This endpoint returns data only if the node is synchronized.
    /// # Parameters
    /// * `transactions` - List of transaction hashes for which you want to get the inclusion state
    /// * `tips` - (Optional) List of tip transaction hashes (including milestones) you want to search for
    pub async fn get_inclusion_states(
        &self,
        transactions: &[Transaction],
        tips: Option<&[Hash]>,
    ) -> Result<GetInclusionStatesResponse> {
        let tips = match tips {
            Some(t) => t.iter().map(|h| h.as_bytes().trytes().unwrap()).collect(),
            None => Vec::new(),
        };
        let body = serde_json::to_string(&GetInclusionStatesRequest {
            command: "getInclusionStates",
            transactions: transactions
                .iter()
                .map(|tx| tx_trytes(tx))
                .collect::<Vec<String>>(),
            tips,
        })?;
        let res: GetInclusionStatesResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Gets all transaction hashes that a node is currently requesting from its neighbors.
    pub async fn get_missing_transactions(&self) -> Result<GetTipsResponse> {
        let body = serde_json::to_string(&SingleRequest {
            command: "getMissingTransactions",
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Gets a node's neighbors and their activity.
    pub async fn get_neighbors(&self) -> Result<GetNeighborsResponse> {
        let body = serde_json::to_string(&SingleRequest {
            command: "getNeighbors",
        })?;
        let res: GetNeighborsResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Gets a node's API configuration settings.
    pub async fn get_node_api_configuration(&self) -> Result<GetNodeAPIConfigurationResponse> {
        let body = serde_json::to_string(&SingleRequest {
            command: "getNodeAPIConfiguration",
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Gets information about a node.
    pub async fn get_node_info(&self) -> Result<GetNodeInfoResponse> {
        let body = serde_json::to_string(&SingleRequest {
            command: "getNodeInfo",
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Gets tip transaction hashes from a node.
    pub async fn get_tips(&self) -> Result<GetTipsResponse> {
        let body = serde_json::to_string(&SingleRequest { command: "getTips" })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Gets two consistent tip transaction hashes to use as branch/trunk transactions.
    /// This endpoint returns data only if the node is synchronized.
    /// # Parameters
    /// * `depth` - Number of milestones to go back to start the tip selection algorithm. Default is 3.
    /// * `reference` - (Optional) Transaction hash from which to start the weighted random walk.
    /// Use this parameter to make sure the returned tip transaction hashes approve a given reference transaction
    pub async fn get_transactions_to_approve(
        &self,
        depth: usize,
        reference: Option<&Hash>,
    ) -> Result<GTTAResponse> {
        let reference = match reference {
            Some(t) => t.as_bytes().trytes().unwrap(),
            None => "".to_owned(),
        };
        let body = serde_json::to_string(&GTTARequest {
            command: "getTransactionsToApprove",
            depth,
            reference: &reference,
        })?;
        let res: GTTAResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Gets a transaction's contents in trytes.
    /// # Parameters
    /// * `hashes` - Transaction hashes
    pub async fn get_trytes(&self, hashes: &[Hash]) -> Result<GetTrytesResponse> {
        let body = serde_json::to_string(&HashesRequest {
            command: "getTrytes",
            hashes: hashes
                .iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
        })?;
        let res: GetTrytesResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Aborts the process that's started by the `attach_to_tangle` method.
    pub async fn interrupt_attaching_to_tangle(&self) -> Result<()> {
        let body = serde_json::to_string(&SingleRequest {
            command: "interruptAttachingToTangle",
        })?;
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

    /// Removes a list of neighbors to your node.
    /// This is only temporary, and if you have your neighbors
    /// added via the command line, they will be retained after
    /// you restart your node.
    /// # Parameters
    /// * `uris` - Slice of neighbor URIs(`&str`) to remove
    pub async fn remove_neighbors(&self, uris: &[&str]) -> Result<RemoveNeighborsResponse> {
        for uri in uris {
            match Url::parse(uri)?.scheme() {
                "tcp" | "udp" => (),
                _ => return Err(anyhow!("Uri scheme should be either tcp or udp")),
            }
        }
        let body = serde_json::to_string(&UrisRequest {
            command: "removeNeighbors",
            uris,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Store transactions into the local storage.
    /// The trytes to be used for this call are returned by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * `trytes` - Transaction trytes
    pub async fn store_transactions(&self, trytes: &[Transaction]) -> Result<()> {
        let body = serde_json::to_string(&TrytesRequest {
            command: "storeTransactions",
            trytes: trytes
                .iter()
                .map(|tx| tx_trytes(tx))
                .collect::<Vec<String>>(),
        })?;
        let res: ErrorResponseBuilder = response!(self, body);

        res.build().await
    }

    /// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
    /// If an address has a pending transaction, it's also considered 'spent'.
    ///
    /// * `address` - addresses to check (do not include the checksum)
    pub async fn were_addresses_spent_from(
        &self,
        addresses: &[Address],
    ) -> Result<WereAddressesSpentFromResponse> {
        let body = serde_json::to_string(&AddressesRequest {
            command: "wereAddressesSpentFrom",
            addresses: addresses
                .iter()
                .map(|h| h.as_bytes().trytes().unwrap())
                .collect(),
        })?;
        let res: WereAddressesSpentFromResponseBuilder = response!(self, body);

        res.build().await
    }
}
