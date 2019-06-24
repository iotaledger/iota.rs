use std::cmp;
use std::time::Duration;

use chrono::prelude::*;
use iota_conversion::Trinary;
use reqwest::Client;

use crate::Result;
use iota_client::*;
use iota_model::*;
use options::*;

use std::convert::TryInto;

/// Generates a new address
///
/// * `seed` - Seed used to generate new address
/// * `security` - Security factor 1-3 with 3 being most secure
/// * `index` - How many iterations of generating to skip
/// * `checksum` - Whether or not to checksum address
pub fn new_address(seed: &str, security: usize, index: usize, checksum: bool) -> Result<String> {
    let key = iota_signing::key(&seed.trits(), index, security)?;
    let digests = iota_signing::digests(&key)?;
    let address_trits = iota_signing::address(&digests)?;
    let mut address = address_trits.trytes()?;
    if checksum {
        address = iota_signing::checksum::add_checksum(&address)?;
    }
    Ok(address)
}

/// An instance of the api using the same IRI URI throughout
#[derive(Clone, Debug)]
pub struct API {
    uri: String,
    client: reqwest::Client,
}

struct AddRemainderOptions<'a, 'b, 'c, 'd> {
    pub seed: &'a str,
    pub tag: &'b str,
    pub remainder_address: Option<&'c str>,
    pub signature_fragments: Vec<String>,
    pub added_hmac: bool,
    pub hmac_key: Option<&'d str>,
    pub security: usize,
}

impl API {
    /// Create a new instance of the API
    ///
    /// * `uri` - The uri to use for all querys, currently only https IRI node are supported
    pub fn new(uri: &str) -> API {
        API {
            uri: uri.to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap(),
        }
    }

