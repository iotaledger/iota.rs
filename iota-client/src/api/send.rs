// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{api::address::search_address, Client, ClientMiner, Error, Result};

use bee_common::packable::Packable;
use bee_message::prelude::*;
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey},
    Seed, Signer,
};
use std::{collections::HashMap, convert::TryInto};

const HARDEND: u32 = 1 << 31;
const TRANSACTION_ID_LENGTH: usize = 32;

/// Structure for sorting of UnlockBlocks
// TODO: move the sorting process to the `Message` crate
struct AddressIndexRecorder {
    input: Input,
    address_index: usize,
    address_path: BIP32Path,
    internal: bool,
}

/// Builder of send API
pub struct SendBuilder<'a> {
    client: &'a Client,
    seed: Option<&'a Seed>,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
    inputs: Option<Vec<UTXOInput>>,
    outputs: Vec<Output>,
    index: Option<String>,
    data: Option<Vec<u8>>,
    parent: Option<MessageId>,
    network_id: Option<u64>,
}

impl<'a> SendBuilder<'a> {
    /// Create send builder
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            seed: None,
            account_index: None,
            initial_address_index: None,
            inputs: None,
            outputs: Vec::new(),
            index: None,
            data: None,
            parent: None,
            network_id: None,
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

    /// Set a transfer to the builder, address needs to be Bech32 encoded
    pub fn with_output(mut self, address: &Bech32Address, amount: u64) -> Result<Self> {
        let address = Address::try_from_bech32(&address.to_string())?;
        let output = SignatureLockedSingleOutput::new(address, amount).unwrap().into();
        self.outputs.push(output);
        Ok(self)
    }

    /// Set a transfer to the builder, address needs to be hex encoded
    pub fn with_output_hex(mut self, address: &str, amount: u64) -> Result<Self> {
        let output = SignatureLockedSingleOutput::new(address.parse::<Ed25519Address>()?.into(), amount)
            .unwrap()
            .into();
        self.outputs.push(output);
        Ok(self)
    }

    /// Set indexation string to the builder
    pub fn with_index(mut self, index: &str) -> Self {
        self.index = Some(index.to_string());
        self
    }

    /// Set data to the builder
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Set a custom parent
    pub fn with_parent(mut self, parent_id: MessageId) -> Self {
        self.parent = Some(parent_id);
        self
    }

    /// Set the network id
    pub fn with_network_id(mut self, network_id: u64) -> Self {
        self.network_id = Some(network_id);
        self
    }

