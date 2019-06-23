#![deny(unused_extern_crates)]
#![allow(dead_code)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::sync::Mutex;

use reqwest::r#async::Response;
use serde_json::Value;
use tokio::runtime::Runtime;

use iota_validation::input_validator;

mod add_neighbors;
mod attach_to_tangle;
mod broadcast_transactions;
mod check_consistency;
mod find_transactions;
mod get_balances;
mod get_inclusion_states;
mod get_neighbors;
mod get_node_info;
mod get_tips;
mod get_transactions_to_approve;
mod get_trytes;
mod interrupt_attaching_to_tangle;
mod remove_neighbors;
/// IRI responses are parsed into typed structs contained in this module
pub mod responses;
mod store_transactions;
mod were_addresses_spent_from;

use crate::responses::*;

pub use attach_to_tangle::{attach_to_tangle_local, AttachOptions};
pub use find_transactions::FindTransactionsOptions;
pub use get_balances::GetBalancesOptions;
pub use get_inclusion_states::GetInclusionStatesOptions;
pub use get_transactions_to_approve::GetTransactionsToApproveOptions;

type Result<T> = ::std::result::Result<T, failure::Error>;

// TODO once async/await merges, this file needs to be updated

lazy_static! {
    static ref CLIENT: reqwest::r#async::Client = { reqwest::r#async::Client::new() };
    static ref RT: Mutex<Runtime> = { Mutex::new(Runtime::new().unwrap()) };
}

/// Add a list of neighbors to your node. It should be noted that
/// this is only temporary, and the added neighbors will be removed
/// from your set of neighbors after you relaunch IRI.
pub fn add_neighbors(uri: String, uris: Vec<String>) -> Result<AddNeighborsResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(add_neighbors::add_neighbors(&*CLIENT, uri, uris))
        .unwrap();
    let parsed_resp: AddNeighborsResponse = runtime.block_on(resp.json()).unwrap();
    Ok(parsed_resp)
}

/// Performs proof of work
///
/// * `uri` - the uri used to make the request
/// * `trunk_transaction` - trunk transaction to confirm
/// * `branch_transaction` - branch transaction to confirm
/// * `min_weight_magnitude` - Difficulty of PoW
/// * `trytes` - tryes to use for PoW
pub fn attach_to_tangle(uri: String, options: AttachOptions) -> Result<AttachToTangleResponse> {
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

    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(attach_to_tangle::attach_to_tangle(&*CLIENT, uri, options))
        .unwrap();
    let attach_resp: AttachToTangleResponse = runtime.block_on(resp.json()).unwrap();

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
    uri: String,
    trytes: Vec<String>,
) -> Result<BroadcastTransactionsResponse> {
    ensure!(
        input_validator::is_array_of_attached_trytes(&trytes),
        "Provided trytes are not valid: {:?}",
        trytes
    );

    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(broadcast_transactions::broadcast_transactions(
            &*CLIENT, uri, trytes,
        ))
        .unwrap();
    let parsed_response: BroadcastTransactionsResponse = runtime.block_on(resp.json()).unwrap();

    if let Some(error) = parsed_response.error() {
        return Err(format_err!("{}", error));
    }
    if let Some(exception) = parsed_response.exception() {
        return Err(format_err!("{}", exception));
    }

    Ok(parsed_response)
}

/// Checks for consistency of given hashes, not part of the public api
pub fn check_consistency(uri: String, hashes: Vec<String>) -> Result<Value> {
    for hash in &hashes {
        ensure!(
            input_validator::is_hash(hash),
            "Provided hash is not valid: {:?}",
            hash
        );
    }
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(check_consistency::check_consistency(&*CLIENT, uri, hashes))
        .unwrap();
    let parsed: Value = runtime.block_on(resp.json()).unwrap();
    Ok(parsed)
}

/// Finds transactions the match any of the provided parameters
pub fn find_transactions(
    uri: String,
    options: FindTransactionsOptions,
) -> Result<FindTransactionsResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(find_transactions::find_transactions(&*CLIENT, uri, options))
        .unwrap();
    let parsed_resp: FindTransactionsResponse = runtime.block_on(resp.json()).unwrap();
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
pub fn get_balances(uri: String, options: GetBalancesOptions) -> Result<GetBalancesResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(get_balances::get_balances(&*CLIENT, uri, options))
        .unwrap();
    let parsed_resp: GetBalancesResponse = runtime.block_on(resp.json()).unwrap();
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
    uri: String,
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

    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(get_inclusion_states::get_inclusion_states(
            &*CLIENT, uri, options,
        ))
        .unwrap();
    let parsed_resp: GetInclusionStatesResponse = runtime.block_on(resp.json()).unwrap();

    if let Some(error) = parsed_resp.error() {
        return Err(format_err!("{}", error));
    }

    Ok(parsed_resp)
}

