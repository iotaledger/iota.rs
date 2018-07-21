use super::iri_api;
use super::model::*;
use super::utils;
use super::utils::constants;
use super::utils::converter;
use super::utils::input_validator;
use chrono::prelude::*;
use crate::crypto;
use crate::Result;
use reqwest::Client;
use std::cmp;
use std::time::Duration;

/// Generates a new address
///
/// * `seed` - seed used to generate new address
/// * `index` - how many iterations of generating to skip
/// * `security` - security factor 1-3 with 3 being most secure
/// * `checksum` - whether or not to checksum address
pub fn new_address(seed: &str, index: usize, security: usize, checksum: bool) -> Result<String> {
    let key = crypto::signing::key(&converter::trits_from_string(seed), index, security)?;
    let digests = crypto::signing::digests(&key)?;
    let address_trits = crypto::signing::address(&digests)?;
    let mut address = converter::trytes(&address_trits);
    if checksum {
        address = utils::add_checksum(&address)?;
    }
    Ok(address)
}

/// An instance of the api using the same IRI URI throughout
#[derive(Clone, Debug)]
pub struct API {
    uri: String,
    client: reqwest::Client,
}

pub struct SendTransferOptions {
    pub threads: Option<usize>,
    pub inputs: Option<Inputs>,
    pub reference: Option<String>,
    pub remainder_address: Option<String>,
    pub security: Option<usize>,
    pub hmac_key: Option<String>,
}

pub struct AddRemainderOptions {
    pub seed: String,
    pub tag: String,
    pub remainder_address: Option<String>,
    pub signature_fragments: Vec<String>,
    pub added_hmac: bool,
    pub hmac_key: Option<String>,
    pub security: usize,
}

