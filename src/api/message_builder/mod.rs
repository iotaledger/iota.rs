// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "wasm"))]
use crate::api::{do_pow, pow::finish_pow};
use crate::{
    api::types::{AddressIndexRecorder, PreparedTransactionData},
    signing::SignerHandle,
    Client, Error, Result,
};

use bee_message::{
    address::{Address, Ed25519Address},
    input::{Input, UtxoInput},
    output::{ExtendedOutput, Output},
    payload::{transaction::TransactionId, Payload, TaggedDataPayload},
    Message, MessageId,
};
#[cfg(not(feature = "wasm"))]
use bee_pow::providers::NonceProviderBuilder;
use bee_rest_api::types::{
    dtos::{AddressDto, OutputDto},
    responses::OutputResponse,
};
use crypto::keys::slip10::Chain;
use packable::PackableExt;
#[cfg(not(feature = "wasm"))]
use tokio::time::sleep;

use std::{ops::Range, str::FromStr, time::Duration};

mod input_selection;
pub mod pow;
pub mod transaction;
use input_selection::{get_custom_inputs, get_inputs};
use transaction::{prepare_transaction, sign_transaction};

const DUST_THRESHOLD: u64 = 1_000_000;

/// Builder of the message API
pub struct ClientMessageBuilder<'a> {
    client: Client,
    signer: Option<&'a SignerHandle>,
    account_index: Option<u32>,
    initial_address_index: Option<u32>,
    inputs: Option<Vec<UtxoInput>>,
    input_range: Range<u32>,
    outputs: Vec<Output>,
    index: Option<Box<[u8]>>,
    data: Option<Vec<u8>>,
    parents: Option<Vec<MessageId>>,
}

impl<'a> ClientMessageBuilder<'a> {
    /// Create message builder
    pub fn new(client: Client) -> Self {
        Self {
            client,
            signer: None,
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
    pub fn with_signer(mut self, signer: &'a SignerHandle) -> Self {
        self.signer.replace(signer);
        self
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: u32) -> Self {
        self.account_index.replace(account_index);
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: u32) -> Self {
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

    /// Set a custom range in which to search for addresses for custom provided inputs. Default: 0..100
    pub fn with_input_range(mut self, range: Range<u32>) -> Self {
        self.input_range = range;
        self
    }

    /// Set a transfer to the builder
    pub fn with_output(mut self, address: &str, amount: u64) -> Result<Self> {
        let output = ExtendedOutput::new(Address::from_str(address)?, amount);
        self.outputs.push(Output::Extended(output));
        Ok(self)
    }

    /// Set outputs to the builder
    pub fn with_outputs(mut self, outputs: Vec<Output>) -> Result<Self> {
        // todo validate length
        self.outputs.extend(outputs);
        Ok(self)
    }

    /// Set a transfer to the builder, address needs to be hex encoded
    pub fn with_output_hex(mut self, address: &str, amount: u64) -> Result<Self> {
        let output = ExtendedOutput::new(address.parse::<Ed25519Address>()?.into(), amount);
        self.outputs.push(Output::Extended(output));
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
            if self.signer.is_none() && self.inputs.is_none() {
                return Err(Error::MissingParameter("Seed"));
            }
            // Send message with transaction
            let prepared_transaction_data = self.prepare_transaction().await?;
            let tx_payload = self.sign_transaction(prepared_transaction_data).await?;
            self.finish_message(Some(tx_payload)).await
        } else if self.index.is_some() {
            // Send message with indexation payload
            self.finish_indexation().await
        } else {
            // Send message without payload
            self.finish_message(None).await
        }
    }

    // Used to store the address data for an input so we can later sign it
    fn create_address_index_recorder(
        account_index: u32,
        address_index: u32,
        internal: bool,
        output: &OutputResponse,
        bech32_address: String,
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
            account_index,
            input,
            output: output.clone(),
            address_index,
            chain,
            internal,
            bech32_address,
        })
    }

    /// Get output amount and address from an OutputDto
    pub fn get_output_amount_and_address(output: &OutputDto) -> Result<(u64, Address)> {
        match output {
            OutputDto::Treasury(_) => Err(Error::OutputError("Treasury output is no supported")),
            OutputDto::Extended(ref r) => match &r.address {
                AddressDto::Ed25519(addr) => {
                    let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                    Ok((r.amount, output_address))
                }
                // todo support other addresses
                _ => Err(Error::OutputError("Only Ed25519Address is implemented")),
            },
            // todo add other outputs
            _ => Err(Error::OutputError("Output is not implemented")),
        }
    }

    // If custom inputs are provided we check if they are unspent, get the balance and search the address for it
    async fn get_custom_inputs(
        &self,
        inputs: &[UtxoInput],
        total_to_spend: u64,
    ) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
        get_custom_inputs(self, inputs, total_to_spend).await
    }

    // Searches inputs for an amount which a user wants to spend, also checks that it doesn't create dust
    async fn get_inputs(&self, total_to_spend: u64) -> Result<(Vec<Input>, Vec<Output>, Vec<AddressIndexRecorder>)> {
        get_inputs(self, total_to_spend).await
    }

    /// Prepare a transaction
    pub async fn prepare_transaction(&self) -> Result<PreparedTransactionData> {
        prepare_transaction(self).await
    }

    /// Sign the transaction
    pub async fn sign_transaction(&self, prepared_transaction_data: PreparedTransactionData) -> Result<Payload> {
        sign_transaction(self, prepared_transaction_data).await
    }

    /// Consume the builder and get the API result
    pub async fn finish_indexation(self) -> Result<Message> {
        let payload: Payload;
        {
            let index = &self.index.as_ref();
            let empty_slice = &vec![];
            let data = &self.data.as_ref().unwrap_or(empty_slice);

            // build indexation
            let index = TaggedDataPayload::new(index.expect("No indexation tag").to_vec(), data.to_vec())
                .map_err(|e| Error::IndexationError(e.to_string()))?;
            payload = Payload::TaggedData(Box::new(index));
        }

        // building message
        self.finish_message(Some(payload)).await
    }

    /// Builds the final message and posts it to the node
    pub async fn finish_message(self, payload: Option<Payload>) -> Result<Message> {
        #[cfg(feature = "wasm")]
        let final_message = {
            let parent_message_ids = match self.parents {
                Some(parents) => parents,
                _ => self.client.get_tips().await?,
            };
            let min_pow_score = self.client.get_min_pow_score().await?;
            let network_id = self.client.get_network_id().await?;
            crate::api::pow::finish_single_thread_pow(
                self.client,
                network_id,
                Some(parent_message_ids),
                payload,
                min_pow_score,
            )
            .await?
        };
        #[cfg(not(feature = "wasm"))]
        let final_message = match self.parents {
            Some(mut parents) => {
                // Sort parents
                parents.sort_unstable_by_key(|a| a.pack_to_vec());
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
        match self.client.get_local_pow().await {
            true => Ok(final_message),
            false => {
                // Request message multiple times because the node maybe didn't process it completely in this time
                // or a node balancer could be used which forwards the request to different node than we published
                for time in 1..3 {
                    if let Ok(message) = self.client.get_message().data(&msg_id).await {
                        return Ok(message);
                    }
                    #[cfg(not(feature = "wasm"))]
                    sleep(Duration::from_millis(time * 50)).await;
                    #[cfg(feature = "wasm")]
                    {
                        use wasm_timer::Delay;
                        Delay::new(Duration::from_millis(time * 50)).await?;
                    }
                }
                self.client.get_message().data(&msg_id).await
            }
        }
    }
}
