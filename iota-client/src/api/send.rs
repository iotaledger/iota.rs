use crate::{Client, Error, Result};

use bee_message::prelude::*;
use bee_signing_ext::{binary::BIP32Path, Seed};

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

        if self.outputs.len() == 0 {
            return Err(Error::MissingParameter(String::from("Outputs")));
        }

        let mut balance = 0;
        let mut inputs = Vec::new();
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
                    let curr_outputs = self.client.get_output(&output_id.0, output_id.1).await?;
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

                                inputs.push((
                                    UTXOInput::new(
                                        TransactionId::from(transaction_id),
                                        output.output_index,
                                    )
                                    .map_err(|_| Error::TransactionError)?
                                    .into(),
                                    address_path,
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
        let total = outputs.iter().fold(0, |acc, x| {
            let Output::SignatureLockedSingle(x) = x;
            acc + &x.amount().get()
        });
        if balance <= total {
            return Err(Error::NotEnoughBalance(balance));
        }
        // TODO overflow check?
        let payload = TransactionBuilder::new(self.seed)
            .set_inputs(inputs)
            .set_outputs(outputs)
            .build()
            .map_err(|_| Error::TransactionError)?;

        // get tips
        let (parent1, parent2) = (MessageId::new([0; 32]), MessageId::new([0; 32])); //self.client.get_tips()?;

        // building message
        let payload = Payload::Transaction(Box::new(payload));
        let message = Message::builder()
            .parent1(parent1)
            .parent2(parent2)
            .payload(payload)
            .build()
            .map_err(|_| Error::TransactionError)?;

        self.client.post_message(&message).await
    }
}
