#![deny(unused_extern_crates)]
#![allow(dead_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Provides access to the Iota Client API

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

use reqwest::r#async::Response;
use serde_json::Value;
use tokio::prelude::*;
use tokio::runtime::Runtime;

use iota_validation::input_validator;

mod core;

/// IRI responses are parsed into structs contained in this module
pub mod responses;
/// Options of Argument for IOTA IRI APIs
pub mod options {
    pub use crate::attach_to_tangle::AttachOptions;
    pub use crate::find_transactions::FindTransactionsOptions;
    pub use crate::get_balances::GetBalancesOptions;
    pub use crate::get_inclusion_states::GetInclusionStatesOptions;
    pub use crate::get_transactions_to_approve::GetTransactionsToApproveOptions;
}

pub use attach_to_tangle::attach_to_tangle_local;

use crate::core::*;
use crate::responses::*;
use options::*;

type Result<T> = ::std::result::Result<T, failure::Error>;

// TODO once async/await is stable, this file needs to be updated

/// An instance of the client using IRI URI
#[derive(Debug)]
pub struct Client<'a> {
    /// URI of IRI connection
    pub uri: &'a str,
    /// Handle to the Tokio runtime
    pub runtime: Runtime,
    /// A reqwest Client to make Requests with
    pub client: reqwest::r#async::Client,
}

impl<'a> Default for Client<'a> {
    fn default() -> Client<'a> {
        Client {
            uri: "",
            runtime: Runtime::new().unwrap(),
            client: reqwest::r#async::Client::new(),
        }
    }
}