impl API {
    /// Create a new instance of the API
    ///
    /// * `uri` - the uri to use for all querys, currently only https IRI node are supported
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
    /// * `seed` - seed used to generate new address
    /// * `index` - how many iterations of generating to skip
    /// * `security` - security factor 1-3 with 3 being most secure
    /// * `checksum` - whether or not to checksum address
    /// * `total` - Number of addresses to generate. If total isn't provided, we generate until we find an unused address
    /// * `return_all` - whether to return all generated addresses, or just the last one
    pub fn get_new_address(
        &self,
        seed: &str,
        index: Option<usize>,
        security: Option<usize>,
        checksum: bool,
        total: Option<usize>,
        return_all: bool,
    ) -> Result<Vec<String>> {
        let mut index = index.unwrap_or_default();
        let security = security.unwrap_or(2);
        ensure!(input_validator::is_trytes(seed), "Invalid seed.");
        ensure!(security > 0 && security < 4, "Invalid security.");

        let mut all_addresses: Vec<String> = Vec::new();

        match total {
            Some(total) => {
                ensure!(total > 0, "Invalid total.");
                for i in index..total {
                    let address = new_address(seed, i, security, checksum)?;
                    all_addresses.push(address);
                }
                Ok(all_addresses)
            }
            None => loop {
                let new_address = new_address(seed, index, security, checksum)?;
                if return_all {
                    all_addresses.push(new_address.clone());
                }
                index += 1;
                let new_address_vec = vec![new_address];
                let were_addr_spent =
                    iri_api::were_addresses_spent_from(&self.client, &self.uri, &new_address_vec)?;
                if !were_addr_spent.state(0) {
                    let resp = iri_api::find_transactions(
                        &self.client,
                        &self.uri,
                        None,
                        Some(&new_address_vec),
                        None,
                        None,
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
    /// * `trytes` - a slice of strings that are tryte-encoded transactions
    /// * `depth` - the depth to search for transactions to approve
    /// * `min_weight_magnitude` - the PoW difficulty factor (14 on mainnet, 9 on testnet)
    /// * `local_pow` - whether or not to do local PoW
    /// * `reference` - Optionally used as the reference to start searching for transactions to approve
    pub fn send_trytes<U, S>(
        &self,
        trytes: &[String],
        depth: usize,
        min_weight_magnitude: usize,
        local_pow: bool,
        threads: U,
        reference: S,
    ) -> Result<Vec<Transaction>>
    where
        U: Copy + Into<Option<usize>>,
        S: Into<Option<String>>,
    {
        let to_approve = iri_api::get_transactions_to_approve(
            &self.client,
            &self.uri,
            depth,
            &reference.into(),
        )?;
        let trytes_list = if local_pow {
            let res = iri_api::attach_to_tangle_local(
                threads,
                &to_approve.trunk_transaction().unwrap(),
                &to_approve.branch_transaction().unwrap(),
                min_weight_magnitude,
                trytes,
            )?;
            res.trytes().unwrap()
        } else {
            let attached = iri_api::attach_to_tangle(
                &self.client,
                &self.uri,
                &to_approve.trunk_transaction().unwrap(),
                &to_approve.branch_transaction().unwrap(),
                min_weight_magnitude,
                trytes,
            )?;
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
        iri_api::store_transactions(&self.client, &self.uri, trytes)?;
        iri_api::broadcast_transactions(&self.client, &self.uri, trytes)?;
        Ok(())
    }

    /// Given a seed, iterates through addresses looking for
    /// enough funds to meet specified threshold
    ///
    /// * `seed` - The wallet seed to use
    /// * `start` - The start index for addresses to search
    /// * `end` - The end index for addresses to search
    /// * `threshold` - The amount of Iota you're trying to find in the wallet
    /// * `security` - The security to use for address generation
    pub fn get_inputs(
        &self,
        seed: &str,
        start: Option<usize>,
        end: Option<usize>,
        threshold: Option<i64>,
        security: Option<usize>,
    ) -> Result<Inputs> {
        ensure!(input_validator::is_trytes(seed), "Invalid seed.");
        let start = start.unwrap_or(0);
        let security = security.unwrap_or(2);

        let get_balance_and_format = |addresses: &[String]| -> Result<Inputs> {
            let resp = iri_api::get_balances(&self.client, &self.uri, addresses, 100)?;
            let mut inputs = Inputs::default();

            let mut threshold_reached = match threshold {
                Some(_) => false,
                None => true,
            };

            let balances = resp.take_balances().unwrap_or_default();
            for (i, address) in addresses.iter().enumerate() {
                let balance: i64 = balances[i].clone().parse()?;
                if balance > 0 {
                    let new_entry = Input::new(address.clone(), balance, start + i, security);
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
        };

        if let Some(end) = end {
            ensure!(
                start <= end && end <= start + 500,
                "Invalid inputs provided."
            );
            let mut all_addresses: Vec<String> = Vec::new();
            for i in start..end {
                all_addresses.push(new_address(seed, i, security, false)?);
            }
            get_balance_and_format(&all_addresses)
        } else {
            let new_address =
                self.get_new_address(seed, Some(start), Some(security), false, None, true)?;
            get_balance_and_format(&new_address)
        }
    }

    /// Prepares a slice of transfers and converts them into a
    /// slice of tryte-encoded strings
    ///
    /// * `seed` - The wallet seed to use
    /// * `transfers` - A slice of transfers to prepare
    /// * `inputs` - Optional inputs to use if you're sending iota
    /// * `remainder_address` - Optional remainder address to use, if not provided, one will be generated
    /// * `security` - Security to use when generating addresses (1-3)
    /// * `hmac_key` - Optional key to use if you want to hmac the transfers
    pub fn prepare_transfers<T, U, S, R>(
        &self,
        seed: &str,
        transfers: T,
        inputs: Option<Inputs>,
        remainder_address: S,
        security: U,
        hmac_key: R,
    ) -> Result<Vec<String>>
    where
        T: Into<Vec<Transfer>>,
        U: Into<Option<usize>>,
        S: Into<Option<String>>,
        R: Into<Option<String>>,
    {
        let mut add_hmac = false;
        let mut added_hmac = false;
        let mut transfers = transfers.into();
        let remainder_address = remainder_address.into();
        let security = security.into();
        let hmac_key = hmac_key.into();

        ensure!(input_validator::is_trytes(seed), "Invalid seed.");
        if let Some(hmac_key) = &hmac_key {
            ensure!(input_validator::is_trytes(&hmac_key), "Invalid trytes.");
            add_hmac = true;
        }
        for transfer in &mut transfers {
            if add_hmac && *transfer.value() > 0 {
                *transfer.message_mut() = "9".repeat(243) + transfer.message();
                added_hmac = true;
            }
            if transfer.address().len() == 90 {
                ensure!(
                    utils::is_valid_checksum(transfer.address())?,
                    "Invalid address."
                );
            }
            *transfer.address_mut() = utils::remove_checksum(transfer.address());
        }
        ensure!(
            input_validator::is_transfers_collection_valid(&transfers),
            "Invalid transfers."
        );
        let security = security.unwrap_or_else(|| 2);
        let mut bundle = Bundle::default();
        let mut total_value = 0;
        let mut signature_fragments: Vec<String> = Vec::new();
        let mut tag = String::new();

        for transfer in transfers {
            let mut signature_message_length = 1;
            if transfer.message().len() > constants::MESSAGE_LENGTH {
                signature_message_length += (transfer.message().len() as f64
                    / constants::MESSAGE_LENGTH as f64)
                    .floor() as usize;
                let mut msg_copy = transfer.message().to_string();
                while !msg_copy.is_empty() {
                    let mut fragment = msg_copy.chars().take(constants::MESSAGE_LENGTH).collect();
                    msg_copy = msg_copy.chars().skip(constants::MESSAGE_LENGTH).collect();
                    utils::right_pad_string(&mut fragment, constants::MESSAGE_LENGTH, '9');
                    signature_fragments.push(fragment);
                }
            } else {
                let mut fragment = if !transfer.message().is_empty() {
                    transfer.message().chars().take(2187).collect()
                } else {
                    String::new()
                };
                utils::right_pad_string(&mut fragment, constants::MESSAGE_LENGTH, '9');
                signature_fragments.push(fragment);
            }
            tag = transfer.tag().unwrap_or_default();
            utils::right_pad_string(&mut tag, constants::TAG_LENGTH, '9');
            bundle.add_entry(
                signature_message_length,
                transfer.address(),
                *transfer.value(),
                &tag,
                Utc::now().timestamp(),
            );
            total_value += *transfer.value();
        }

        if total_value > 0 {
            match inputs {
                Some(inputs) => {
                    let input_addresses: Vec<String> = inputs
                        .inputs_list()
                        .iter()
                        .map(|input| input.address().to_string())
                        .collect();
                    let resp =
                        iri_api::get_balances(&self.client, &self.uri, &input_addresses, 100)?;
                    let mut confirmed_inputs = Inputs::default();
                    let balances = resp.take_balances().unwrap_or_default();
                    for (i, balance) in balances.iter().enumerate() {
                        let b: i64 = balance.parse()?;
                        if b > 0 {
                            *confirmed_inputs.total_balance_mut() += b;
                            let mut confirmed_input = inputs.inputs_list()[i].clone();
                            confirmed_input.set_balance(b);
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
                            seed: seed.to_string(),
                            tag,
                            remainder_address,
                            signature_fragments,
                            added_hmac,
                            hmac_key,
                            security,
                        },
                    )
                }
                None => {
                    let inputs =
                        self.get_inputs(seed, None, None, Some(total_value), Some(security))?;
                    self.add_remainder(
                        &inputs,
                        &mut bundle,
                        AddRemainderOptions {
                            seed: seed.to_string(),
                            tag,
                            remainder_address,
                            signature_fragments,
                            added_hmac,
                            hmac_key,
                            security,
                        },
                    )
                }
            }
        } else {
            bundle.finalize()?;
            bundle.add_trytes(&signature_fragments);
            let mut bundle_trytes: Vec<String> = Vec::new();
            for b in bundle.bundle().iter().rev() {
                bundle_trytes.push(b.to_trytes());
            }
            Ok(bundle_trytes)
        }
    }

    /// Prepares and sends a slice of transfers
    /// This helper does everything for you, PoW and such
    ///
    /// * `seed` - The wallet seed to use
    /// * `depth` - The depth to search when looking for transactions to approve
    /// * `min_weight_magnitude` - The PoW difficulty factor (14 on mainnet, 9 on testnet)
    /// * `transfers` - A slice of transfers to send
    /// * `local_pow` - Whether or not to do local PoW
    /// * `inputs` - Optionally specify which inputs to use when trying to find funds for transfers
    /// * `reference` - Optionally specify where to start searching for transactions to approve
    /// * `remainder_address` - Optionally specify where to send remaining funds after spending from addresses, automatically generated if not specified
    /// * `security` - Optioanlly specify the security to use for address generation (1-3). Default is 2
    /// * `hmac_key` - Optionally specify an HMAC key to use for this transaction
    pub fn send_transfers<T>(
        &self,
        seed: &str,
        depth: usize,
        min_weight_magnitude: usize,
        transfers: T,
        local_pow: bool,
        options: SendTransferOptions,
    ) -> Result<Vec<Transaction>>
    where
        T: Into<Vec<Transfer>>,
    {
        let trytes = self.prepare_transfers(
            seed,
            transfers,
            options.inputs,
            options.remainder_address,
            options.security,
            options.hmac_key,
        )?;
        let t = self.send_trytes(
            &trytes,
            depth,
            min_weight_magnitude,
            local_pow,
            options.threads,
            options.reference,
        )?;
        Ok(t)
    }

    /// Traverses a bundle by going through trunk transactions until
    /// the bundle hash of the transaction is no longer the same.
    ///
    /// * `trunk_tx` - The trunk transaction to start searching at
    /// * `bundle_hash` - The bundle hash to compare against while searching
    /// * `bundle` - The bundle add transactions to, until hash no longer matches
    pub fn traverse_bundle(
        &self,
        trunk_tx: &str,
        bundle_hash: Option<String>,
        mut bundle: Vec<Transaction>,
    ) -> Result<Vec<Transaction>> {
        let tryte_list = iri_api::get_trytes(&self.client, &self.uri, &[trunk_tx.to_string()])?
            .take_trytes()
            .unwrap_or_default();
        ensure!(!tryte_list.is_empty(), "Bundle transactions not visible");
        let trytes = &tryte_list[0];
        let tx: Transaction = trytes.parse()?;
        let tx_bundle = tx.bundle().unwrap_or_default();
        ensure!(
            tx.current_index().unwrap_or_default() == 0,
            "Invalid tail transaction supplied."
        );
        let bundle_hash = bundle_hash.unwrap_or_else(|| tx_bundle.clone());
        if bundle_hash != tx_bundle {
            return Ok(bundle.to_vec());
        }

        if tx.last_index().unwrap_or_default() == 0 && tx.current_index().unwrap_or_default() == 0 {
            return Ok(vec![tx]);
        }

        let trunk_tx = tx.trunk_transaction().unwrap_or_default();
        bundle.push(tx);
        self.traverse_bundle(&trunk_tx, Some(bundle_hash), bundle)
    }

    /// Gets the associated bundle transactions of a transaction
    /// Validates the signatures, total sum, and bundle order
    ///
    /// * `transaction` - The transaction hash to search for
    pub fn get_bundle(&self, transaction: &str) -> Result<Vec<Transaction>> {
        ensure!(
            input_validator::is_hash(transaction),
            "Invalid transaction."
        );
        let bundle = self.traverse_bundle(transaction, None, Vec::new())?;
        ensure!(utils::is_bundle(&bundle)?, "Invalid bundle provided.");
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
            let this_balance = input.balance();
            let to_subtract = 0 - this_balance;
            let timestamp = Utc::now().timestamp();
            let address = utils::remove_checksum(input.address());

            bundle.add_entry(
                input.security(),
                &address,
                to_subtract,
                &options.tag,
                timestamp,
            );

            if this_balance >= total_transfer_value {
                let remainder = this_balance - total_transfer_value;
                if let Some(remainder_address) = &options.remainder_address {
                    if remainder > 0 {
                        bundle.add_entry(1, remainder_address, remainder, &options.tag, timestamp);
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
                        start_index = cmp::max(input.key_index(), start_index);
                    }
                    start_index += 1;
                    let new_address = self.get_new_address(
                        &options.seed,
                        Some(start_index),
                        Some(options.security),
                        false,
                        None,
                        false,
                    )?[0]
                        .clone();
                    bundle.add_entry(
                        1,
                        &new_address,
                        remainder,
                        &options.tag,
                        Utc::now().timestamp(),
                    );
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

    fn sign_inputs_and_return(
        &self,
        seed: &str,
        inputs: &Inputs,
        bundle: &mut Bundle,
        signature_fragments: &[String],
        added_hmac: bool,
        hmac_key: Option<String>,
    ) -> Result<Vec<String>> {
        bundle.finalize()?;
        bundle.add_trytes(&signature_fragments);
        for i in 0..bundle.bundle().len() {
            if bundle.bundle()[i].value().unwrap_or_default() < 0 {
                let this_address = bundle.bundle()[i].address().unwrap_or_default();
                let mut key_index = 0;
                let mut key_security = 0;
                for input in inputs.inputs_list() {
                    if input.address() == this_address {
                        key_index = input.key_index();
                        key_security = input.security();
                        break;
                    }
                }
                let bundle_hash = bundle.bundle()[i].bundle().unwrap_or_default();
                let key = crypto::signing::key(
                    &converter::trits_from_string(seed),
                    key_index,
                    key_security,
                )?;
                let normalized_bundle_hash = Bundle::normalized_bundle(&bundle_hash).to_vec();
                let mut normalized_bundle_fragments = [[0; 27]; 3];
                for (j, c) in normalized_bundle_hash.chunks(27).enumerate() {
                    normalized_bundle_fragments[j].copy_from_slice(c);
                }
                let first_fragment = key[0..6561].to_vec();
                let first_bundle_fragment = normalized_bundle_fragments[0];
                let first_signed_fragment =
                    crypto::signing::signature_fragment(&first_bundle_fragment, &first_fragment)?;
                *bundle.bundle_mut()[i].signature_fragments_mut() =
                    Some(converter::trytes(&first_signed_fragment));
                for j in 1..key_security {
                    if bundle.bundle()[i + j].address().unwrap_or_default() == this_address
                        && bundle.bundle()[i + j].value().unwrap_or_default() == 0
                    {
                        let next_fragment = key[6561 * j..(j + 1) * 6561].to_vec();
                        let next_bundle_fragment = normalized_bundle_fragments[j];
                        let next_signed_fragment = crypto::signing::signature_fragment(
                            &next_bundle_fragment,
                            &next_fragment,
                        )?;
                        *bundle.bundle_mut()[i + j].signature_fragments_mut() =
                            Some(converter::trytes(&next_signed_fragment));
                    }
                }
            }
        }
        if added_hmac {
            let hmac = crypto::HMAC::new(&hmac_key.unwrap_or_default());
            hmac.add_hmac(bundle)?;
        }
        let mut bundle_trytes: Vec<String> = Vec::new();
        for tx in bundle.bundle().iter().rev() {
            bundle_trytes.push(tx.to_trytes());
        }
        Ok(bundle_trytes)
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
        assert_eq!(new_address(TEST_SEED, 0, 2, true).unwrap(), "LXQHWNY9CQOHPNMKFJFIJHGEPAENAOVFRDIBF99PPHDTWJDCGHLYETXT9NPUVSNKT9XDTDYNJKJCPQMZCCOZVXMTXC");
        assert_eq!(new_address(TEST_SEED, 5, 2, true).unwrap(), "HLHRSJNPUUGRYOVYPSTEQJKETXNXDIWQURLTYDBJADGIYZCFXZTTFSOCECPPPPY9BYWPODZOCWJKXEWXDPUYEOTFQA");

        assert_eq!(
            new_address(ADDR_SEED, 0, 1, false).unwrap(),
            "HIPPOUPZFMHJUQBLBVWORCNJWAOSFLHDWF9IOFEYVHPTTAAF9NIBMRKBICAPHYCDKMEEOXOYHJBMONJ9D"
        );
        assert_eq!(
            new_address(ADDR_SEED, 0, 2, false).unwrap(),
            "BPYZABTUMEIOARZTMCDNUDAPUOFCGKNGJWUGUXUKNNBVKQARCZIXFVBZAAMDAFRS9YOIXWOTEUNSXVOG9"
        );
        assert_eq!(
            new_address(ADDR_SEED, 0, 3, false).unwrap(),
            "BYWHJJYSHSEGVZKKYTJTYILLEYBSIDLSPXDLDZSWQ9XTTRLOSCBCQ9TKXJYQAVASYCMUCWXZHJYRGDOBW"
        );

        let concat = ADDR_SEED.to_string() + ADDR_SEED;
        assert_eq!(
            new_address(&concat, 0, 1, false).unwrap(),
            "VKPCVHWKSCYQNHULMPYDZTNKOQHZNPEGJVPEHPTDIUYUBFKFICDRLLSIULHCVHOHZRHJOHNASOFRWFWZC"
        );
        assert_eq!(
            new_address(&concat, 0, 2, false).unwrap(),
            "PTHVACKMXOKIERJOFSRPBWCNKVEXQ9CWUTIJGEUORSKWEDDJCBFQCCBQZLTYXQCXEDWLTMRQM9OQPUGNC"
        );
        assert_eq!(
            new_address(&concat, 0, 3, false).unwrap(),
            "AGSAAETPMSBCDOSNXFXIOBAE9MVEJCSWVP9PAULQ9VABOTWLDMXID9MXCCWQIWRTJBASWPIJDFUC9ISWD"
        );
    }
}
