//! The Client module to connect through IRI with API usages

//use reqwest::Response;

//use iota_validation::input_validator;
use crate::request::*;
use crate::response::*;
use anyhow::Result;

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
    pub async fn add_neighbors(&self, uris: &[String]) -> Result<AddNeighborsResponse> {
        // TODO validate uris
        let body = serde_json::to_string(&UrisRequest {
            command: "addNeighbors",
            uris,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Performs proof of work
    ///
    /// * `trunk_transaction` - trunk transaction to confirm
    /// * `branch_transaction` - branch transaction to confirm
    /// * `min_weight_magnitude` - Difficulty of PoW
    /// * `trytes` - tryes to use for PoW
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

    /// Broadcast a list of transactions to all neighbors.
    /// The input trytes for this call are provided by attachToTangle.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    pub async fn broadcast_transactions(&self, trytes: &[String]) -> Result<ErrorResponse> {
        // TODO validate trytes
        let body = serde_json::to_string(&TrytesRequest {
            command: "broadcastTransactions",
            trytes,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Checks for consistency of given hashes, not part of the public api
    pub async fn check_consistency(&self, hashes: &[String]) -> Result<ConsistencyResponse> {
        // TODO validate hashes
        let body = serde_json::to_string(&HashesRequest {
            command: "checkConsistency",
            hashes,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Finds transactions the match any of the provided parameters
    pub async fn find_transactions(
        &self,
        bundles: Option<Vec<String>>,
        addresses: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        approvees: Option<Vec<String>>,
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

    /// Returns the balance based on the latest confirmed milestone.
    /// In addition to the balances, it also returns the referencing tips (or milestone),
    /// as well as the index with which the confirmed balance was
    /// determined. The balances is returned as a list in the same
    /// order as the addresses were provided as input.
    pub async fn get_balances(
        &self,
        addresses: Vec<String>,
        threshold: Option<u8>,
        tips: Option<Vec<String>>,
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

    /// Get the inclusion states of a set of transactions. This is
    /// for determining if a transaction was accepted and confirmed
    /// by the network or not. You can search for multiple tips (and
    /// thus, milestones) to get past inclusion states of transactions.
    ///
    /// This API call simply returns a list of boolean values in the
    /// same order as the transaction list you submitted, thus you get
    /// a true/false whether a transaction is confirmed or not.
    pub async fn get_inclusion_states(
        &self,
        transactions: Vec<String>,
        tips: Option<Vec<String>>,
    ) -> Result<GetInclusionStatesResponse> {
        let body = serde_json::to_string(&GetInclusionStatesRequest {
            command: "getInclusionStates",
            transactions,
            tips,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Returns the set of neighbors you are connected with, as
    /// well as their activity count. The activity counter is reset
    /// after restarting IRI.
    pub async fn get_neighbors(&self) -> Result<GetNeighborsResponse> {
        let body = serde_json::to_string(&SingleRequest {
            command: "getNeighbors",
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Gets information about the specified node
    pub async fn get_node_info(&self) -> Result<GetNodeInfoResponse> {
        let body = serde_json::to_string(&SingleRequest {
            command: "getNodeInfo",
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Returns the list of tips
    pub async fn get_tips(&self) -> Result<GetTipsResponse> {
        let body = serde_json::to_string(&SingleRequest { command: "getTips" })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Tip selection which returns `trunkTransaction` and
    /// `branchTransaction`. The input value depth determines
    /// how many milestones to go back to for finding the
    /// transactions to approve. The higher your depth value,
    /// the more work you have to do as you are confirming more
    /// transactions. If the depth is too large (usually above 15,
    /// it depends on the node's configuration) an error will be
    /// returned. The reference is an optional hash of a transaction
    /// you want to approve. If it can't be found at the specified
    /// depth then an error will be returned.
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

    /// Returns the raw transaction data (trytes) of a specific
    /// transaction. These trytes can then be easily converted
    /// into the actual transaction object. See utility functions
    /// for more details.
    pub async fn get_trytes(&self, hashes: &[String]) -> Result<GetTrytesResponse> {
        let body = serde_json::to_string(&HashesRequest {
            command: "getTrytes",
            hashes,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Interupts an existing PoW request if you made one
    /// The response is empty no matter what.
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
    pub async fn remove_neighbors(&self, uris: &[String]) -> Result<RemoveNeighborsResponse> {
        // TODO validate uris
        let body = serde_json::to_string(&UrisRequest {
            command: "removeNeighbors",
            uris,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Store transactions into the local storage.
    /// The trytes to be used for this call are
    /// returned by attachToTangle.
    /// Response only contains errors and exceptions, it would be `None` if the call success.
    pub async fn store_transactions(&self, trytes: &[String]) -> Result<ErrorResponse> {
        // TODO validate trytes
        let body = serde_json::to_string(&TrytesRequest {
            command: "storeTransactions",
            trytes,
        })?;
        let res = response!(self, body);

        Ok(res)
    }

    /// Check if a list of addresses was ever spent from.
    pub async fn were_addresses_spent_from(
        &self,
        addresses: &[String],
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