impl<'a> Client<'a> {
    /// Create a new instance of Client
    pub fn new(uri: &str) -> Client<'_> {
        Client {
            uri: uri,
            runtime: Runtime::new().unwrap(),
            client: reqwest::r#async::Client::new(),
        }
    }

    /// Add a list of neighbors to your node. It should be noted that
    /// this is only temporary, and the added neighbors will be removed
    /// from your set of neighbors after you relaunch IRI.
    /// ```
    /// use iota_client;
    /// let mut client = iota_client::Client::new("https://node01.iotatoken.nl");
    /// let resp = client.add_neighbors(&vec!["".into()]).unwrap();
    /// println!("{:?}", resp);
    /// ```
    pub fn add_neighbors(&mut self, uris: &[String]) -> Result<AddNeighborsResponse> {
        let parsed_resp: AddNeighborsResponse = self
            .runtime
            .block_on(
                add_neighbors::add_neighbors(&self.client, self.uri, uris)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
        Ok(parsed_resp)
    }

    /// Performs proof of work
    ///
    /// * `uri` - the uri used to make the request
    /// * `trunk_transaction` - trunk transaction to confirm
    /// * `branch_transaction` - branch transaction to confirm
    /// * `min_weight_magnitude` - Difficulty of PoW
    /// * `trytes` - tryes to use for PoW
    pub fn attach_to_tangle(
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

        let attach_resp: AttachToTangleResponse = self
            .runtime
            .block_on(
                attach_to_tangle::attach_to_tangle(&self.client, self.uri, options)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();

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
    pub fn broadcast_transactions(
        &mut self,
        trytes: &[String],
    ) -> Result<BroadcastTransactionsResponse> {
        ensure!(
            input_validator::is_array_of_attached_trytes(&trytes),
            "Provided trytes are not valid: {:?}",
            trytes
        );

        let parsed_response: BroadcastTransactionsResponse = self
            .runtime
            .block_on(
                broadcast_transactions::broadcast_transactions(&self.client, self.uri, trytes)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
        // let parsed_response: BroadcastTransactionsResponse = self.runtime.block_on(resp.json()).unwrap();

        if let Some(error) = parsed_response.error() {
            return Err(format_err!("{}", error));
        }
        if let Some(exception) = parsed_response.exception() {
            return Err(format_err!("{}", exception));
        }

        Ok(parsed_response)
    }

    /// Checks for consistency of given hashes, not part of the public api
    pub fn check_consistency(&mut self, hashes: &[String]) -> Result<Value> {
        for hash in hashes {
            ensure!(
                input_validator::is_hash(hash),
                "Provided hash is not valid: {:?}",
                hash
            );
        }
        let parsed: Value = self
            .runtime
            .block_on(
                check_consistency::check_consistency(&self.client, self.uri, hashes)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
        Ok(parsed)
    }

    /// Finds transactions the match any of the provided parameters
    pub fn find_transactions(
        &mut self,
        options: FindTransactionsOptions,
    ) -> Result<FindTransactionsResponse> {
        let parsed_resp: FindTransactionsResponse = self
            .runtime
            .block_on(
                find_transactions::find_transactions(&self.client, self.uri, options)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
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
    pub fn get_balances(&mut self, options: GetBalancesOptions) -> Result<GetBalancesResponse> {
        let parsed_resp: GetBalancesResponse = self
            .runtime
            .block_on(
                get_balances::get_balances(&self.client, self.uri, options)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
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
    pub fn get_inclusion_states(
        &mut self,
        options: GetInclusionStatesOptions,
    ) -> Result<GetInclusionStatesResponse> {
        ensure!(
            input_validator::is_array_of_hashes(&options.transactions),
            "Provided transactions are not valid: {:?}",
            options.transactions
        );
        ensure!(
            input_validator::is_array_of_hashes(&options.tips),
            "Provided tips are not valid: {:?}",
            options.tips
        );

        let parsed_resp: GetInclusionStatesResponse = self
            .runtime
            .block_on(
                get_inclusion_states::get_inclusion_states(&self.client, self.uri, options)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();

        if let Some(error) = parsed_resp.error() {
            return Err(format_err!("{}", error));
        }

        Ok(parsed_resp)
    }

    /// Returns the set of neighbors you are connected with, as
    /// well as their activity count. The activity counter is reset
    /// after restarting IRI.
    pub fn get_neighbors(&mut self) -> Result<GetNeighborsResponse> {
        let parsed_resp: GetNeighborsResponse = self
            .runtime
            .block_on(
                get_neighbors::get_neighbors(&self.client, self.uri)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();

        if let Some(error) = parsed_resp.error() {
            return Err(format_err!("{}", error));
        }

        Ok(parsed_resp)
    }

    /// Gets information about the specified node
    pub fn get_node_info(&mut self) -> Result<GetNodeInfoResponse> {
        let parsed_resp: GetNodeInfoResponse = self
            .runtime
            .block_on(
                get_node_info::get_node_info(&self.client, self.uri)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();

        Ok(parsed_resp)
    }

    /// Returns the list of tips
    pub fn get_tips(&mut self) -> Result<GetTipsResponse> {
        let parsed_resp: GetTipsResponse = self
            .runtime
            .block_on(get_tips::get_tips(&self.client, self.uri).and_then(|mut resp| resp.json()))
            .unwrap();
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
    pub fn get_transactions_to_approve(
        &mut self,
        options: GetTransactionsToApproveOptions<'_>,
    ) -> Result<GetTransactionsToApprove> {
        let parsed_resp: GetTransactionsToApprove = self
            .runtime
            .block_on(
                get_transactions_to_approve::get_transactions_to_approve(
                    &self.client,
                    self.uri,
                    options,
                )
                .and_then(|mut resp| resp.json()),
            )
            .unwrap();

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
    pub fn get_trytes(&mut self, hashes: &[String]) -> Result<GetTrytesResponse> {
        ensure!(
            input_validator::is_array_of_hashes(&hashes),
            "Provided hashes are not valid: {:?}",
            hashes
        );

        let parsed_resp: GetTrytesResponse = self
            .runtime
            .block_on(
                get_trytes::get_trytes(&self.client, self.uri, hashes)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
        Ok(parsed_resp)
    }

    /// Interupts an existing PoW request if you made one
    pub fn interrupt_attaching_to_tangle(&mut self) -> Result<Response> {
        let resp = self
            .runtime
            .block_on(
                interrupt_attaching_to_tangle::interrupt_attaching_to_tangle(
                    &self.client,
                    self.uri,
                ),
            )
            .unwrap();
        Ok(resp)
    }

    /// Removes a list of neighbors to your node.
    /// This is only temporary, and if you have your neighbors
    /// added via the command line, they will be retained after
    /// you restart your node.
    pub fn remove_neighbors(&mut self, uris: &[String]) -> Result<RemoveNeighborsResponse> {
        let parsed_resp: RemoveNeighborsResponse = self
            .runtime
            .block_on(
                remove_neighbors::remove_neighbors(&self.client, self.uri, uris)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
        Ok(parsed_resp)
    }

    /// Store transactions into the local storage.
    /// The trytes to be used for this call are
    /// returned by attachToTangle.
    pub fn store_transactions(&mut self, trytes: &[String]) -> Result<StoreTransactionsResponse> {
        let parsed_resp: StoreTransactionsResponse = self
            .runtime
            .block_on(
                store_transactions::store_transactions(&self.client, self.uri, trytes)
                    .and_then(|mut resp| resp.json()),
            )
            .unwrap();
        Ok(parsed_resp)
    }

    /// Check if a list of addresses was ever spent from.
    pub fn were_addresses_spent_from(
        &mut self,
        addresses: &[String],
    ) -> Result<WereAddressesSpentFromResponse> {
        let addresses: Vec<String> = addresses
            .iter()
            .filter(|address| input_validator::is_address(address))
            .map(|address| iota_signing::checksum::remove_checksum(address))
            .collect();
        ensure!(!addresses.is_empty(), "No valid addresses provided.");

        let parsed_resp: WereAddressesSpentFromResponse = self
            .runtime
            .block_on(
                were_addresses_spent_from::were_addresses_spent_from(
                    &self.client,
                    self.uri,
                    &addresses,
                )
                .and_then(|mut resp| resp.json()),
            )
            .unwrap();
        Ok(parsed_resp)
    }
}
