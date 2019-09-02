use chrono::prelude::*;
use reqwest::r#async::Response;
use serde_json::Value;
use tokio::prelude::*;
use tokio::runtime::Runtime;

use iota_conversion::Trinary;
use iota_model::*;
use iota_validation::input_validator;

use std::cmp;
use std::convert::TryInto;

use crate::core::*;
use crate::extended::*;
use crate::options::*;
use crate::responses::*;

use get_new_address::new_address;
use prepare_transfers::AddRemainderOptions;

pub(crate) type Result<T> = ::std::result::Result<T, failure::Error>;

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

    ///////////////////////////////TODO EXTENDED

    /// Given a seed, iterates through addresses looking for
    /// enough funds to meet specified threshold
    ///
    /// * `seed` - The wallet seed to use
    /// * `options` - See `GetInputsOptions`
    pub fn get_inputs(&mut self, seed: &str, options: GetInputsOptions) -> Result<Inputs> {
        ensure!(iota_validation::is_trytes(&seed), "Invalid seed.");
        let start = options.start.unwrap_or(0);
        let security = options.security.unwrap_or(2);

        if let Some(end) = options.end {
            ensure!(
                start <= end && end <= start + 500,
                "Invalid inputs provided."
            );
            let mut all_addresses: Vec<String> = vec![];
            for i in start..end {
                all_addresses.push((new_address(&seed, security, i, false))?);
            }
            self.get_balance_and_format(&all_addresses, start, options.threshold, security)
        } else {
            let new_address = self.get_new_address(
                seed,
                false,
                true,
                GetNewAddressOptions {
                    security: Some(security),
                    index: Some(start),
                    total: None,
                },
            )?;
            self.get_balance_and_format(&new_address, start, options.threshold, security)
        }
    }

    /// Generates a new address
    ///
    /// * `seed` - Seed used to generate new address
    /// * `checksum` - Whether or not to checksum address
    /// * `return_all` - Whether to return all generated addresses, or just the last one
    /// * `options` - See `GetNewAddressOptions`
    pub fn get_new_address(
        &mut self,
        seed: &str,
        checksum: bool,
        return_all: bool,
        options: GetNewAddressOptions,
    ) -> Result<Vec<String>> {
        let mut index = options.index.unwrap_or_default();
        let security = options.security.unwrap_or(2);
        ensure!(iota_validation::is_trytes(&seed), "Invalid seed.");
        ensure!(security > 0 && security < 4, "Invalid security.");

        let mut all_addresses: Vec<String> = Vec::new();

        match options.total {
            Some(total) => {
                ensure!(total > 0, "Invalid total.");
                for i in index..total {
                    let address = new_address(&seed, security, i, checksum)?;
                    all_addresses.push(address);
                }
                Ok(all_addresses)
            }
            None => loop {
                let new_address = new_address(&seed, security, index, checksum)?;
                if return_all {
                    all_addresses.push(new_address.clone());
                }
                index += 1;
                let new_address_vec = vec![new_address];
                let were_addr_spent = self.were_addresses_spent_from(&new_address_vec)?;
                if !were_addr_spent.state(0) {
                    let resp = self.find_transactions(FindTransactionsOptions {
                        addresses: new_address_vec.clone(),
                        ..FindTransactionsOptions::default()
                    })?;
                    if resp.take_hashes().unwrap_or_default().is_empty() {
                        if return_all {
                            return Ok(all_addresses);
                        } else {
                            return Ok(new_address_vec);
                        }
                    }
                }
            },
        }
    }

    /// Prepares a slice of transfers and converts them into a
    /// slice of tryte-encoded strings
    ///
    /// * `seed` - The wallet seed to use
    /// * `transfers` - A slice of transfers to prepare
    /// * `options` - See `PrepareTransfersOptions`
    pub fn prepare_transfers(
        &mut self,
        seed: &str,
        transfers: impl Into<Vec<Transfer>>,
        options: PrepareTransfersOptions<'_, '_>,
    ) -> Result<Vec<String>> {
        let mut transfers = transfers.into();
        let mut add_hmac = false;
        let mut added_hmac = false;

        ensure!(iota_validation::is_trytes(&seed), "Invalid seed.");
        if let Some(hmac_key) = &options.hmac_key {
            ensure!(iota_validation::is_trytes(&hmac_key), "Invalid trytes.");
            add_hmac = true;
        }
        for transfer in &mut transfers {
            if add_hmac && transfer.value > 0 {
                transfer.message = "9".repeat(243) + &transfer.message;
                added_hmac = true;
            }
            if transfer.address.len() == 90 {
                ensure!(
                    iota_signing::checksum::is_valid_checksum(&transfer.address)?,
                    "Invalid address."
                );
            }
            transfer.address = iota_signing::checksum::remove_checksum(&transfer.address);
        }
        ensure!(
            iota_validation::is_transfers_collection_valid(&transfers),
            "Invalid transfers."
        );
        let security = options.security;
        let mut bundle = Bundle::default();
        let mut total_value = 0;
        let mut signature_fragments: Vec<String> = Vec::new();
        let mut tag = String::new();

        for transfer in transfers {
            let mut signature_message_length = 1;
            if transfer.message.len() > iota_constants::MESSAGE_LENGTH {
                signature_message_length += (transfer.message.len() as f64
                    / iota_constants::MESSAGE_LENGTH as f64)
                    .floor() as usize;
                let mut msg_copy = transfer.message.to_string();
                while !msg_copy.is_empty() {
                    let mut fragment = msg_copy
                        .chars()
                        .take(iota_constants::MESSAGE_LENGTH)
                        .collect();
                    msg_copy = msg_copy
                        .chars()
                        .skip(iota_constants::MESSAGE_LENGTH)
                        .collect();
                    iota_model::right_pad_string(
                        &mut fragment,
                        iota_constants::MESSAGE_LENGTH,
                        '9',
                    );
                    signature_fragments.push(fragment);
                }
            } else {
                let mut fragment = if !transfer.message.is_empty() {
                    transfer.message.chars().take(2187).collect()
                } else {
                    String::new()
                };
                iota_model::right_pad_string(&mut fragment, iota_constants::MESSAGE_LENGTH, '9');
                signature_fragments.push(fragment);
            }
            tag = transfer.tag;
            iota_model::right_pad_string(&mut tag, iota_constants::TAG_LENGTH, '9');
            bundle.add_entry(BundleEntry {
                signature_message_length,
                address: &transfer.address,
                value: transfer.value,
                tag: &tag,
                timestamp: Utc::now().timestamp(),
            });
            total_value += transfer.value;
        }

        if total_value > 0 {
            match options.inputs {
                Some(inputs) => {
                    let input_addresses: Vec<String> = inputs
                        .inputs_list()
                        .iter()
                        .map(|input| input.address.to_string())
                        .collect();
                    let resp = self.get_balances(GetBalancesOptions {
                        addresses: input_addresses,
                        ..GetBalancesOptions::default()
                    })?;
                    let mut confirmed_inputs = Inputs::default();
                    let balances = resp.take_balances().unwrap_or_default();
                    for (i, balance) in balances.iter().enumerate() {
                        let b: i64 = balance.parse()?;
                        if b > 0 {
                            *confirmed_inputs.total_balance_mut() += b;
                            let mut confirmed_input = inputs.inputs_list()[i].clone();
                            confirmed_input.balance = b;
                            confirmed_inputs.add(confirmed_input);
                            if confirmed_inputs.total_balance() >= total_value {
                                break;
                            }
                        }
                    }
                    ensure!(
                        total_value <= confirmed_inputs.total_balance(),
                        "Not enough balance."
                    );
                    self.add_remainder(
                        &confirmed_inputs,
                        &mut bundle,
                        AddRemainderOptions {
                            seed,
                            tag: &tag,
                            remainder_address: options.remainder_address,
                            signature_fragments,
                            added_hmac,
                            hmac_key: options.hmac_key,
                            security,
                        },
                    )
                }
                None => {
                    let inputs = self.get_inputs(
                        &seed,
                        GetInputsOptions {
                            start: None,
                            end: None,
                            threshold: Some(total_value),
                            security: Some(security),
                        },
                    )?;
                    self.add_remainder(
                        &inputs,
                        &mut bundle,
                        AddRemainderOptions {
                            seed,
                            tag: &tag,
                            remainder_address: options.remainder_address,
                            signature_fragments,
                            added_hmac,
                            hmac_key: options.hmac_key,
                            security,
                        },
                    )
                }
            }
        } else {
            bundle.finalize()?;
            bundle.add_trytes(&signature_fragments);
            let mut bundle_trytes: Vec<String> = Vec::new();
            for b in bundle.iter().rev() {
                bundle_trytes.push(b.try_into()?);
            }
            Ok(bundle_trytes)
        }
    }

    /// Prepares and sends a slice of transfers
    /// This helper does everything for you, PoW and such
    ///
    /// * `transfers` - A slice of transfers to send
    /// * `seed` - The wallet seed to use
    /// * `depth` - The depth to search when looking for transactions to approve
    /// * `min_weight_magnitude` - The PoW difficulty factor (14 on mainnet, 9 on testnet)
    /// * `local_pow` - Whether or not to do local PoW
    /// * `options` - See `SendTransferOptions`
    pub fn send_transfers(
        &mut self,
        transfers: impl Into<Vec<Transfer>>,
        seed: &str,
        options: SendTransferOptions<'_, '_, '_>,
    ) -> Result<Vec<Transaction>> {
        let transfers = transfers.into();
        let trytes = self.prepare_transfers(
            seed,
            transfers,
            PrepareTransfersOptions {
                inputs: options.inputs,
                remainder_address: options.remainder_address,
                security: options.security,
                hmac_key: options.hmac_key,
            },
        )?;
        let t = self.send_trytes(
            &trytes,
            SendTrytesOptions {
                depth: options.depth,
                min_weight_magnitude: options.min_weight_magnitude,
                local_pow: options.local_pow,
                threads: options.threads,
                reference: options.reference,
            },
        )?;
        Ok(t)
    }

    /// Send trytes is a helper function that:
    ///
    /// 1. Gets transactions to approve
    /// 2. Does PoW
    /// 3. Sends your transactions to the IRI
    ///
    /// You should probably use `send_transfers`
    ///
    /// * `trytes` - A slice of strings that are tryte-encoded transactions
    /// * `depth` - The depth to search for transactions to approve
    /// * `min_weight_magnitude` - The PoW difficulty factor (14 on mainnet, 9 on testnet)
    /// * `local_pow` - Whether or not to do local PoW
    /// * `options` - See `SendTrytesOptions`
    pub fn send_trytes(
        &mut self,
        trytes: &[String],
        options: SendTrytesOptions<'_>,
    ) -> Result<Vec<Transaction>> {
        let to_approve = self.get_transactions_to_approve(GetTransactionsToApproveOptions {
            depth: options.depth,
            reference: options.reference,
        })?;
        let attach_options = AttachOptions {
            threads: options.threads,
            trunk_transaction: &to_approve
                .trunk_transaction()
                .clone()
                .ok_or_else(|| format_err!("Trunk transaction is empty"))?,
            branch_transaction: &to_approve
                .branch_transaction()
                .clone()
                .ok_or_else(|| format_err!("Branch transaction is empty"))?,
            trytes,
            ..AttachOptions::default()
        };
        let trytes_list = if options.local_pow {
            let res = attach_to_tangle::attach_to_tangle_local(attach_options)?;
            res.trytes().unwrap()
        } else {
            let attached = self.attach_to_tangle(attach_options)?;
            attached.trytes().unwrap()
        };
        self.store_and_broadcast(&trytes_list)?;
        Ok(trytes_list
            .iter()
            .map(|trytes| trytes.parse().unwrap())
            .collect())
    }

    /// Helper function that both stores, and broadcast trytes to
    /// the IRI. Trytes must have been PoW-ed.
    ///
    /// * `trytes` - PoW-ed slice of tryte-encoded transaction strings
    pub fn store_and_broadcast(&mut self, trytes: &[String]) -> Result<()> {
        self.store_transactions(&trytes)?;
        self.broadcast_transactions(&trytes)?;
        Ok(())
    }

    /// Traverses a bundle by going through trunk transactions until
    /// the bundle hash of the transaction is no longer the same.
    ///
    /// * `trunk_tx` - The trunk transaction to start searching at
    /// * `bundle_hash` - The bundle hash to compare against while searching
    /// * `bundle` - The bundle add transactions to, until hash no longer matches
    pub fn traverse_bundle<S, T>(
        &mut self,
        trunk_tx: &str,
        bundle_hash: S,
        bundle: T,
    ) -> Result<Vec<Transaction>>
    where
        S: Into<Option<String>>,
        T: Into<Vec<Transaction>>,
    {
        let mut bundle = bundle.into();
        let tryte_list = self
            .get_trytes(&[trunk_tx.into()])?
            .take_trytes()
            .unwrap_or_default();
        ensure!(!tryte_list.is_empty(), "Bundle transactions not visible");
        let trytes = &tryte_list[0];
        let tx: Transaction = trytes.parse()?;
        let tx_bundle = &tx.bundle;
        ensure!(tx.current_index == 0, "Invalid tail transaction supplied.");
        let bundle_hash = bundle_hash.into().unwrap_or_else(|| tx_bundle.clone());
        if bundle_hash != *tx_bundle {
            return Ok(bundle);
        }

        if tx.last_index == 0 && tx.current_index == 0 {
            return Ok(vec![tx]);
        }

        let trunk_tx = &tx.trunk_transaction;
        bundle.push(tx.clone());
        self.traverse_bundle(&trunk_tx, Some(bundle_hash), bundle)
    }

    /// Gets the associated bundle transactions of a transaction
    /// Validates the signatures, total sum, and bundle order
    ///
    /// * `transaction` - The transaction hash to search for
    pub fn get_bundle(&mut self, transaction: &str) -> Result<Vec<Transaction>> {
        ensure!(
            iota_validation::is_hash(&transaction),
            "Invalid transaction."
        );
        let bundle = self.traverse_bundle(&transaction, None, vec![])?;
        ensure!(
            iota_validation::is_bundle(&bundle)?,
            "Invalid bundle provided."
        );
        Ok(bundle)
    }

    fn add_remainder(
        &mut self,
        inputs: &Inputs,
        bundle: &mut Bundle,
        options: AddRemainderOptions<'_, '_, '_, '_>,
    ) -> Result<Vec<String>> {
        let mut total_transfer_value = inputs.total_balance();
        for input in inputs.inputs_list() {
            let this_balance = input.balance;
            let to_subtract = 0 - this_balance;
            let timestamp = Utc::now().timestamp();
            let address = iota_signing::checksum::remove_checksum(&input.address);

            bundle.add_entry(BundleEntry {
                signature_message_length: input.security,
                address: &address,
                value: to_subtract,
                tag: &options.tag,
                timestamp,
            });

            if this_balance >= total_transfer_value {
                let remainder = this_balance - total_transfer_value;
                if let Some(remainder_address) = &options.remainder_address {
                    if remainder > 0 {
                        bundle.add_entry(BundleEntry {
                            signature_message_length: 1,
                            address: &remainder_address,
                            value: remainder,
                            tag: &options.tag,
                            timestamp,
                        });
                        return self.sign_inputs_and_return(
                            &options.seed,
                            inputs,
                            bundle,
                            &options.signature_fragments,
                            options.added_hmac,
                            options.hmac_key,
                        );
                    }
                } else if remainder > 0 {
                    let mut start_index = 0;
                    for input in inputs.inputs_list() {
                        start_index = cmp::max(input.key_index, start_index);
                    }
                    start_index += 1;
                    let new_address = &self.get_new_address(
                        &options.seed,
                        false,
                        false,
                        GetNewAddressOptions {
                            security: Some(options.security),
                            index: Some(start_index),
                            total: None,
                        },
                    )?[0];
                    bundle.add_entry(BundleEntry {
                        signature_message_length: 1,
                        address: &new_address,
                        value: remainder,
                        tag: &options.tag,
                        timestamp: Utc::now().timestamp(),
                    });
                    return self.sign_inputs_and_return(
                        &options.seed,
                        inputs,
                        bundle,
                        &options.signature_fragments,
                        options.added_hmac,
                        options.hmac_key,
                    );
                } else {
                    return self.sign_inputs_and_return(
                        &options.seed,
                        inputs,
                        bundle,
                        &options.signature_fragments,
                        options.added_hmac,
                        options.hmac_key,
                    );
                }
            } else {
                total_transfer_value -= this_balance;
            }
        }
        Err(format_err!("Something wen't wrong..."))
    }

    fn get_balance_and_format(
        &mut self,
        addresses: &[String],
        start: usize,
        threshold: Option<i64>,
        security: usize,
    ) -> Result<Inputs> {
        let resp = self.get_balances(GetBalancesOptions {
            addresses: addresses.to_owned(),
            ..GetBalancesOptions::default()
        })?;
        let mut inputs = Inputs::default();

        let mut threshold_reached = threshold.is_none();

        let balances = resp.take_balances().unwrap_or_default();
        for (i, address) in addresses.iter().enumerate() {
            let balance: i64 = balances[i].clone().parse()?;
            if balance > 0 {
                let new_entry = Input {
                    address: address.clone(),
                    balance,
                    key_index: start + i,
                    security,
                };
                inputs.add(new_entry);
                *inputs.total_balance_mut() += balance;
                if let Some(threshold) = threshold {
                    if inputs.total_balance() >= threshold {
                        threshold_reached = true;
                    }
                }
            }
        }
        if threshold_reached {
            Ok(inputs)
        } else {
            Err(format_err!("Not enough balance."))
        }
    }

    fn sign_inputs_and_return<'b>(
        &mut self,
        seed: &str,
        inputs: &Inputs,
        bundle: &mut Bundle,
        signature_fragments: &[String],
        added_hmac: bool,
        hmac_key: Option<&'b str>,
    ) -> Result<Vec<String>> {
        bundle.finalize()?;
        bundle.add_trytes(&signature_fragments);
        for i in 0..bundle.len() {
            if bundle[i].value < 0 {
                let this_address = bundle[i].address.clone();
                let mut key_index = 0;
                let mut key_security = 0;
                for input in inputs.inputs_list() {
                    if input.address == *this_address {
                        key_index = input.key_index;
                        key_security = input.security;
                        break;
                    }
                }
                let bundle_hash = &bundle[i].bundle;
                let key = iota_signing::key(&seed.trits(), key_index, key_security)?;
                let normalized_bundle_hash = Bundle::normalized_bundle(&bundle_hash).to_vec();
                let mut normalized_bundle_fragments = [[0; 27]; 3];
                for (j, c) in normalized_bundle_hash.chunks(27).enumerate() {
                    normalized_bundle_fragments[j].copy_from_slice(c);
                }
                let first_fragment = key[0..6561].to_vec();
                let first_bundle_fragment = normalized_bundle_fragments[0];
                let first_signed_fragment =
                    iota_signing::signature_fragment(&first_bundle_fragment, &first_fragment)?;
                bundle[i].signature_fragments = first_signed_fragment.trytes()?;
                for j in 1..key_security {
                    if bundle[i + j].address == *this_address && bundle[i + j].value == 0 {
                        let next_fragment = key[6561 * j..(j + 1) * 6561].to_vec();
                        let next_bundle_fragment = normalized_bundle_fragments[j];
                        let next_signed_fragment = iota_signing::signature_fragment(
                            &next_bundle_fragment,
                            &next_fragment,
                        )?;
                        bundle[i + j].signature_fragments = next_signed_fragment.trytes()?;
                    }
                }
            }
        }
        if added_hmac {
            let hmac = iota_signing::HMAC::new(&hmac_key.unwrap_or_default());
            hmac.add_hmac(bundle)?;
        }
        let mut bundle_trytes: Vec<String> = Vec::new();
        for tx in bundle.iter().rev() {
            let tx_trytes: String = tx.try_into()?;
            bundle_trytes.push(tx_trytes);
        }
        Ok(bundle_trytes)
    }
}