/// Returns the set of neighbors you are connected with, as
/// well as their activity count. The activity counter is reset
/// after restarting IRI.
pub fn get_neighbors(uri: String) -> Result<GetNeighborsResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(get_neighbors::get_neighbors(&*CLIENT, uri))
        .unwrap();
    let parsed_resp: GetNeighborsResponse = runtime.block_on(resp.json()).unwrap();

    if let Some(error) = parsed_resp.error() {
        return Err(format_err!("{}", error));
    }

    Ok(parsed_resp)
}

/// Gets information about the specified node
pub fn get_node_info(uri: String) -> Result<GetNodeInfoResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(get_node_info::get_node_info(&*CLIENT, uri))
        .unwrap();
    let parsed_resp: GetNodeInfoResponse = runtime.block_on(resp.json()).unwrap();

    Ok(parsed_resp)
}

/// Returns the list of tips
pub fn get_tips(uri: String) -> Result<GetTipsResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime.block_on(get_tips::get_tips(&*CLIENT, uri)).unwrap();
    let parsed_resp: GetTipsResponse = runtime.block_on(resp.json()).unwrap();

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
    uri: String,
    options: GetTransactionsToApproveOptions,
) -> Result<GetTransactionsToApprove> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(get_transactions_to_approve::get_transactions_to_approve(
            &*CLIENT, uri, options,
        ))
        .unwrap();
    let parsed_resp: GetTransactionsToApprove = runtime.block_on(resp.json()).unwrap();

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
pub fn get_trytes(uri: String, hashes: Vec<String>) -> Result<GetTrytesResponse> {
    ensure!(
        input_validator::is_array_of_hashes(&hashes),
        "Provided hashes are not valid: {:?}",
        hashes
    );
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(get_trytes::get_trytes(&*CLIENT, uri, hashes))
        .unwrap();
    let parsed_resp: GetTrytesResponse = runtime.block_on(resp.json()).unwrap();
    Ok(parsed_resp)
}

/// Interupts an existing PoW request if you made one
pub fn interrupt_attaching_to_tangle(uri: String) -> Result<Response> {
    let mut runtime = RT.lock().unwrap();
    let resp = runtime
        .block_on(interrupt_attaching_to_tangle::interrupt_attaching_to_tangle(&*CLIENT, uri))
        .unwrap();
    Ok(resp)
}

/// Removes a list of neighbors to your node.
/// This is only temporary, and if you have your neighbors
/// added via the command line, they will be retained after
/// you restart your node.
pub fn remove_neighbors(uri: String, uris: Vec<String>) -> Result<RemoveNeighborsResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(remove_neighbors::remove_neighbors(&*CLIENT, uri, uris))
        .unwrap();
    let parsed_resp: RemoveNeighborsResponse = runtime.block_on(resp.json()).unwrap();
    Ok(parsed_resp)
}

/// Store transactions into the local storage.
/// The trytes to be used for this call are
/// returned by attachToTangle.
pub fn store_transactions(uri: String, trytes: Vec<String>) -> Result<StoreTransactionsResponse> {
    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(store_transactions::store_transactions(
            &*CLIENT, uri, trytes,
        ))
        .unwrap();
    let parsed_resp: StoreTransactionsResponse = runtime.block_on(resp.json()).unwrap();
    Ok(parsed_resp)
}

/// Check if a list of addresses was ever spent from.
pub fn were_addresses_spent_from(
    uri: String,
    addresses: Vec<String>,
) -> Result<WereAddressesSpentFromResponse> {
    let addresses: Vec<String> = addresses
        .iter()
        .filter(|address| input_validator::is_address(address))
        .map(|address| iota_signing::checksum::remove_checksum(address))
        .collect();
    ensure!(!addresses.is_empty(), "No valid addresses provided.");

    let mut runtime = RT.lock().unwrap();
    let mut resp = runtime
        .block_on(were_addresses_spent_from::were_addresses_spent_from(
            &*CLIENT, uri, addresses,
        ))
        .unwrap();
    let parsed_resp: WereAddressesSpentFromResponse = runtime.block_on(resp.json()).unwrap();
    Ok(parsed_resp)
}