    /// Consume the builder and get the API result
    pub async fn finish(self) -> Result<MessageId> {
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
    pub async fn finish_transaction(self) -> Result<MessageId> {
        let account_index = self.account_index.unwrap_or(0);
        let path = BIP32Path::from_str(&crate::account_path!(account_index)).expect("invalid account index");

        let mut index = self.initial_address_index.unwrap_or(0);

        if self.outputs.is_empty() {
            return Err(Error::MissingParameter(String::from("Outputs")));
        }

        // Calculate the total tokens to spend
        let mut total_to_spend = 0;
        let mut total_already_spent = 0;
        for output in &self.outputs {
            if let Output::SignatureLockedSingle(x) = &output {
                total_to_spend += x.amount();
            }
        }

        let mut paths = Vec::new();
        let mut essence = TransactionPayloadEssence::builder();
        let mut address_index_recorders = Vec::new();

        match self.inputs.clone() {
            Some(inputs) => {
                for input in inputs {
                    // Only add unspent outputs
                    if let Ok(output) = self.client.get_output(&input).await {
                        if !output.is_spent {
                            total_already_spent += output.amount;
                            let mut address_path = path.clone();
                            // Note that we need to sign the original address, i.e., `path/index`,
                            // instead of `path/index/_offset` or `path/_offset`.
                            // Todo: Make the range 0..100 configurable
                            let bech32_hrp = self.client.get_network_info().bech32_hrp;
                            let bech32_addresses = output.address.to_bech32(&bech32_hrp);
                            let (address_index, internal) = search_address(
                                &self.seed.expect("No seed"),
                                account_index,
                                0..100,
                                &bech32_addresses.into(),
                            )?;
                            address_path.push(internal as u32 + HARDEND);
                            address_path.push(address_index as u32 + HARDEND);
                            paths.push(address_path.clone());
                            let transaction_id: [u8; TRANSACTION_ID_LENGTH] = output.transaction_id[..]
                                .try_into()
                                .map_err(|_| Error::TransactionError)?;
                            let input = Input::UTXO(
                                UTXOInput::new(TransactionId::from(transaction_id), output.output_index)
                                    .map_err(|_| Error::TransactionError)?,
                            );
                            essence = essence.add_input(input.clone());
                            address_index_recorders.push(AddressIndexRecorder {
                                input,
                                address_index,
                                address_path,
                                internal,
                            });
                            // Output the remaining tokens back to the original address
                            if total_already_spent > total_to_spend {
                                essence = essence.add_output(
                                    SignatureLockedSingleOutput::new(
                                        output.address.clone(),
                                        total_already_spent - total_to_spend,
                                    )
                                    .unwrap()
                                    .into(),
                                );
                            }
                        }
                    }
                }
            }
            None => {
                'input_selection: loop {
                    // Reset the empty_address_count for each run of output address searching
                    let mut empty_address_count = 0;
                    // Get the addresses in the BIP path/index ~ path/index+20
                    let addresses = self
                        .client
                        .find_addresses(self.seed.expect("No seed"))
                        .with_account_index(account_index)
                        .with_range(index..index + 20)
                        .get_all()?;
                    // For each address, get the address outputs
                    let mut address_index = 0;
                    for (index, (address, internal)) in addresses.iter().enumerate() {
                        let address_outputs = self.client.get_address().outputs(&address).await?;
                        let mut outputs = vec![];
                        for output_id in address_outputs.iter() {
                            let curr_outputs = self.client.get_output(output_id).await?;
                            outputs.push(curr_outputs);
                        }
                        // If there are more than 20 (gap limit) consecutive empty addresses, then we stop looking
                        // up the addresses belonging to the seed. Note that we don't really count the exact 20
                        // consecutive empty addresses, which is uncessary. We just need to check the address range,
                        // [k*20, k*20 + 20), where k is natural number, and to see if the outpus are all empty.
                        if outputs.is_empty() {
                            // Accumulate the empty_address_count for each run of output address searching
                            empty_address_count += 1;
                        }
                        for (_offset, output) in outputs.into_iter().enumerate() {
                            match output.is_spent {
                                true => {
                                    if output.amount != 0 {
                                        return Err(Error::SpentAddress);
                                    }
                                }
                                false => {
                                    if output.amount != 0 && total_already_spent < total_to_spend {
                                        total_already_spent += output.amount;
                                        let mut address_path = path.clone();
                                        // Note that we need to sign the original address, i.e., `path/index`,
                                        // instead of `path/index/_offset` or `path/_offset`.
                                        address_path.push(*internal as u32 + HARDEND);
                                        address_path.push(address_index as u32 + HARDEND);
                                        paths.push(address_path.clone());
                                        let transaction_id: [u8; TRANSACTION_ID_LENGTH] = output.transaction_id[..]
                                            .try_into()
                                            .map_err(|_| Error::TransactionError)?;
                                        let input = Input::UTXO(
                                            UTXOInput::new(TransactionId::from(transaction_id), output.output_index)
                                                .map_err(|_| Error::TransactionError)?,
                                        );
                                        essence = essence.add_input(input.clone());
                                        address_index_recorders.push(AddressIndexRecorder {
                                            input,
                                            address_index,
                                            address_path,
                                            internal: *internal,
                                        });
                                        // Output the remaining tokens back to the original address
                                        if total_already_spent > total_to_spend {
                                            essence = essence.add_output(
                                                SignatureLockedSingleOutput::new(
                                                    Address::try_from_bech32(address)?,
                                                    total_already_spent - total_to_spend,
                                                )
                                                .unwrap()
                                                .into(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        if total_already_spent > total_to_spend {
                            break 'input_selection;
                        }
                        // if we just processed an even index, increase the address index
                        // (because the list has public and internal addresses)
                        if index % 2 == 1 {
                            address_index += 1;
                        }
                    }
                    index += 20;
                    // The gap limit is 20 and use reference 40 here because there's public and internal addresses
                    if empty_address_count == 40 {
                        break;
                    }
                }
            }
        }

        if total_already_spent < total_to_spend {
            return Err(Error::NotEnoughBalance(total_to_spend));
        }

        // Build signed transaction payload
        for output in self.outputs.clone() {
            essence = essence.add_output(output);
        }
        // Add indexation_payload if index set
        if let Some(index) = self.index.clone() {
            let indexation_payload = IndexationPayload::new(index, &self.data.clone().unwrap_or_default())?;
            essence = essence.with_payload(Payload::Indexation(Box::new(indexation_payload)))
        }
        let essence = essence.finish()?;
        let mut serialized_essence = Vec::new();
        essence
            .pack(&mut serialized_essence)
            .map_err(|_| Error::InvalidParameter("inputs".to_string()))?;

        let mut unlock_blocks = Vec::new();
        let mut current_block_index: usize = 0;
        let mut signature_indexes = HashMap::<String, usize>::new();
        address_index_recorders.sort_by(|a, b| a.input.cmp(&b.input));

        for recorder in address_index_recorders.iter() {
            // Check if current path is same as previous path
            // If so, add a reference unlock block

            // Format to differentiate between public and private addresses
            let index = format!("{}{}", recorder.address_index, recorder.internal);
            if let Some(block_index) = signature_indexes.get(&index) {
                unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
            } else {
                // If not, we should create a signature unlock block
                match &self.seed.expect("No seed") {
                    Seed::Ed25519(s) => {
                        let private_key = Ed25519PrivateKey::generate_from_seed(s, &recorder.address_path)
                            .map_err(|_| Error::InvalidParameter("seed inputs".to_string()))?;
                        let public_key = private_key.generate_public_key().to_bytes();
                        // The block should sign the entire transaction essence part of the transaction payload
                        let signature = Box::new(private_key.sign(&serialized_essence).to_bytes());
                        unlock_blocks.push(UnlockBlock::Signature(SignatureUnlock::Ed25519(Ed25519Signature::new(
                            public_key, signature,
                        ))));
                    }
                    Seed::Wots(_) => panic!("Wots signing scheme isn't supported."),
                }
                signature_indexes.insert(index, current_block_index);

                // Update current block index
                current_block_index += 1;
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
    pub async fn finish_indexation(self) -> Result<MessageId> {
        let payload: Payload;
        {
            let index = &self.index.as_ref();
            let empty_slice = &vec![];
            let data = &self.data.as_ref().unwrap_or(empty_slice);

            // build indexation
            let index = IndexationPayload::new(index.expect("No indexation tag").to_string(), data)
                .map_err(|e| Error::IndexationError(e.to_string()))?;
            payload = Payload::Indexation(Box::new(index));
        }

        // building message
        self.finish_message(Some(payload)).await
    }

    /// Builds the final message and posts it to the node
    pub async fn finish_message(self, payload: Option<Payload>) -> Result<MessageId> {
        // get tips
        let tips = self.client.get_tips().await?;

        // building message
        let mut message = MessageBuilder::<ClientMiner>::new();

        match self.network_id {
            Some(id) => message = message.with_network_id(id),
            _ => message = message.with_network_id(self.client.get_network_id().await?),
        }

        match self.parent {
            Some(p) => message = message.with_parent1(p),
            _ => message = message.with_parent1(tips.0),
        }
        if let Some(p) = payload {
            message = message.with_payload(p);
        }
        let final_message = message
            .with_parent2(tips.1)
            .with_nonce_provider(self.client.get_pow_provider(), 4000f64)
            .finish()
            .map_err(Error::MessageError)?;

        self.client.post_message(&final_message).await
    }
}
