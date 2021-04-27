// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    api::address::search_address,
    Client, ClientMiner, Error, Result,
};

use bee_common::packable::Packable;
use bee_message::prelude::*;
#[cfg(not(feature = "wasm"))]
use bee_pow::providers::{miner::MinerCancel, NonceProviderBuilder};
use bee_rest_api::types::{
    dtos::{AddressDto, OutputDto},
    responses::OutputResponse,
};
use crypto::keys::slip10::{Chain, Curve, Seed};

use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

// https://github.com/GalRogozinski/protocol-rfcs/blob/dust/text/0032-dust-protection/0032-dust-protection.md
const MAX_ALLOWED_DUST_OUTPUTS: i64 = 100;
const DUST_DIVISOR: i64 = 100_000;
const DUST_THRESHOLD: u64 = 1_000_000;

/// Structure for sorting of UnlockBlocks
// TODO: move the sorting process to the `Message` crate
struct AddressIndexRecorder {
    input: Input,
    address_index: usize,
    chain: Chain,
    internal: bool,
}

#[derive(Debug, Clone)]
struct OutputWrapper {
    output: OutputResponse,
    address_index: usize,
    internal: bool,
    amount: u64,
    address: bee_message::address::Address,
    check_treshold: bool,
}

/// Builder of send API
pub struct ClientMessageBuilder<'a> {
    client: &'a Client,
    seed: Option<&'a Seed>,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
    inputs: Option<Vec<UtxoInput>>,
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
        self.seed.replace(seed);
        self
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: usize) -> Self {
        self.account_index.replace(account_index);
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: usize) -> Self {
        self.initial_address_index.replace(initial_address_index);
        self
    }

    /// Set a custom input(transaction output)
    pub fn with_input(mut self, input: UtxoInput) -> Self {
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

    /// Set a transfer to the builder
    pub fn with_output(mut self, address: &str, amount: u64) -> Result<Self> {
        let output = SignatureLockedSingleOutput::new(Address::from_str(address)?, amount)?.into();
        self.outputs.push(output);
        Ok(self)
    }

    /// Set a dust allowance transfer to the builder, address needs to be Bech32 encoded
    pub fn with_dust_allowance_output(mut self, address: &str, amount: u64) -> Result<Self> {
        if amount < DUST_THRESHOLD {
            return Err(Error::DustError(
                "Amount for SignatureLockedDustAllowanceOutput needs to be >= 1_000_000".into(),
            ));
        }
        let output = SignatureLockedDustAllowanceOutput::new(Address::from_str(address)?, amount)?.into();
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
        self.index.replace(index.as_ref().into());
        self
    }

    /// Set data to the builder
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data.replace(data);
        self
    }

    /// Set 1-8 custom parent message ids
    pub fn with_parents(mut self, parent_ids: Vec<MessageId>) -> Result<Self> {
        if !(1..=8).contains(&parent_ids.len()) {
            return Err(Error::InvalidParentsAmount(parent_ids.len()));
        }
        self.parents.replace(parent_ids);
        Ok(self)
    }

    /// Consume the builder and get the API result
    pub async fn finish(self) -> Result<Message> {
        // Indexation payload requires an indexation tag
        if self.data.is_some() && self.index.is_none() {
            return Err(Error::MissingParameter("index"));
        }
        if self.inputs.is_some() && self.outputs.is_empty() {
            return Err(Error::MissingParameter("output"));
        }
        if !self.outputs.is_empty() {
            if self.seed.is_none() {
                return Err(Error::MissingParameter("Seed"));
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

    fn create_address_index_recorder(
        account_index: usize,
        address_index: usize,
        internal: bool,
        output: &OutputResponse,
    ) -> Result<AddressIndexRecorder> {
        // Note that we need to sign the original address, i.e., `path/index`,
        // instead of `path/index/_offset` or `path/_offset`.

        // 44 is for BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        let chain = Chain::from_u32_hardened(vec![
            44,
            4218,
            account_index as u32,
            internal as u32,
            address_index as u32,
        ]);
        let input = Input::Utxo(
            UtxoInput::new(TransactionId::from_str(&output.transaction_id)?, output.output_index)
                .map_err(|_| Error::TransactionError)?,
        );

        Ok(AddressIndexRecorder {
            input,
            address_index,
            chain,
            internal,
        })
    }

    fn get_output_amount_and_address(output: &OutputDto) -> Result<(u64, Address, bool)> {
        match output {
            OutputDto::Treasury(_) => Err(Error::OutputError("Treasury output is no supported")),
            OutputDto::SignatureLockedSingle(ref r) => match &r.address {
                AddressDto::Ed25519(addr) => {
                    let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                    Ok((r.amount, output_address, true))
                }
            },
            OutputDto::SignatureLockedDustAllowance(ref r) => match &r.address {
                AddressDto::Ed25519(addr) => {
                    let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                    Ok((r.amount, output_address, false))
                }
            },
        }
    }

    async fn get_custom_inputs(
        &self,
        inputs: &[UtxoInput],
        total_to_spend: u64,
        dust_and_allowance_recorders: &mut Vec<(u64, Address, bool)>,
    ) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
        let mut inputs_for_essence = Vec::new();
        let mut outputs_for_essence = Vec::new();
        let mut address_index_recorders = Vec::new();
        let mut remainder_address_balance: (Option<Address>, u64) = (None, 0);
        let mut total_already_spent = 0;
        let account_index = self.account_index.unwrap_or(0);
        for input in inputs {
            // Only add unspent outputs
            if let Ok(output) = self.client.get_output(&input).await {
                if !output.is_spent {
                    let (output_amount, output_address, check_treshold) =
                        ClientMessageBuilder::get_output_amount_and_address(&output.output)?;
                    if !check_treshold || output_amount < DUST_THRESHOLD {
                        dust_and_allowance_recorders.push((output_amount, output_address, false));
                    }

                    total_already_spent += output_amount;
                    let bech32_hrp = self.client.get_bech32_hrp().await?;
                    let (address_index, internal) = search_address(
                        self.seed.ok_or(crate::Error::MissingParameter("seed"))?,
                        bech32_hrp,
                        account_index,
                        self.input_range.clone(),
                        &output_address,
                    )
                    .await?;

                    let address_index_record = ClientMessageBuilder::create_address_index_recorder(
                        account_index,
                        address_index,
                        internal,
                        &output,
                    )?;
                    inputs_for_essence.push(address_index_record.input.clone());
                    address_index_recorders.push(address_index_record);
                    // Output the remaining tokens back to the original address
                    if total_already_spent > total_to_spend {
                        let remaining_balance = total_already_spent - total_to_spend;
                        // Keep track of remaining balance, we don't add an output here, because we could have
                        // multiple inputs from the same address, which would create multiple outputs with the
                        // same address, which is not allowed
                        remainder_address_balance = (Some(output_address), remaining_balance);
                    }
                }
            }
        }
        // Add output from remaining balance of custom inputs if necessary
        if let Some(address) = remainder_address_balance.0 {
            if remainder_address_balance.1 < DUST_THRESHOLD {
                dust_and_allowance_recorders.push((remainder_address_balance.1, address, true));
            }
            outputs_for_essence.push(SignatureLockedSingleOutput::new(address, remainder_address_balance.1)?.into());
        }

        if total_already_spent < total_to_spend {
            return Err(Error::NotEnoughBalance(total_already_spent, total_to_spend));
        }

        Ok((inputs_for_essence, outputs_for_essence, address_index_recorders))
    }

    async fn get_inputs(
        &self,
        total_to_spend: u64,
        dust_and_allowance_recorders: &mut Vec<(u64, Address, bool)>,
    ) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
        let mut outputs = Vec::new();
        let mut dust_outputs = Vec::new();
        let mut inputs_for_essence = Vec::new();
        let mut outputs_for_essence = Vec::new();
        let mut address_index_recorders = Vec::new();
        let mut total_already_spent = 0;
        let account_index = self.account_index.unwrap_or(0);
        let mut gap_index = self.initial_address_index.unwrap_or(0);
        let mut empty_address_count: u64 = 0;
        'input_selection: loop {
            // Get the addresses in the BIP path/index ~ path/index+20
            let addresses = self
                .client
                .get_addresses(self.seed.ok_or(crate::Error::MissingParameter("seed"))?)
                .with_account_index(account_index)
                .with_range(gap_index..gap_index + super::ADDRESS_GAP_RANGE)
                .get_all()
                .await?;
            // For each address, get the address outputs
            let mut address_index = gap_index;
            for (index, (address, internal)) in addresses.iter().enumerate() {
                let address_outputs = self.client.get_address().outputs(&address, Default::default()).await?;
                for output_id in address_outputs.iter() {
                    let output = self.client.get_output(output_id).await?;
                    if !output.is_spent {
                        let (amount, address, check_treshold) =
                            ClientMessageBuilder::get_output_amount_and_address(&output.output)?;
                        if !check_treshold || amount < DUST_THRESHOLD {
                            dust_and_allowance_recorders.push((amount, address, false));
                        }
                        let output_wrapper = OutputWrapper {
                            output,
                            address_index,
                            internal: *internal,
                            amount,
                            address,
                            check_treshold,
                        };
                        match output_wrapper.output.output {
                            OutputDto::SignatureLockedSingle(_) => outputs.push(output_wrapper),
                            OutputDto::SignatureLockedDustAllowance(_) => dust_outputs.push(output_wrapper),
                            OutputDto::Treasury(_) => {}
                        };
                    }
                }

                // If there are more than 20 (ADDRESS_GAP_RANGE) consecutive empty addresses, then we stop
                // looking up the addresses belonging to the seed. Note that we don't
                // really count the exact 20 consecutive empty addresses, which is
                // unnecessary. We just need to check the address range,
                // (index * ADDRESS_GAP_RANGE, index * ADDRESS_GAP_RANGE + ADDRESS_GAP_RANGE), where index is
                // natural number, and to see if the outputs are all empty.
                if outputs.is_empty() {
                    // Accumulate the empty_address_count for each run of output address searching
                    empty_address_count += 1;
                } else {
                    // Reset counter if there is an output
                    empty_address_count = 0;
                }

                outputs.sort_by(|l, r| l.amount.cmp(&r.amount));
                for (_offset, output_wrapper) in [outputs.clone(), dust_outputs.clone()].concat().iter().enumerate() {
                    if total_already_spent < total_to_spend {
                        total_already_spent += output_wrapper.amount;
                        let address_index_record = ClientMessageBuilder::create_address_index_recorder(
                            account_index,
                            output_wrapper.address_index,
                            output_wrapper.internal,
                            &output_wrapper.output,
                        )?;
                        inputs_for_essence.push(address_index_record.input.clone());
                        address_index_recorders.push(address_index_record);
                        if total_already_spent > total_to_spend {
                            let remaining_balance = total_already_spent - total_to_spend;
                            if remaining_balance < DUST_THRESHOLD {
                                dust_and_allowance_recorders.push((
                                    remaining_balance,
                                    Address::try_from_bech32(address)?,
                                    true,
                                ));
                            }
                            // Output the remaining tokens back to the original address
                            outputs_for_essence.push(
                                SignatureLockedSingleOutput::new(
                                    Address::try_from_bech32(address)?,
                                    remaining_balance,
                                )?
                                .into(),
                            );
                        }
                    }
                }
                // Break if we have enough funds and don't create dust for the remainder
                if total_already_spent == total_to_spend || total_already_spent >= total_to_spend + DUST_THRESHOLD {
                    break 'input_selection;
                }
                // if we just processed an even index, increase the address index
                // (because the list has public and internal addresses)
                // TODO: remove - even: public
                if index % 2 == 1 {
                    address_index += 1;
                }
            }
            gap_index += super::ADDRESS_GAP_RANGE;
            // The gap limit is 20 and use reference 40 here because there's public and internal addresses
            if empty_address_count >= (super::ADDRESS_GAP_RANGE * 2) as u64 {
                break;
            }
        }

        if total_already_spent < total_to_spend {
            return Err(Error::NotEnoughBalance(total_already_spent, total_to_spend));
        }

        Ok((inputs_for_essence, outputs_for_essence, address_index_recorders))
    }

    /// Consume the builder and get the API result
    pub async fn finish_transaction(self) -> Result<Message> {
        // store (amount, address, new_created) to check later if dust is allowed
        let mut dust_and_allowance_recorders = Vec::new();

        // Calculate the total tokens to spend
        let mut total_to_spend = 0;
        for output in &self.outputs {
            match output {
                Output::SignatureLockedSingle(x) => {
                    total_to_spend += x.amount();
                    if x.amount() < DUST_THRESHOLD {
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

        let (mut inputs_for_essence, mut outputs_for_essence, mut address_index_recorders) = match &self.inputs {
            Some(inputs) => {
                self.get_custom_inputs(inputs, total_to_spend, dust_and_allowance_recorders.as_mut())
                    .await?
            }
            None => {
                self.get_inputs(total_to_spend, dust_and_allowance_recorders.as_mut())
                    .await?
            }
        };

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
            is_dust_allowed(&self.client, address, created_or_consumed_outputs).await?;
        }

        // Build signed transaction payload
        for output in self.outputs.clone() {
            outputs_for_essence.push(output);
        }

        let mut essence = RegularEssence::builder();
        // Order inputs and add them to the essence
        inputs_for_essence.sort_unstable_by_key(|a| a.pack_new());
        essence = essence.with_inputs(inputs_for_essence);

        // Order outputs and add them to the essence
        outputs_for_essence.sort_unstable_by_key(|a| a.pack_new());
        essence = essence.with_outputs(outputs_for_essence);

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
                    public_key, *signature,
                ))));
                signature_indexes.insert(index, current_block_index);
            }
        }

        let unlock_blocks = UnlockBlocks::new(unlock_blocks)?;
        let payload = TransactionPayloadBuilder::new()
            .with_essence(essence)
            .with_unlock_blocks(unlock_blocks)
            .finish()
            .map_err(|_| Error::TransactionError)?;
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
        #[cfg(feature = "wasm")]
        let final_message = match self.parents {
            Some(mut parents) => {
                // Sort parents
                parents.sort_unstable_by_key(|a| a.pack_new());
                parents.dedup();

                let network_id = self.client.get_network_id().await?;
                let mut message = MessageBuilder::<ClientMiner>::new();
                message = message.with_network_id(network_id);
                if let Some(p) = payload {
                    message = message.with_payload(p);
                }
                message
                    .with_parents(Parents::new(parents)?)
                    .finish()
                    .map_err(Error::MessageError)?
            }
            _ => {
                let network_id = self.client.get_network_id().await?;
                let tips = self.client.get_tips().await?;
                let mut message = MessageBuilder::<ClientMiner>::new();
                message = message.with_network_id(network_id).with_parents(Parents::new(tips)?);
                if let Some(p) = payload {
                    message = message.with_payload(p);
                }
                message.finish().map_err(Error::MessageError)?
            }
        };
        #[cfg(not(feature = "wasm"))]
        let final_message = match self.parents {
            Some(mut parents) => {
                // Sort parents
                parents.sort_unstable_by_key(|a| a.pack_new());
                parents.dedup();

                let min_pow_score = self.client.get_min_pow_score().await?;
                let network_id = self.client.get_network_id().await?;
                do_pow(
                    crate::client::ClientMinerBuilder::new()
                        .with_local_pow(self.client.get_local_pow().await)
                        .finish(),
                    min_pow_score,
                    network_id,
                    payload,
                    parents,
                )?
                .1
                .ok_or_else(|| Error::Pow("final message pow failed.".to_string()))?
            }
            None => finish_pow(&self.client, payload).await?,
        };

        let msg_id = self.client.post_message(&final_message).await?;
        // Get message if we use remote PoW, because the node will change parents and nonce
        let msg = match self.client.get_local_pow().await {
            true => final_message,
            false => self.client.get_message().data(&msg_id).await?,
        };

        Ok(msg)
    }
}

