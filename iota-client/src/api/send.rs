use crate::{Client, Error, Result};

use bee_signing_ext::{binary::BIP32Path, Seed};
use bee_transaction::prelude::*;

use std::num::NonZeroU64;

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
        let output = SigLockedSingleDeposit::new(address, amount).into();
        self.outputs.push(output);
        self
    }

    /// Consume the builder and get the API result
    pub fn post(self) -> Result<Vec<Hash>> {
        let path = match self.path {
            Some(p) => p,
            None => return Err(Error::MissingParameter),
        };

        let mut index = match self.index {
            Some(r) => r,
            None => 0,
        };

        if self.outputs.len() == 0 {
            return Err(Error::MissingParameter);
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

            let outputs = self.client.get_outputs().addresses(&addresses).get()?;

            let mut end = false;
            for (offset, output) in outputs.into_iter().enumerate() {
                match output.spent {
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
                            inputs.push((
                                UTXOInput::new(output.producer, output.output_index).into(),
                                address_path,
                            ));
                        } else {
                            end = true;
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
            let Output::SigLockedSingleDeposit(x) = x;
            acc + &x.amount().get()
        });
        if balance <= total {
            return Err(Error::NotEnoughBalance(balance));
        }
        // TODO overflow check?
        let payload = SignedTransactionBuilder::new(self.seed)
            .set_inputs(inputs)
            .set_outputs(outputs)
            .build()
            .map_err(|_| Error::TransactionError)?;

        // get tips
        let tips = self.client.get_tips()?;

        // building message
        let payload = Payload::SignedTransaction(Box::new(payload));
        let message = Message::builder()
            .tips(tips)
            .payload(payload)
            .build()
            .map_err(|_| Error::TransactionError)?;

        self.client.post_messages(vec![message])
    }
}
