use crate::{Client, Error, Result};

use bee_common::packable::Packable;
use bee_message::prelude::*;
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey},
    Seed, Signer,
};
use std::{convert::TryInto, num::NonZeroU64};
const HARDEND: u32 = 1 << 31;
const TRANSACTION_ID_LENGTH: usize = 32;

/// Builder of send API
pub struct SendBuilder<'a> {
    client: &'a Client,
    seed: &'a Seed,
    path: Option<&'a BIP32Path>,
    index: Option<usize>,
    outputs: Vec<Output>,
}

impl<'a> SendBuilder<'a> {
    /// Create sned builder
    pub fn new(client: &'a Client, seed: &'a Seed) -> Self {
        Self {
            client,
            seed,
            path: None,
            index: None,
            outputs: Vec::new(),
        }
    }

    /// Set path to the builder
    pub fn path(mut self, path: &'a BIP32Path) -> Self {
        self.path = Some(path);
        self
    }

    /// Set index to the builder
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Set transfers to the builder
    pub fn output(mut self, address: Address, amount: NonZeroU64) -> Self {
        let output = SignatureLockedSingleOutput::new(address, amount).into();
        self.outputs.push(output);
        self
    }

    /// Consume the builder and get the API result
    pub async fn post(self) -> Result<MessageId> {
        let path = match self.path {
            Some(p) => p,
            None => return Err(Error::MissingParameter(String::from("BIP32 path"))),
        };

        let mut index = match self.index {
            Some(r) => r,
            None => 0,
        };

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
        let mut end = false;
        while !end {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .path(path)
                .range(index..index + 20)
                .get()?;

            for address in addresses {
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
                    end = true;
                    break;
                }

                for (offset, output) in outputs.into_iter().enumerate() {
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
                                address_path.push(offset as u32 + HARDEND);
                                let transaction_id: [u8; TRANSACTION_ID_LENGTH] = output
                                    .transaction_id[..]
                                    .try_into()
                                    .map_err(|_| Error::TransactionError)?;
                                paths.push(address_path);
                                essence = essence.add_input(Input::UTXO(
                                    UTXOInput::new(
                                        TransactionId::from(transaction_id),
                                        output.output_index,
                                    )
                                    .map_err(|_| Error::TransactionError)?,
                                ));

                                // Output the remaining tokens back to the original address
                                if total_already_spent > total_to_spend {
                                    essence = essence.add_output(
                                        SignatureLockedSingleOutput::new(
                                            address.clone(),
                                            NonZeroU64::new(total_already_spent - total_to_spend)
                                                .unwrap(),
                                        )
                                        .into(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            index += 20;
        }

        if total_already_spent < total_to_spend {
            return Err(Error::NotEnoughBalance(total_to_spend));
        }

        // Build signed transaction payload
        let outputs = self.outputs;
        for output in outputs {
            essence = essence.add_output(output);
        }
        let essence = essence.finish()?;
        let mut serialized_essence = Vec::new();
        essence
            .pack(&mut serialized_essence)
            .map_err(|_| Error::InvalidParameter("inputs".to_string()))?;

        let mut unlock_blocks = Vec::new();
        let mut last_index = (None, -1);
        for path in paths.iter() {
            // Check if current path is same as previous path
            if last_index.0 == Some(path) {
                // If so, add a reference unlock block
                unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlock::new(
                    last_index.1 as u16,
                )?));
            } else {
                // If not, we should create a signature unlock block
                match &self.seed {
                    Seed::Ed25519(s) => {
                        let private_key = Ed25519PrivateKey::generate_from_seed(s, path)
                            .map_err(|_| Error::InvalidParameter("seed inputs".to_string()))?;
                        let public_key = private_key.generate_public_key().to_bytes();
                        // The block should sign the entire transaction essence part of the transaction payload
                        let signature = Box::new(private_key.sign(&serialized_essence).to_bytes());
                        unlock_blocks.push(UnlockBlock::Signature(SignatureUnlock::Ed25519(
                            Ed25519Signature::new(public_key, signature),
                        )));
                    }
                    Seed::Wots(_) => panic!("Wots signing scheme isn't supported."),
                }

                // Update last signature block path and index
                last_index = (Some(path), (unlock_blocks.len() - 1) as isize);
            }
        }
        // TODO overflow check
        let mut payload_builder = TransactionBuilder::new().with_essence(essence);
        for unlock in unlock_blocks {
            payload_builder = payload_builder.add_unlock_block(unlock);
        }

        let payload = payload_builder
            .finish()
            .map_err(|_| Error::TransactionError)?;

        // get tips
        let tips = self.client.get_tips().await.unwrap();

        // building message
        let payload = Payload::Transaction(Box::new(payload));
        let message = Message::builder()
            // TODO: make the newtwork id configurable
            // TODO temporarily removed .with_network_id(0)
            .with_parent1(tips.0)
            .with_parent2(tips.1)
            .with_payload(payload)
            .finish()
            .map_err(|_| Error::TransactionError)?;

        self.client.post_message(&message).await
    }
}