// Calculate the outputs on this address after this transaction gets confirmed so we know if we can send dust or
// dust allowance outputs (as input), the bool in the outputs defines if we consume this output (false) or create a new
// one (true)
async fn is_dust_allowed(client: &Client, address: Address, outputs: Vec<(u64, Address, bool)>) -> Result<()> {
    // balance of all dust allowance outputs
    let mut dust_allowance_balance: i64 = 0;
    // Amount of dust outputs
    let mut dust_outputs_amount: i64 = 0;

    // Add outputs from this transaction
    for (amount, _, add_outputs) in outputs {
        let sign = if add_outputs { 1 } else { -1 };
        if amount >= DUST_THRESHOLD {
            dust_allowance_balance += sign * amount as i64;
        } else {
            dust_outputs_amount += sign;
        }
    }

    let bech32_hrp = client.get_bech32_hrp().await?;

    let address_data = client.get_address().balance(&address.to_bech32(&bech32_hrp)).await?;
    // If we create a dust output and a dust allowance output we don't need to check more outputs if the balance/100_000
    // is < 100 because then we are sure that we didn't reach the max dust outputs
    if address_data.dust_allowed
        && dust_outputs_amount == 1
        && dust_allowance_balance >= 0
        && address_data.balance as i64 / DUST_DIVISOR < MAX_ALLOWED_DUST_OUTPUTS
    {
        return Ok(());
    } else if !address_data.dust_allowed && dust_outputs_amount == 1 && dust_allowance_balance <= 0 {
        return Err(Error::DustError(format!(
            "No dust output allowed on address {}",
            address.to_bech32(&bech32_hrp)
        )));
    }

    // Check all outputs of the address because we want to consume a dust allowance output and don't know if we are
    // allowed to do that
    let address_outputs_metadata = client.find_outputs(&[], &[address.to_bech32(&bech32_hrp)]).await?;
    for output_metadata in address_outputs_metadata {
        match output_metadata.output {
            OutputDto::Treasury(_) => {}
            OutputDto::SignatureLockedDustAllowance(d_a_o) => {
                dust_allowance_balance += d_a_o.amount as i64;
            }
            OutputDto::SignatureLockedSingle(s_o) => {
                if s_o.amount < DUST_THRESHOLD {
                    dust_outputs_amount += 1;
                }
            }
        }
    }

    // Here dust_allowance_balance and dust_outputs_amount should be as if this transaction gets confirmed
    // Max allowed dust outputs is 100
    let allowed_dust_amount = std::cmp::min(dust_allowance_balance / DUST_DIVISOR, MAX_ALLOWED_DUST_OUTPUTS);
    if dust_outputs_amount > allowed_dust_amount {
        return Err(Error::DustError(format!(
            "No dust output allowed on address {}",
            address.to_bech32(&bech32_hrp)
        )));
    }
    Ok(())
}

