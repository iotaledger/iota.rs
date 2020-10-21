use crate::{Client, Error, Result};

use bee_common_ext::packable::Packable;
use bee_message::prelude::*;
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey},
    Seed, Signer,
};

use std::num::NonZeroU64;

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

        let mut balance = 0;
        let mut paths = Vec::new();
        let mut essence = TransactionEssence::builder();
        loop {
            let addresses = self
                .client
                .get_addresses(self.seed)
                .path(path)
                .range(index..index + 20)
                .get()?;

            let mut end = false;
            for address in addresses {
                let address_outputs = self.client.get_address().outputs(&address).await?;
                let mut outputs = vec![];
                for output_id in address_outputs.iter() {
                    let curr_outputs = self.client.get_output(output_id).await?;
                    outputs.push(curr_outputs);
                }
                for (offset, output) in outputs.into_iter().enumerate() {
                    match output.is_spent {
                        true => {
                            if output.amount != 0 {
                                return Err(Error::SpentAddress);
                            }
                        }
                        false => {
                            if output.amount != 0 {
                                balance += output.amount;
                                let mut address_path = path.clone();
                                address_path.push(offset as u32);

                                let mut transaction_id = [0u8; TRANSACTION_ID_LENGTH];
                                hex::decode_to_slice(output.transaction_id, &mut transaction_id)?;

                                paths.push(address_path);
                                essence = essence.add_input(Input::UTXO(
                                    UTXOInput::new(
                                        TransactionId::from(transaction_id),
                                        output.output_index,
                                    )
                                    .map_err(|_| Error::TransactionError)?,
                                ));
                            } else {
                                end = true;
                            }
                        }
                    }
                }
            }

            match end {
                true => break,
                false => index += 20,
            }
        }

        // Build signed transaction payload
        let outputs = self.outputs;
        let mut total = 0;
        for output in outputs {
            let Output::SignatureLockedSingle(x) = &output;
            total += x.amount().get();
            essence = essence.add_output(output);
        }
        if balance <= total {
            return Err(Error::NotEnoughBalance(balance));
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
        let (parent1, parent2) = (MessageId::new([0; 32]), MessageId::new([0; 32])); //self.client.get_tips()?;

        // building message
        let payload = Payload::Transaction(Box::new(payload));
        let message = Message::builder()
            .with_parent1(parent1)
            .with_parent2(parent2)
            .with_payload(payload)
            .finish()
            .map_err(|_| Error::TransactionError)?;

        self.client.post_message(&message).await
    }
}
