// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, ClientMiner, Error, Result};

use bee_common::packable::Packable;
use bee_message::prelude::*;
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey},
    Seed, Signer,
};
use std::{collections::HashMap, convert::TryInto, num::NonZeroU64};

/// Builder of send API
pub struct SendBuilder<'a> {
    client: &'a Client,
}

impl<'a> SendBuilder<'a> {
    /// Create send builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }
    /// Build a transaction message
    pub fn transaction(self, seed: &'a Seed) -> SendTransactionBuilder<'_> {
        SendTransactionBuilder::new(self.client, seed)
    }
    /// Build an indexation message
    pub fn indexation<S: Into<String>>(self, index: S) -> SendIndexationBuilder<'a> {
        SendIndexationBuilder::new(self.client, index.into())
    }
}

const HARDEND: u32 = 1 << 31;
const TRANSACTION_ID_LENGTH: usize = 32;

/// Builder for transaction messages
pub struct SendTransactionBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    account_index: Option<usize>,
    initial_address_index: Option<usize>,
    outputs: Vec<Output>,
    indexation: Option<Indexation>,
}

/// Structure for sorting of UnlockBlocks
// TODO: move the sorting process to the `Message` crate
struct AddressIndexRecorder {
    input: Input,
    address_index: usize,
    address_path: BIP32Path,
}

impl<'a> SendTransactionBuilder<'a> {
    /// Create send builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            account_index: None,
            initial_address_index: None,
            outputs: Vec::new(),
            indexation: None,
        }
    }

    /// Sets the account index.
    pub fn account_index(mut self, account_index: usize) -> Self {
        self.account_index = Some(account_index);
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn initial_address_index(mut self, initial_address_index: usize) -> Self {
        self.initial_address_index = Some(initial_address_index);
        self
    }

    /// Set transfers to the builder
    pub fn output(mut self, address: Address, amount: NonZeroU64) -> Self {
        let output = SignatureLockedSingleOutput::new(address, amount).into();
        self.outputs.push(output);
        self
    }

    /// Set indexation payload to the builder
    pub fn indexation(mut self, indexation_payload: Indexation) -> Self {
        self.indexation = Some(indexation_payload);
        self
    }

    /// Consume the builder and get the API result
    pub async fn post(self) -> Result<MessageId> {
        let account_index = self
            .account_index
            .ok_or_else(|| Error::MissingParameter(String::from("account index")))?;
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
                total_to_spend += x.amount().get();
            }
        }

        let mut paths = Vec::new();
        let mut essence = TransactionEssence::builder();
        let mut address_index_recorders = Vec::new();

        'input_selection: loop {
            // Reset the empty_address_count for each run of output address searching
            let mut empty_address_count = 0;

            // Get the addresses in the BIP path/index ~ path/index+20
            let addresses = self
                .client
                .find_addresses(self.seed)
                .account_index(account_index)
                .range(index..index + 20)
                .get()?;

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
                                });
                                // Output the remaining tokens back to the original address
                                if total_already_spent > total_to_spend {
                                    essence = essence.add_output(
                                        SignatureLockedSingleOutput::new(
                                            address.clone(),
                                            NonZeroU64::new(total_already_spent - total_to_spend).unwrap(),
                                        )
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

        if total_already_spent < total_to_spend {
            return Err(Error::NotEnoughBalance(total_to_spend));
        }

        // Build signed transaction payload
        let outputs = self.outputs;
        for output in outputs {
            essence = essence.add_output(output);
        }
        if let Some(indexation_payload) = self.indexation {
            essence = essence.with_payload(Payload::Indexation(Box::new(indexation_payload)))
        }
        let essence = essence.finish()?;
        let mut serialized_essence = Vec::new();
        essence
            .pack(&mut serialized_essence)
            .map_err(|_| Error::InvalidParameter("inputs".to_string()))?;

        let mut unlock_blocks = Vec::new();
        let mut current_block_index: usize = 0;
        let mut signature_indexes = HashMap::<usize, usize>::new();
        address_index_recorders.sort_by(|a, b| a.input.cmp(&b.input));

        for recorder in address_index_recorders.iter() {
            // Check if current path is same as previous path
            // If so, add a reference unlock block
            if let Some(block_index) = signature_indexes.get(&recorder.address_index) {
                unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
            } else {
                // If not, we should create a signature unlock block
                match &self.seed {
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
                signature_indexes.insert(recorder.address_index, current_block_index);

                // Update current block index
                current_block_index += 1;
            }
        }
        // TODO overflow check
        let mut payload_builder = TransactionBuilder::new().with_essence(essence);
        for unlock in unlock_blocks {
            payload_builder = payload_builder.add_unlock_block(unlock);
        }

        let payload = payload_builder.finish().map_err(|_| Error::TransactionError)?;

        // get tips
        let tips = self.client.get_tips().await?;

        // building message
        let payload = Payload::Transaction(Box::new(payload));
        let message = MessageBuilder::<ClientMiner>::new()
            .with_network_id(self.client.get_network_id().await?)
            .with_parent1(tips.0)
            .with_parent2(tips.1)
            .with_payload(payload)
            .with_nonce_provider(self.client.get_pow_provider(), 4000f64)
            .finish()
            .map_err(|_| Error::TransactionError)?;

        self.client.post_message(&message).await
    }
}

/// Builder for indexation messages
pub struct SendIndexationBuilder<'a> {
    client: &'a Client,
    index: String,
    data: Option<Vec<u8>>,
}

impl<'a> SendIndexationBuilder<'a> {
    /// Create send builder
    pub fn new(client: &'a Client, index: String) -> Self {
        Self {
            client,
            index,
            data: None,
        }
    }

    /// Set data to the builder
    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Consume the builder and get the API result
    pub async fn post(self) -> Result<MessageId> {
        let index = self.index;
        let data = self.data.unwrap_or_default();

        // build indexation
        let index = Indexation::new(index, &data).map_err(|e| Error::IndexationError(e.to_string()))?;

        // get tips
        let tips = self.client.get_tips().await?;

        // building message
        let payload = Payload::Indexation(Box::new(index));
        let message = MessageBuilder::<ClientMiner>::new()
            .with_network_id(self.client.get_network_id().await?)
            .with_parent1(tips.0)
            .with_parent2(tips.1)
            .with_payload(payload)
            .with_nonce_provider(self.client.get_pow_provider(), 4000f64)
            .finish()
            .map_err(|e| Error::IndexationError(e.to_string()))?;

        self.client.post_message(&message).await
    }
}