    /// Generates a new address
    ///
    /// * `seed` - Seed used to generate new address
    /// * `checksum` - Whether or not to checksum address
    /// * `return_all` - Whether to return all generated addresses, or just the last one
    /// * `options` - See `GetNewAddressOptions`
    pub fn get_new_address(
        &self,
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
                let were_addr_spent =
                    iota_client::were_addresses_spent_from(&self.uri, &new_address_vec)?;
                if !were_addr_spent.state(0) {
                    let resp = iota_client::find_transactions(
                        &self.uri,
                        FindTransactionsOptions {
                            addresses: new_address_vec.clone(),
                            ..FindTransactionsOptions::default()
                        },
                    )?;
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
        &self,
        trytes: Vec<String>,
        options: SendTrytesOptions,
    ) -> Result<Vec<Transaction>> {
        let to_approve = iota_client::get_transactions_to_approve(
            &self.uri,
            GetTransactionsToApproveOptions {
                depth: options.depth,
                reference: options.reference,
            },
        )?;
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
            let res = iota_client::attach_to_tangle_local(attach_options)?;
            res.trytes().unwrap()
        } else {
            let attached = iota_client::attach_to_tangle(&self.uri, attach_options)?;
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
    pub fn store_and_broadcast(&self, trytes: &[String]) -> Result<()> {
        iota_client::store_transactions(&self.uri, &trytes)?;
        iota_client::broadcast_transactions(&self.uri, &trytes)?;
        Ok(())
    }

    /// Given a seed, iterates through addresses looking for
    /// enough funds to meet specified threshold
    ///
    /// * `seed` - The wallet seed to use
    /// * `options` - See `GetInputsOptions`
    pub fn get_inputs(&self, seed: &str, options: GetInputsOptions) -> Result<Inputs> {
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

    fn get_balance_and_format(
        &self,
        addresses: &[String],
        start: usize,
        threshold: Option<i64>,
        security: usize,
    ) -> Result<Inputs> {
        let resp = iota_client::get_balances(
            &self.uri,
            GetBalancesOptions {
                addresses: addresses.to_owned(),
                ..GetBalancesOptions::default()
            },
        )?;
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

    /// Prepares a slice of transfers and converts them into a
    /// slice of tryte-encoded strings
    ///
    /// * `seed` - The wallet seed to use
    /// * `transfers` - A slice of transfers to prepare
    /// * `options` - See `PrepareTransfersOptions`
    pub fn prepare_transfers(
        &self,
        seed: &str,
        mut transfers: Vec<Transfer>,
        options: PrepareTransfersOptions,
    ) -> Result<Vec<String>> {
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
                    let resp = iota_client::get_balances(
                        &self.uri,
                        GetBalancesOptions {
                            addresses: input_addresses,
                            ..GetBalancesOptions::default()
                        },
                    )?;
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
        &self,
        transfers: Vec<Transfer>,
        seed: &str,
        options: SendTransferOptions,
    ) -> Result<Vec<Transaction>> {
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
            trytes,
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

    /// Traverses a bundle by going through trunk transactions until
    /// the bundle hash of the transaction is no longer the same.
    ///
    /// * `trunk_tx` - The trunk transaction to start searching at
    /// * `bundle_hash` - The bundle hash to compare against while searching
    /// * `bundle` - The bundle add transactions to, until hash no longer matches
    pub fn traverse_bundle<S, T>(
        &self,
        trunk_tx: &str,
        bundle_hash: S,
        bundle: T,
    ) -> Result<Vec<Transaction>>
    where
        S: Into<Option<String>>,
        T: Into<Vec<Transaction>>,
    {
        let mut bundle = bundle.into();
        let tryte_list = iota_client::get_trytes(&self.uri, &[trunk_tx.into()])?
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
    pub fn get_bundle(&self, transaction: &str) -> Result<Vec<Transaction>> {
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
        &self,
        inputs: &Inputs,
        bundle: &mut Bundle,
        options: AddRemainderOptions,
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

    fn sign_inputs_and_return<'a>(
        &self,
        seed: &str,
        inputs: &Inputs,
        bundle: &mut Bundle,
        signature_fragments: &[String],
        added_hmac: bool,
        hmac_key: Option<&'a str>,
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

pub mod options {
    use iota_model::*;

    /// SendTransferOptions
    ///
    /// * `threads` - Optionally specify the number of threads to use for PoW. This is ignored if `local_pow` is false.
    /// * `inputs` - Optionally specify which inputs to use when trying to find funds for transfers
    /// * `reference` - Optionally specify where to start searching for transactions to approve
    /// * `remainder_address` - Optionally specify where to send remaining funds after spending from addresses, automatically generated if not specified
    /// * `security` - Optioanlly specify the security to use for address generation (1-3). Default is 2
    /// * `hmac_key` - Optionally specify an HMAC key to use for this transaction
    #[derive(Clone, Debug, PartialEq)]
    pub struct SendTransferOptions<'a, 'b, 'c> {
        pub depth: usize,
        pub min_weight_magnitude: usize,
        pub local_pow: bool,
        pub threads: usize,
        pub inputs: Option<Inputs>,
        pub reference: Option<&'a str>,
        pub remainder_address: Option<&'b str>,
        pub security: usize,
        pub hmac_key: Option<&'c str>,
    }

    impl<'a, 'b, 'c> Default for SendTransferOptions<'a, 'b, 'c> {
        fn default() -> Self {
            SendTransferOptions {
                depth: 3,
                min_weight_magnitude: 14,
                local_pow: true,
                threads: num_cpus::get(),
                inputs: None,
                reference: None,
                remainder_address: None,
                security: 3,
                hmac_key: None,
            }
        }
    }

    /// GetNewAddressOptions
    ///
    /// * `security` - Security factor 1-3 with 3 being most secure
    /// * `index` - How many iterations of generating to skip
    /// * `total` - Number of addresses to generate. If total isn't provided, we generate until we find an unused address
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GetNewAddressOptions {
        pub security: Option<usize>,
        pub index: Option<usize>,
        pub total: Option<usize>,
    }

    /// SendTrytesOptions
    ///
    /// * `thread` - Optionally specify how many threads to use, defaults to max available
    /// * `reference` - Optionally used as the reference to start searching for transactions to approve
    #[derive(Clone, Debug, PartialEq)]
    pub struct SendTrytesOptions<'a> {
        pub depth: usize,
        pub min_weight_magnitude: usize,
        pub local_pow: bool,
        pub threads: usize,
        pub reference: Option<&'a str>,
    }

    impl<'a> Default for SendTrytesOptions<'a> {
        fn default() -> Self {
            SendTrytesOptions {
                depth: 3,
                min_weight_magnitude: 14,
                local_pow: true,
                threads: num_cpus::get(),
                reference: None,
            }
        }
    }

    /// GetInputsOptions
    ///
    /// * `start` - The start index for addresses to search
    /// * `end` - The end index for addresses to search
    /// * `threshold` - The amount of Iota you're trying to find in the wallet
    /// * `security` - The security to use for address generation
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct GetInputsOptions {
        pub start: Option<usize>,
        pub end: Option<usize>,
        pub threshold: Option<i64>,
        pub security: Option<usize>,
    }

    /// PrepareTransfersOptions
    ///
    /// * `inputs` - Optional inputs to use if you're sending iota
    /// * `remainder_address` - Optional remainder address to use, if not provided, one will be generated
    /// * `security` - Security to use when generating addresses (1-3)
    /// * `hmac_key` - Optional key to use if you want to hmac the transfers
    #[derive(Clone, Debug, PartialEq)]
    pub struct PrepareTransfersOptions<'a, 'b> {
        pub inputs: Option<Inputs>,
        pub remainder_address: Option<&'a str>,
        pub security: usize,
        pub hmac_key: Option<&'b str>,
    }

    impl<'a, 'b> Default for PrepareTransfersOptions<'a, 'b> {
        fn default() -> Self {
            PrepareTransfersOptions {
                inputs: None,
                remainder_address: None,
                security: 3,
                hmac_key: None,
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEED: &str =
        "IHDEENZYITYVYSPKAURUZAQKGVJEREFDJMYTANNXXGPZ9GJWTEOJJ9IPMXOGZNQLSNMFDSQOTZAEETUEA";
    const ADDR_SEED: &str =
        "LIESNFZLPFNWAPWXBLKEABZEEWUDCXKTRKZIRTPCKLKWOMJSEREWKMMMODUOFWM9ELEVXADTSQWMSNFVD";

    #[test]
    fn test_address_generation() {
        assert_eq!(new_address(&TEST_SEED, 2, 0, true).unwrap(), "LXQHWNY9CQOHPNMKFJFIJHGEPAENAOVFRDIBF99PPHDTWJDCGHLYETXT9NPUVSNKT9XDTDYNJKJCPQMZCCOZVXMTXC");
        assert_eq!(new_address(&TEST_SEED, 2, 5, true).unwrap(), "HLHRSJNPUUGRYOVYPSTEQJKETXNXDIWQURLTYDBJADGIYZCFXZTTFSOCECPPPPY9BYWPODZOCWJKXEWXDPUYEOTFQA");

        assert_eq!(
            new_address(&ADDR_SEED, 1, 0, false).unwrap(),
            "HIPPOUPZFMHJUQBLBVWORCNJWAOSFLHDWF9IOFEYVHPTTAAF9NIBMRKBICAPHYCDKMEEOXOYHJBMONJ9D"
        );
        assert_eq!(
            new_address(&ADDR_SEED, 2, 0, false).unwrap(),
            "BPYZABTUMEIOARZTMCDNUDAPUOFCGKNGJWUGUXUKNNBVKQARCZIXFVBZAAMDAFRS9YOIXWOTEUNSXVOG9"
        );
        assert_eq!(
            new_address(&ADDR_SEED, 3, 0, false).unwrap(),
            "BYWHJJYSHSEGVZKKYTJTYILLEYBSIDLSPXDLDZSWQ9XTTRLOSCBCQ9TKXJYQAVASYCMUCWXZHJYRGDOBW"
        );

        let concat = ADDR_SEED.to_string() + &ADDR_SEED;
        assert_eq!(
            new_address(&concat, 1, 0, false).unwrap(),
            "VKPCVHWKSCYQNHULMPYDZTNKOQHZNPEGJVPEHPTDIUYUBFKFICDRLLSIULHCVHOHZRHJOHNASOFRWFWZC"
        );
        assert_eq!(
            new_address(&concat, 2, 0, false).unwrap(),
            "PTHVACKMXOKIERJOFSRPBWCNKVEXQ9CWUTIJGEUORSKWEDDJCBFQCCBQZLTYXQCXEDWLTMRQM9OQPUGNC"
        );
        assert_eq!(
            new_address(&concat, 3, 0, false).unwrap(),
            "AGSAAETPMSBCDOSNXFXIOBAE9MVEJCSWVP9PAULQ9VABOTWLDMXID9MXCCWQIWRTJBASWPIJDFUC9ISWD"
        );
    }
}