/// Does PoW with always new tips
#[cfg(not(feature = "wasm"))]
pub async fn finish_pow(client: &Client, payload: Option<Payload>) -> Result<Message> {
    let local_pow = client.get_local_pow().await;
    let min_pow_score = client.get_min_pow_score().await?;
    let tips_interval = client.get_tips_interval().await;
    let network_id = client.get_network_id().await?;
    loop {
        let cancel = MinerCancel::new();
        let cancel_2 = cancel.clone();
        let payload_ = payload.clone();
        let parent_messages = client.get_tips().await?;
        let time_thread = std::thread::spawn(move || Ok(pow_timeout(tips_interval, cancel)));
        let pow_thread = std::thread::spawn(move || {
            do_pow(
                crate::client::ClientMinerBuilder::new()
                    .with_local_pow(local_pow)
                    .with_cancel(cancel_2)
                    .finish(),
                min_pow_score,
                network_id,
                payload_,
                parent_messages,
            )
        });

        let threads = vec![pow_thread, time_thread];
        for t in threads {
            match t.join().expect("Failed to join threads.") {
                Ok(res) => {
                    if res.0 != 0 || !local_pow {
                        if let Some(message) = res.1 {
                            return Ok(message);
                        }
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

#[cfg(not(feature = "wasm"))]
fn pow_timeout(after_seconds: u64, cancel: MinerCancel) -> (u64, Option<Message>) {
    std::thread::sleep(std::time::Duration::from_secs(after_seconds));
    cancel.trigger();
    (0, None)
}

/// Does PoW
pub fn do_pow(
    client_miner: ClientMiner,
    min_pow_score: f64,
    network_id: u64,
    payload: Option<Payload>,
    parent_messages: Vec<MessageId>,
) -> Result<(u64, Option<Message>)> {
    let mut message = MessageBuilder::<ClientMiner>::new();
    message = message.with_network_id(network_id);
    if let Some(p) = payload {
        message = message.with_payload(p);
    }
    let message = message
        .with_parents(Parents::new(parent_messages)?)
        .with_nonce_provider(client_miner, min_pow_score)
        .finish()
        .map_err(Error::MessageError)?;
    Ok((message.nonce(), Some(message)))
}
