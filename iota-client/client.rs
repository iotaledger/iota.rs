//! The Client module to connect through IRI with API usages
use crate::request::*;
use crate::response::*;
use anyhow::Result;
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
        trunk_transaction: &str,
        branch_transaction: &str,
        min_weight_magnitude: usize,
        trytes: &[&str],
    ) -> Result<AttachToTangleResponse> {
        // TODO validate inputs
        let body = serde_json::to_string(&AttachToTangleRequest {
            command: "attachToTangle",
            trunk_transaction,
            branch_transaction,
            min_weight_magnitude,
            trytes,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Sends transaction trytes to a node.
    /// The input trytes for this call are provided by `attach_to_tangle`.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    /// # Parameters
    /// * `tryres` - Valid transaction trytes
    pub async fn broadcast_transactions(&self, trytes: &[&str]) -> Result<ErrorResponse> {
        // TODO validate trytes
        let body = serde_json::to_string(&TrytesRequest {
            command: "broadcastTransactions",
            trytes,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Checks the consistency of transactions. A consistent transaction is one where the following statements are true:
    /// * The node isn't missing the transaction's branch or trunk transactions
    /// * The transaction's bundle is valid
    /// * The transaction's branch and trunk transactions are valid
    /// # Parameters
    /// * `tails` - Transaction hashes to check
    pub async fn check_consistency(&self, tails: &[&str]) -> Result<ConsistencyResponse> {
        // TODO validate hashes
        let body = serde_json::to_string(&TailsRequest {
            command: "checkConsistency",
            tails,
        })?;
        let res = response!(self, body);

        Ok(res)
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
        bundles: Option<&[&str]>,
        addresses: Option<&[&str]>,
        tags: Option<&[&str]>,
        approvees: Option<&[&str]>,
    ) -> Result<FindTransactionsResponse> {
        // TODO validate inputs
        let body = serde_json::to_string(&FindTransactionsRequest {
            command: "findTransactions",
            bundles,
            addresses,
            tags,
            approvees,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    // TODO getNodeAPIConfiguration
    // Gets a node's API configuration settings.

    /// Gets the confirmed balance of an address.
    /// If the tips parameter is missing, the returned balance is correct as of the latest confirmed milestone.
    /// This endpoint returns data only if the node is synchronized.
    /// # Parameters
    /// * `addresses` - Address for which to get the balance (do not include the checksum)
    /// * `threshold` - (Optional) Confirmation threshold between 0 and 100, default is 100.
    /// * `tips` - (Optional) Tips whose history of transactions to traverse to find the balance
    pub async fn get_balances(
        &self,
        addresses: &[&str],
        threshold: Option<u8>,
        tips: Option<&[&str]>,
    ) -> Result<GetBalancesResponse> {
        // TODO validate address and tips
        let threshold = match threshold {
            Some(i) => i,
            None => 100,
        };
        let body = serde_json::to_string(&GetBalancesRequest {
            command: "getBalances",
            addresses,
            threshold,
            tips,
        })?;
        let res = response!(self, body);

        Ok(res)
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
        transactions: &[&str],
        tips: Option<&[&str]>,
    ) -> Result<GetInclusionStatesResponse> {
        let body = serde_json::to_string(&GetInclusionStatesRequest {
            command: "getInclusionStates",
            transactions,
            tips,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    // TODO getMissingTransactions
    // Gets all transaction hashes that a node is currently requesting from its neighbors.

    /// Gets a node's neighbors and their activity.
    pub async fn get_neighbors(&self) -> Result<GetNeighborsResponse> {
        let body = serde_json::to_string(&SingleRequest {
            command: "getNeighbors",
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
        reference: Option<&str>,
    ) -> Result<GTTAResponse> {
        // TODO validate reference
        let body = serde_json::to_string(&GTTARequest {
            command: "getTransactionsToApprove",
            depth,
            reference,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Gets a transaction's contents in trytes.
    /// # Parameters
    /// * `hashes` - Transaction hashes
    pub async fn get_trytes(&self, hashes: &[&str]) -> Result<GetTrytesResponse> {
        let body = serde_json::to_string(&HashesRequest {
            command: "getTrytes",
            hashes,
        })?;
        let res = response!(self, body);

        Ok(res)
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
    pub async fn store_transactions(&self, trytes: &[&str]) -> Result<ErrorResponse> {
        // TODO validate trytes
        let body = serde_json::to_string(&TrytesRequest {
            command: "storeTransactions",
            trytes,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Checks if an address was ever withdrawn from, either in the current epoch or in any previous epochs.
    /// If an address has a pending transaction, it's also considered 'spent'.
    ///
    /// * `address` - addresses to check (do not include the checksum)
    pub async fn were_addresses_spent_from(
        &self,
        addresses: &[&str],
    ) -> Result<WereAddressesSpentFromResponse> {
        // TODO validate addresses
        let body = serde_json::to_string(&AddressesRequest {
            command: "wereAddressesSpentFrom",
            addresses,
        })?;
        let res = response!(self, body);

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_123() {
        let client = Client::new("https://nodes.thetangle.org:443");
        let res = client
            .get_transactions_to_approve(3, None)
            .await
            .expect("?????");
        dbg!(res);
    }
}
