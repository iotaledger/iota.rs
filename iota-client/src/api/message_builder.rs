// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{api::address::search_address, Client, ClientMiner, Error, Result};

use bee_message::prelude::*;
use bee_pow::providers::ProviderBuilder;
use bee_rest_api::types::{AddressDto, OutputDto};
use crypto::slip10::{Chain, Curve, Seed};

use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

const MAX_ALLOWED_DUST_OUTPUTS: i64 = 100;
const ADDRESS_GAP_LIMIT: usize = 20;

/// Structure for sorting of UnlockBlocks
// TODO: move the sorting process to the `Message` crate
struct AddressIndexRecorder {
    input: Input,
    address_index: usize,
    chain: Chain,
    internal: bool,
}

/// Builder of send API
pub struct ClientMessageBuilder<'a> {
    client: &'a Client,
    seed: Option<&'a Seed>,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
    inputs: Option<Vec<UTXOInput>>,
    input_range: Range<usize>,
    outputs: Vec<Output>,
    index: Option<Box<[u8]>>,
    data: Option<Vec<u8>>,
    parents: Option<Vec<MessageId>>,
}

impl<'a> ClientMessageBuilder<'a> {
    /// Create send builder
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            seed: None,
            account_index: None,
            initial_address_index: None,
            inputs: None,
            input_range: 0..100,
            outputs: Vec::new(),
            index: None,
            data: None,
            parents: None,
        }
    }

    /// Sets the seed.
    pub fn with_seed(mut self, seed: &'a Seed) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: usize) -> Self {
        self.account_index = Some(account_index);
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: usize) -> Self {
        self.initial_address_index = Some(initial_address_index);
        self
    }

    /// Set a custom input(transaction output)
    pub fn with_input(mut self, input: UTXOInput) -> Self {
        self.inputs = match self.inputs {
            Some(mut inputs) => {
                inputs.push(input);
                Some(inputs)
            }
            None => Some(vec![input]),
        };
        self
    }

    /// Set a custom range in which to search for addresses for custom inputs. Default: 0..100
    pub fn with_input_range(mut self, range: Range<usize>) -> Self {
        self.input_range = range;
        self
    }

    /// Set a transfer to the builder, address needs to be Bech32 encoded
    pub fn with_output(mut self, address: &Bech32Address, amount: u64) -> Result<Self> {
        let address = Address::try_from_bech32(&address.to_string())?;
        let output = SignatureLockedSingleOutput::new(address, amount)?.into();
        self.outputs.push(output);
        Ok(self)
    }

    /// Set a dust allowance transfer to the builder, address needs to be Bech32 encoded
    pub fn with_dust_allowance_output(mut self, address: &Bech32Address, amount: u64) -> Result<Self> {
        if amount < 1_000_000 {
            return Err(Error::DustError(
                "Amount for SignatureLockedDustAllowanceOutput needs to be >= 1_000_000".into(),
            ));
        }
        let address = Address::try_from_bech32(&address.to_string())?;
        let output = SignatureLockedDustAllowanceOutput::new(address, amount)?.into();
        self.outputs.push(output);
        Ok(self)
    }

    /// Set a transfer to the builder, address needs to be hex encoded
    pub fn with_output_hex(mut self, address: &str, amount: u64) -> Result<Self> {
        let output = SignatureLockedSingleOutput::new(address.parse::<Ed25519Address>()?.into(), amount)?.into();
        self.outputs.push(output);
        Ok(self)
    }

    /// Set indexation to the builder
    pub fn with_index<I: AsRef<[u8]>>(mut self, index: I) -> Self {
        self.index = Some(index.as_ref().into());
        self
    }

    /// Set data to the builder
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Set 1-8 custom parent message ids
    pub fn with_parents(mut self, parent_ids: Vec<MessageId>) -> Result<Self> {
        if !(1..=8).contains(&parent_ids.len()) {
            return Err(Error::InvalidParentsAmount);
        }
        self.parents = Some(parent_ids);
        Ok(self)
    }

    /// Consume the builder and get the API result
    pub async fn finish(self) -> Result<Message> {
        // Indexation payload requires an indexation tag
        if self.data.is_some() && self.index.is_none() {
            return Err(Error::MissingParameter(String::from("index")));
        }
        if self.inputs.is_some() && self.outputs.is_empty() {
            return Err(Error::MissingParameter(String::from("output")));
        }
        if !self.outputs.is_empty() {
            if self.seed.is_none() {
                return Err(Error::MissingParameter(String::from("Seed")));
            }
            // Send message with transaction
            self.finish_transaction().await
        } else if self.index.is_some() {
            // Send message with indexation payload
            self.finish_indexation().await
        } else {
            // Send message without payload
            self.finish_message(None).await
        }
    }

    /// Consume the builder and get the API result
    pub async fn finish_transaction(self) -> Result<Message> {
        let account_index = self.account_index.unwrap_or(0);

        let mut index = self.initial_address_index.unwrap_or(0);

        let bech32_hrp = self.client.get_bech32_hrp().await?;

        // store (amount, address, new_created) to check later if dust is allowed
        let mut dust_and_allowance_recorders = Vec::new();

        // Calculate the total tokens to spend
        let mut total_to_spend = 0;
        let mut total_already_spent = 0;
        for output in &self.outputs {
            match output {
                Output::SignatureLockedSingle(x) => {
                    total_to_spend += x.amount();
                    if x.amount() < 1_000_000 {
                        dust_and_allowance_recorders.push((x.amount(), *x.address(), true));
                    }
                }
                Output::SignatureLockedDustAllowance(x) => {
                    total_to_spend += x.amount();
                    dust_and_allowance_recorders.push((x.amount(), *x.address(), true));
                }
                _ => {}
            }
        }

        let mut essence = RegularEssence::builder();
        let mut address_index_recorders = Vec::new();

        match self.inputs.clone() {
            Some(inputs) => {
                for input in inputs {
                    // Only add unspent outputs
                    if let Ok(output) = self.client.get_output(&input).await {
                        if !output.is_spent {
                            let (output_amount, output_address) = match output.output {
                                OutputDto::Treasury(_) => panic!("Can't be used as input"),
                                OutputDto::SignatureLockedSingle(r) => match r.address {
                                    AddressDto::Ed25519(addr) => {
                                        let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                                        // Only add dust
                                        if r.amount < 1_000_000 {
                                            dust_and_allowance_recorders.push((r.amount, output_address, false));
                                        }
                                        (r.amount, output_address)
                                    }
                                },
                                OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                                    AddressDto::Ed25519(addr) => {
                                        let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                                        dust_and_allowance_recorders.push((r.amount, output_address, false));
                                        (r.amount, output_address)
                                    }
                                },
                            };
                            total_already_spent += output_amount;
                            // Note that we need to sign the original address, i.e., `path/index`,
                            // instead of `path/index/_offset` or `path/_offset`.
                            let bech32_address = output_address.to_bech32(&bech32_hrp);
                            let (address_index, internal) = search_address(
                                &self.seed.expect("No seed"),
                                bech32_hrp.clone(),
                                account_index,
                                self.input_range.clone(),
                                &bech32_address.into(),
                            )
                            .await?;
                            let chain = Chain::from_u32_hardened(vec![
                                44,
                                4218,
                                account_index as u32,
                                internal as u32,
                                address_index as u32,
                            ]);
                            let input = Input::UTXO(
                                UTXOInput::new(TransactionId::from_str(&output.transaction_id)?, output.output_index)
                                    .map_err(|_| Error::TransactionError)?,
                            );
                            essence = essence.add_input(input.clone());
                            address_index_recorders.push(AddressIndexRecorder {
                                input,
                                address_index,
                                chain,
                                internal,
                            });
                            // Output the remaining tokens back to the original address
                            if total_already_spent > total_to_spend {
                                let remaining_balance = total_already_spent - total_to_spend;
                                if remaining_balance < 1_000_000 {
                                    dust_and_allowance_recorders.push((remaining_balance, output_address, true));
                                }
                                essence = essence.add_output(
                                    SignatureLockedSingleOutput::new(output_address, remaining_balance)?.into(),
                                );
                            }
                        }
                    }
                }
            }
            None => {
                'input_selection: loop {
                    // Reset the empty_address_count for each run of output address searching
                    let mut empty_address_count: u64 = 0;
                    // Get the addresses in the BIP path/index ~ path/index+20
                    let addresses = self
                        .client
                        .get_addresses(self.seed.expect("No seed"))
                        .with_account_index(account_index)
                        .with_range(index..index + ADDRESS_GAP_LIMIT)
                        .get_all()
                        .await?;
                    // For each address, get the address outputs
                    let mut address_index = index;
                    for (index, (address, internal)) in addresses.iter().enumerate() {
                        let address_outputs = self.client.get_address().outputs(&address).await?;
                        let mut outputs = vec![];
                        for output_id in address_outputs.iter() {
                            let curr_outputs = self.client.get_output(output_id).await?;
                            if !curr_outputs.is_spent {
                                outputs.push(curr_outputs);
                            }
                        }
                        // If there are more than 20 (gap limit) consecutive empty addresses, then we stop looking
                        // up the addresses belonging to the seed. Note that we don't really count the exact 20
                        // consecutive empty addresses, which is unnecessary. We just need to check the address range,
                        // [k*20, k*20 + 20), where k is natural number, and to see if the outputs are all empty.
                        if outputs.is_empty() {
                            // Accumulate the empty_address_count for each run of output address searching
                            empty_address_count += 1;
                        }
                        for (_offset, output) in outputs.into_iter().enumerate() {
                            let output_amount = match output.output {
                                OutputDto::Treasury(_) => panic!("Can't be used as input"),
                                OutputDto::SignatureLockedSingle(r) => match r.address {
                                    AddressDto::Ed25519(addr) => {
                                        if r.amount < 1_000_000 {
                                            let output_address =
                                                Address::from(Ed25519Address::from_str(&addr.address)?);
                                            dust_and_allowance_recorders.push((r.amount, output_address, false));
                                        }
                                        r.amount
                                    }
                                },
                                OutputDto::SignatureLockedDustAllowance(r) => match r.address {
                                    AddressDto::Ed25519(addr) => {
                                        let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                                        dust_and_allowance_recorders.push((r.amount, output_address, false));
                                        r.amount
                                    }
                                },
                            };
                            match output.is_spent {
                                true => {
                                    return Err(Error::SpentOutput);
                                }
                                false => {
                                    if total_already_spent < total_to_spend {
                                        total_already_spent += output_amount;
                                        // Note that we need to sign the original address, i.e., `path/index`,
                                        // instead of `path/index/_offset` or `path/_offset`.
                                        let chain = Chain::from_u32_hardened(vec![
                                            44,
                                            4218,
                                            account_index as u32,
                                            *internal as u32,
                                            address_index as u32,
                                        ]);
                                        let input = Input::UTXO(
                                            UTXOInput::new(
                                                TransactionId::from_str(&output.transaction_id)?,
                                                output.output_index,
                                            )
                                            .map_err(|_| Error::TransactionError)?,
                                        );
                                        essence = essence.add_input(input.clone());
                                        address_index_recorders.push(AddressIndexRecorder {
                                            input,
                                            address_index,
                                            chain,
                                            internal: *internal,
                                        });
                                        if total_already_spent > total_to_spend {
                                            let remaining_balance = total_already_spent - total_to_spend;
                                            if remaining_balance < 1_000_000 {
                                                dust_and_allowance_recorders.push((
                                                    remaining_balance,
                                                    Address::try_from_bech32(address)?,
                                                    true,
                                                ));
                                            }
                                            // Output the remaining tokens back to the original address
                                            essence = essence.add_output(
                                                SignatureLockedSingleOutput::new(
                                                    Address::try_from_bech32(address)?,
                                                    remaining_balance,
                                                )?
                                                .into(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        if total_already_spent >= total_to_spend {
                            break 'input_selection;
                        }
                        // if we just processed an even index, increase the address index
                        // (because the list has public and internal addresses)
                        if index % 2 == 1 {
                            address_index += 1;
                        }
                    }
                    index += ADDRESS_GAP_LIMIT;
                    // The gap limit is 20 and use reference 40 here because there's public and internal addresses
                    if empty_address_count == 40 {
                        break;
                    }
                }
            }
        }

        if total_already_spent < total_to_spend {
            return Err(Error::NotEnoughBalance(total_already_spent, total_to_spend));
        }

        // Check if we would let dust on an address behind or send new dust, which would make the tx unconfirmable
        let mut single_addresses = HashSet::new();
        for dust_or_allowance in &dust_and_allowance_recorders {
            single_addresses.insert(dust_or_allowance.1);
        }
        for address in single_addresses {
            let created_or_consumed_outputs: Vec<(u64, Address, bool)> = dust_and_allowance_recorders
                .iter()
                .cloned()
                .filter(|d| d.1 == address)
                .collect();
            is_dust_allowed(
                &self.client,
                address.to_bech32(&bech32_hrp).into(),
                created_or_consumed_outputs,
            )
            .await?;
        }

        // Build signed transaction payload
        for output in self.outputs.clone() {
            essence = essence.add_output(output);
        }
        // Add indexation_payload if index set
        if let Some(index) = self.index.clone() {
            let indexation_payload = IndexationPayload::new(&index, &self.data.clone().unwrap_or_default())?;
            essence = essence.with_payload(Payload::Indexation(Box::new(indexation_payload)))
        }
        let regular_essence = essence.finish()?;
        let essence = Essence::Regular(regular_essence);
        let hashed_essence = essence.hash();
        let mut unlock_blocks = Vec::new();
        let mut signature_indexes = HashMap::<String, usize>::new();
        address_index_recorders.sort_by(|a, b| a.input.cmp(&b.input));

        for (current_block_index, recorder) in address_index_recorders.iter().enumerate() {
            // Check if current path is same as previous path
            // If so, add a reference unlock block
            // Format to differentiate between public and private addresses
            let index = format!("{}{}", recorder.address_index, recorder.internal);
            if let Some(block_index) = signature_indexes.get(&index) {
                unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
            } else {
                // If not, we should create a signature unlock block
                let private_key = self
                    .seed
                    .expect("No seed")
                    .derive(Curve::Ed25519, &recorder.chain)?
                    .secret_key()?;
                let public_key = private_key.public_key().to_compressed_bytes();
                // The block should sign the entire transaction essence part of the transaction payload
                let signature = Box::new(private_key.sign(&hashed_essence).to_bytes());
                unlock_blocks.push(UnlockBlock::Signature(SignatureUnlock::Ed25519(Ed25519Signature::new(
                    public_key, signature,
                ))));
                signature_indexes.insert(index, current_block_index);
            }
        }
        // TODO overflow check
        let mut payload_builder = TransactionPayloadBuilder::new().with_essence(essence);
        for unlock in unlock_blocks {
            payload_builder = payload_builder.add_unlock_block(unlock);
        }

        let payload = payload_builder.finish().map_err(|_| Error::TransactionError)?;
        // building message
        let payload = Payload::Transaction(Box::new(payload));

        self.finish_message(Some(payload)).await
    }

    /// Consume the builder and get the API result
    pub async fn finish_indexation(self) -> Result<Message> {
        let payload: Payload;
        {
            let index = &self.index.as_ref();
            let empty_slice = &vec![];
            let data = &self.data.as_ref().unwrap_or(empty_slice);

            // build indexation
            let index = IndexationPayload::new(index.expect("No indexation tag"), data)
                .map_err(|e| Error::IndexationError(e.to_string()))?;
            payload = Payload::Indexation(Box::new(index));
        }

        // building message
        self.finish_message(Some(payload)).await
    }

    /// Builds the final message and posts it to the node
    pub async fn finish_message(self, payload: Option<Payload>) -> Result<Message> {
        let final_message = match self.parents {
            Some(mut parents) => {
                parents.sort_unstable();
                parents.dedup();
                let min_pow_score = self.client.get_min_pow_score().await?;
                let network_id = self.client.get_network_id().await?;
                do_pow(
                    crate::client::ClientMinerBuilder::new()
                        .with_local_pow(self.client.get_local_pow())
                        .finish(),
                    min_pow_score,
                    network_id,
                    payload,
                    parents,
                    Arc::new(AtomicBool::new(false)),
                )?
                .1
                .unwrap()
            }
            None => finish_pow(&self.client, payload).await?,
        };

        let msg_id = self.client.post_message(&final_message).await?;
        // Get message if we use remote PoW, because the node will change parents and nonce
        let msg = match self.client.get_local_pow() {
            true => final_message,
            false => self.client.get_message().data(&msg_id).await?,
        };

        Ok(msg)
    }
}

// Calculate the outputs on this address after this transaction gets confirmed so we know if we can send dust or
// dust allowance outputs (as input), the bool in the outputs defines if we consume this output (false) or create a new
// one (true)
async fn is_dust_allowed(client: &Client, address: Bech32Address, outputs: Vec<(u64, Address, bool)>) -> Result<()> {
    // balance of all dust allowance outputs
    let mut dust_allowance_balance: i64 = 0;
    // Amount of dust outputs
    let mut dust_outputs_amount: i64 = 0;

    // Add outputs from this transaction
    for output in outputs {
        match output.2 {
            // add newly created outputs
            true => {
                if output.0 >= 1_000_000 {
                    dust_allowance_balance += output.0 as i64;
                } else {
                    dust_outputs_amount += 1
                }
            }
            // remove consumed outputs
            false => {
                if output.0 >= 1_000_000 {
                    dust_allowance_balance -= output.0 as i64;
                } else {
                    dust_outputs_amount -= 1;
                }
            }
        }
    }

    // Get outputs from address and apply values
    let address_outputs_metadata = client.find_outputs(&[], &[address.clone()]).await?;
    for output_metadata in address_outputs_metadata {
        match output_metadata.output {
            OutputDto::Treasury(_) => {}
            OutputDto::SignatureLockedDustAllowance(d_a_o) => {
                dust_allowance_balance += d_a_o.amount as i64;
            }
            OutputDto::SignatureLockedSingle(s_o) => {
                if s_o.amount < 1_000_000 {
                    dust_outputs_amount += 1;
                }
            }
        }
    }

    // Here dust_allowance_balance and dust_outputs_amount should be as if this transaction gets confirmed
    // Max allowed dust outputs is 100
    let allowed_dust_amount = std::cmp::min(dust_allowance_balance / 100_000, MAX_ALLOWED_DUST_OUTPUTS);
    if dust_outputs_amount > allowed_dust_amount {
        return Err(Error::DustError(format!(
            "No dust output allowed on address {}",
            address
        )));
    }
    Ok(())
}

/// Does PoW with always new tips
pub async fn finish_pow(client: &Client, payload: Option<Payload>) -> Result<Message> {
    let done = Arc::new(AtomicBool::new(false));
    let local_pow = client.get_local_pow();
    let min_pow_score = client.get_min_pow_score().await?;
    let tips_interval = client.get_tips_interval();
    let network_id = client.get_network_id().await?;
    loop {
        let abort1 = Arc::clone(&done);
        let abort2 = Arc::clone(&done);
        let payload_ = payload.clone();
        let parent_messages = client.get_tips().await?;
        let time_thread = std::thread::spawn(move || Ok(pow_timeout(tips_interval, &abort1)));
        let pow_thread = std::thread::spawn(move || {
            do_pow(
                crate::client::ClientMinerBuilder::new()
                    .with_local_pow(local_pow)
                    .finish(),
                min_pow_score,
                network_id,
                payload_,
                parent_messages,
                abort2,
            )
        });

        let threads = vec![pow_thread, time_thread];
        for t in threads {
            match t.join().unwrap() {
                Ok(res) => {
                    if res.0 != 0 || !local_pow {
                        if let Some(message) = res.1 {
                            return Ok(message);
                        }
                    }
                    done.swap(false, Ordering::Relaxed);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

fn pow_timeout(after_seconds: u64, done: &AtomicBool) -> (u64, Option<Message>) {
    std::thread::sleep(std::time::Duration::from_secs(after_seconds));
    done.swap(true, Ordering::Relaxed);
    (0, None)
}

/// Does PoW
pub fn do_pow(
    client_miner: ClientMiner,
    min_pow_score: f64,
    network_id: u64,
    payload: Option<Payload>,
    parent_messages: Vec<MessageId>,
    done: Arc<AtomicBool>,
) -> Result<(u64, Option<Message>)> {
    let mut message = MessageBuilder::<ClientMiner>::new();
    message = message.with_network_id(network_id);
    if let Some(p) = payload {
        message = message.with_payload(p);
    }
    let message = message
        .with_parents(parent_messages)
        .with_nonce_provider(client_miner, min_pow_score, Some(Arc::clone(&done)))
        .finish()
        .map_err(Error::MessageError)?;
    Ok((message.nonce(), Some(message)))
}
