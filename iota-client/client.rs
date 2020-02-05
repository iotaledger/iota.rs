use reqwest::Response;
use serde_json::Value;

use iota_validation::input_validator;

use crate::core::*;
use crate::options::*;
use crate::Result;

/// An instance of the client using IRI URI
#[derive(Debug)]
pub struct Client<'a> {
    /// URI of IRI connection
    pub uri: &'a str,
    /// A reqwest Client to make Requests with
    pub client: reqwest::Client,
}

impl<'a> Default for Client<'a> {
    fn default() -> Client<'a> {
        Client {
            uri: "",
            client: reqwest::Client::new(),
        }
    }
}

impl<'a> Client<'a> {
    /// Create a new instance of Client
    pub fn new(uri: &str) -> Client<'_> {
        Client {
            uri,
            client: reqwest::Client::new(),
        }
    }

    /// Add a list of neighbors to your node. It should be noted that
    /// this is only temporary, and the added neighbors will be removed
    /// from your set of neighbors after you relaunch IRI.
    /// ```
    /// use iota_client;
    /// let mut client = iota_client::Client::new("https://nodes.devnet.iota.org");
    /// let resp = client.add_neighbors(&vec!["".into()]).unwrap();
    /// println!("{:?}", resp);
    /// ```
    pub async fn add_neighbors(&mut self, uris: &[String]) -> Result<AddNeighborsResponse> {
        let parsed_resp = add_neighbors::add_neighbors(&self.client, self.uri, uris).await?;

        Ok(parsed_resp)
    }

    /// Performs proof of work
    ///
    /// * `uri` - the uri used to make the request
    /// * `trunk_transaction` - trunk transaction to confirm
    /// * `branch_transaction` - branch transaction to confirm
    /// * `min_weight_magnitude` - Difficulty of PoW
    /// * `trytes` - tryes to use for PoW
    pub async fn attach_to_tangle(
        &mut self,
        options: AttachOptions<'_, '_, '_>,
    ) -> Result<AttachToTangleResponse> {
        ensure!(
            input_validator::is_hash(&options.trunk_transaction),
            "Provided trunk transaction is not valid: {:?}",
            options.trunk_transaction
        );
        ensure!(
            input_validator::is_hash(&options.branch_transaction),
            "Provided branch transaction is not valid: {:?}",
            options.branch_transaction
        );
        ensure!(
            input_validator::is_array_of_trytes(&options.trytes),
            "Provided trytes are not valid: {:?}",
            options.trytes
        );

        let attach_resp =
            attach_to_tangle::attach_to_tangle(&self.client, self.uri, options).await?;

        if let Some(error) = attach_resp.error() {
            return Err(format_err!("{}", error));
        }
        if let Some(exception) = attach_resp.exception() {
            return Err(format_err!("{}", exception));
        }

        Ok(attach_resp)
    }

    /// Broadcast a list of transactions to all neighbors.
    /// The input trytes for this call are provided by attachToTangle.
    pub async fn broadcast_transactions(
        &mut self,
        trytes: &[String],
    ) -> Result<BroadcastTransactionsResponse> {
        ensure!(
            input_validator::is_array_of_attached_trytes(&trytes),
            "Provided trytes are not valid: {:?}",
            trytes
        );

        let parsed_response =
            broadcast_transactions::broadcast_transactions(&self.client, self.uri, trytes).await?;

        if let Some(error) = parsed_response.error() {
            return Err(format_err!("{}", error));
        }
        if let Some(exception) = parsed_response.exception() {
            return Err(format_err!("{}", exception));
        }

        Ok(parsed_response)
    }

    /// Checks for consistency of given hashes, not part of the public api
    pub async fn check_consistency(&mut self, hashes: &[String]) -> Result<Value> {
        for hash in hashes {
            ensure!(
                input_validator::is_hash(hash),
                "Provided hash is not valid: {:?}",
                hash
            );
        }
        let parsed = check_consistency::check_consistency(&self.client, self.uri, hashes).await?;

        Ok(parsed)
    }

    /// Finds transactions the match any of the provided parameters
    pub async fn find_transactions(
        &mut self,
        options: FindTransactionsOptions,
    ) -> Result<FindTransactionsResponse> {
        let parsed_resp =
            find_transactions::find_transactions(&self.client, self.uri, options).await?;

        if let Some(error) = parsed_resp.error() {
            return Err(format_err!("{}", error));
        }

        Ok(parsed_resp)
    }

    /// Returns the balance based on the latest confirmed milestone.
    /// In addition to the balances, it also returns the referencing tips (or milestone),
    /// as well as the index with which the confirmed balance was
    /// determined. The balances is returned as a list in the same
    /// order as the addresses were provided as input.
    pub async fn get_balances(
        &mut self,
        options: GetBalancesOptions,
    ) -> Result<GetBalancesResponse> {
        ensure!(
            input_validator::is_array_of_hashes(&options.addresses),
            "Provided addresses are not valid: {:?}",
            options.addresses
        );
        let parsed_resp = get_balances::get_balances(&self.client, self.uri, options).await?;

        Ok(parsed_resp)
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
        &mut self,
        options: GetInclusionStatesOptions,
    ) -> Result<GetInclusionStatesResponse> {
        ensure!(
            input_validator::is_array_of_hashes(&options.transactions),
            "Provided transactions are not valid: {:?}",
            options.transactions
        );
        if !options.tips.is_empty() {
            ensure!(
                input_validator::is_array_of_hashes(&options.tips),
                "Provided tips are not valid: {:?}",
                options.tips
            );
        }

        let parsed_resp =
            get_inclusion_states::get_inclusion_states(&self.client, self.uri, options).await?;

        if let Some(error) = parsed_resp.error() {
            return Err(format_err!("{}", error));
        }

        Ok(parsed_resp)
    }

    /// Returns the set of neighbors you are connected with, as
    /// well as their activity count. The activity counter is reset
    /// after restarting IRI.
    pub async fn get_neighbors(&mut self) -> Result<GetNeighborsResponse> {
        let parsed_resp = get_neighbors::get_neighbors(&self.client, self.uri).await?;

        if let Some(error) = parsed_resp.error() {
            return Err(format_err!("{}", error));
        }

        Ok(parsed_resp)
    }

    /// Gets information about the specified node
    pub async fn get_node_info(&mut self) -> Result<GetNodeInfoResponse> {
        let parsed_resp = get_node_info::get_node_info(&self.client, self.uri).await?;

        Ok(parsed_resp)
    }

    /// Returns the list of tips
    pub async fn get_tips(&mut self) -> Result<GetTipsResponse> {
        let parsed_resp = get_tips::get_tips(&self.client, self.uri).await?;

        Ok(parsed_resp)
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
        &mut self,
        options: GetTransactionsToApproveOptions<'_>,
    ) -> Result<GetTransactionsToApprove> {
        let parsed_resp = get_transactions_to_approve::get_transactions_to_approve(
            &self.client,
            self.uri,
            options,
        )
        .await?;

        if let Some(error) = parsed_resp.error() {
            return Err(format_err!("{}", error));
        }
        if let Some(exception) = parsed_resp.exception() {
            return Err(format_err!("{}", exception));
        }

        Ok(parsed_resp)
    }

    /// Returns the raw transaction data (trytes) of a specific
    /// transaction. These trytes can then be easily converted
    /// into the actual transaction object. See utility functions
    /// for more details.
    pub async fn get_trytes(&mut self, hashes: &[String]) -> Result<GetTrytesResponse> {
        ensure!(
            input_validator::is_array_of_hashes(&hashes),
            "Provided hashes are not valid: {:?}",
            hashes
        );

        let parsed_resp = get_trytes::get_trytes(&self.client, self.uri, hashes).await?;

        Ok(parsed_resp)
    }

    /// Interupts an existing PoW request if you made one
    pub async fn interrupt_attaching_to_tangle(&mut self) -> Result<Response> {
        let resp =
            interrupt_attaching_to_tangle::interrupt_attaching_to_tangle(&self.client, self.uri)
                .await?;

        Ok(resp)
    }

    /// Removes a list of neighbors to your node.
    /// This is only temporary, and if you have your neighbors
    /// added via the command line, they will be retained after
    /// you restart your node.
    pub async fn remove_neighbors(&mut self, uris: &[String]) -> Result<RemoveNeighborsResponse> {
        let parsed_resp = remove_neighbors::remove_neighbors(&self.client, self.uri, uris).await?;

        Ok(parsed_resp)
    }

    /// Store transactions into the local storage.
    /// The trytes to be used for this call are
    /// returned by attachToTangle.
    pub async fn store_transactions(
        &mut self,
        trytes: &[String],
    ) -> Result<StoreTransactionsResponse> {
        ensure!(
            input_validator::is_array_of_attached_trytes(&trytes),
            "Provided trytes are not valid: {:?}",
            trytes
        );

        let parsed_resp =
            store_transactions::store_transactions(&self.client, self.uri, trytes).await?;

        Ok(parsed_resp)
    }

    /// Check if a list of addresses was ever spent from.
    pub async fn were_addresses_spent_from(
        &mut self,
        addresses: &[String],
    ) -> Result<WereAddressesSpentFromResponse> {
        let addresses: Vec<String> = addresses
            .iter()
            .filter(|address| input_validator::is_address(address))
            .map(|address| iota_signing::checksum::remove_checksum(address))
            .collect();
        ensure!(!addresses.is_empty(), "No valid addresses provided.");

        let parsed_resp = were_addresses_spent_from::were_addresses_spent_from(
            &self.client,
            self.uri,
            &addresses,
        )
        .await?;

        Ok(parsed_resp)
    }
}
