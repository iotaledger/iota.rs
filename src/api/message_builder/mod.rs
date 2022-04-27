// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod pow;
pub mod transaction;

use std::{collections::HashSet, ops::Range};

use bee_message::{
    address::{Address, Ed25519Address},
    input::{UtxoInput, INPUT_COUNT_MAX},
    output::{
        dto::OutputDto,
        unlock_condition::{dto::UnlockConditionDto, AddressUnlockCondition, UnlockCondition},
        AliasId, ByteCostConfig, Output, OUTPUT_COUNT_RANGE,
    },
    payload::{Payload, TaggedDataPayload},
    Message, MessageId,
};
#[cfg(feature = "wasm")]
use gloo_timers::future::TimeoutFuture;
use packable::bounded::{TryIntoBoundedU16Error, TryIntoBoundedU8Error};
#[cfg(not(feature = "wasm"))]
use {
    crate::api::{do_pow, miner::ClientMinerBuilder, pow::finish_pow},
    bee_pow::providers::NonceProviderBuilder,
    packable::PackableExt,
    std::time::Duration,
    tokio::time::sleep,
};

pub use self::transaction::verify_semantic;
use self::{
    input_selection::{get_custom_inputs, get_inputs},
    transaction::{prepare_transaction, sign_transaction},
};
use crate::{
    api::{input_selection::types::SelectedTransactionData, types::PreparedTransactionData},
    bee_message::{input::dto::UtxoInputDto, output::BasicOutputBuilder},
    constants::SHIMMER_COIN_TYPE,
    secret::SecretManager,
    Client, Error, Result,
};

/// Builder of the message API
pub struct ClientMessageBuilder<'a> {
    client: &'a Client,
    secret_manager: Option<&'a SecretManager>,
    coin_type: u32,
    account_index: u32,
    initial_address_index: u32,
    inputs: Option<Vec<UtxoInput>>,
    input_range: Range<u32>,
    outputs: Vec<Output>,
    custom_remainder_address: Option<Address>,
    tag: Option<Box<[u8]>>,
    data: Option<Vec<u8>>,
    parents: Option<Vec<MessageId>>,
    allow_burning: bool,
}

/// Message output address
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMessageBuilderOutputAddress {
    /// Address
    pub address: String,
    /// Amount
    // Using a String to prevent overflow issues in other languages
    pub amount: String,
}

/// Options for generating message
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMessageBuilderOptions {
    /// Coin type
    pub coin_type: Option<u32>,
    /// Account index
    pub account_index: Option<u32>,
    /// Initial address index
    pub initial_address_index: Option<u32>,
    /// Inputs
    pub inputs: Option<Vec<UtxoInputDto>>,
    /// Input range
    pub input_range: Option<Range<u32>>,
    /// Bech32 encoded output address and amount
    pub output: Option<ClientMessageBuilderOutputAddress>,
    /// Hex encoded output address and amount
    pub output_hex: Option<ClientMessageBuilderOutputAddress>,
    /// Outputs
    pub outputs: Option<Vec<OutputDto>>,
    /// Custom remainder address
    pub custom_remainder_address: Option<String>,
    /// Tag
    pub tag: Option<Box<[u8]>>,
    /// Data
    pub data: Option<Vec<u8>>,
    /// Parents
    pub parents: Option<Vec<MessageId>>,
    /// Allow burning of native tokens
    pub allow_burning: Option<bool>,
}

impl<'a> ClientMessageBuilder<'a> {
    /// Create message builder
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            secret_manager: None,
            coin_type: SHIMMER_COIN_TYPE,
            account_index: 0,
            initial_address_index: 0,
            inputs: None,
            input_range: 0..100,
            outputs: Vec::new(),
            custom_remainder_address: None,
            tag: None,
            data: None,
            parents: None,
            allow_burning: false,
        }
    }

    /// Allow burning of native tokens when custom inputs are provided.
    pub fn with_burning_allowed(mut self, allow_burning: bool) -> Self {
        self.allow_burning = allow_burning;
        self
    }

    /// Sets the seed.
    pub fn with_secret_manager(mut self, manager: &'a SecretManager) -> Self {
        self.secret_manager.replace(manager);
        self
    }

    /// Sets the coin type.
    pub fn with_coin_type(mut self, coin_type: u32) -> Self {
        self.coin_type = coin_type;
        self
    }

    /// Sets the account index.
    pub fn with_account_index(mut self, account_index: u32) -> Self {
        self.account_index = account_index;
        self
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(mut self, initial_address_index: u32) -> Self {
        self.initial_address_index = initial_address_index;
        self
    }

    /// Set a custom input(transaction output)
    pub fn with_input(mut self, input: UtxoInput) -> Result<Self> {
        self.inputs = match self.inputs {
            Some(mut inputs) => {
                inputs.push(input);
                // 128 is the maximum input amount
                if inputs.len() > INPUT_COUNT_MAX.into() {
                    return Err(Error::ConsolidationRequired(inputs.len()));
                }
                Some(inputs)
            }
            None => Some(vec![input]),
        };
        Ok(self)
    }

    /// Set a custom range in which to search for addresses for custom provided inputs. Default: 0..100
    pub fn with_input_range(mut self, range: Range<u32>) -> Self {
        self.input_range = range;
        self
    }

    /// Set a transfer to the builder
    pub fn with_output(mut self, address: &str, amount: u64) -> Result<Self> {
        let output = BasicOutputBuilder::new_with_amount(amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                Address::try_from_bech32(address)?.1,
            )))
            .finish()?;
        self.outputs.push(Output::Basic(output));
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(crate::Error::MessageError(bee_message::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(self.outputs.len()),
            )));
        }
        Ok(self)
    }

    /// Set outputs to the builder
    pub fn with_outputs(mut self, outputs: Vec<Output>) -> Result<Self> {
        self.outputs.extend(outputs);
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(crate::Error::MessageError(bee_message::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(self.outputs.len()),
            )));
        }
        Ok(self)
    }

    /// Set a transfer to the builder, address needs to be hex encoded
    pub fn with_output_hex(mut self, address: &str, amount: u64) -> Result<Self> {
        let output = BasicOutputBuilder::new_with_amount(amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                address.parse::<Ed25519Address>()?.into(),
            )))
            .finish()?;
        self.outputs.push(Output::Basic(output));
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(crate::Error::MessageError(bee_message::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(self.outputs.len()),
            )));
        }
        Ok(self)
    }

    /// Set a custom remainder address
    pub fn with_custom_remainder_address(mut self, address: &str) -> Result<Self> {
        let address = Address::try_from_bech32(address)?.1;
        self.custom_remainder_address.replace(address);
        Ok(self)
    }

    /// Set tagged_data to the builder
    pub fn with_tag<I: AsRef<[u8]>>(mut self, tag: I) -> Self {
        self.tag.replace(tag.as_ref().into());
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
            return Err(crate::Error::MessageError(bee_message::Error::InvalidParentCount(
                TryIntoBoundedU8Error::Truncated(parent_ids.len()),
            )));
        }
        self.parents.replace(parent_ids);
        Ok(self)
    }

    /// Set multiple options from client message builder options type
    /// Useful for bindings
    pub fn set_options(mut self, options: ClientMessageBuilderOptions) -> Result<Self> {
        if let Some(coin_type) = options.coin_type {
            self = self.with_coin_type(coin_type);
        }

        if let Some(account_index) = options.account_index {
            self = self.with_account_index(account_index);
        }

        if let Some(initial_address_index) = options.initial_address_index {
            self = self.with_initial_address_index(initial_address_index);
        }

        if let Some(inputs) = options.inputs {
            for input in inputs {
                self = self.with_input(UtxoInput::try_from(&input)?)?;
            }
        }

        if let Some(input_range) = options.input_range {
            self = self.with_input_range(input_range);
        }

        if let Some(output) = options.output {
            self = self.with_output(
                &output.address,
                output
                    .amount
                    .parse::<u64>()
                    .map_err(|_| Error::InvalidAmount(output.amount))?,
            )?;
        }

        if let Some(output_hex) = options.output_hex {
            self = self.with_output_hex(
                &output_hex.address,
                output_hex
                    .amount
                    .parse::<u64>()
                    .map_err(|_| Error::InvalidAmount(output_hex.amount))?,
            )?;
        }

        if let Some(outputs) = options.outputs {
            self = self.with_outputs(
                outputs
                    .iter()
                    .map(|o| Ok(Output::try_from(o)?))
                    .collect::<Result<Vec<Output>>>()?,
            )?;
        }

        if let Some(custom_remainder_address) = options.custom_remainder_address {
            self = self.with_custom_remainder_address(&custom_remainder_address)?;
        }

        if let Some(tag) = options.tag {
            self = self.with_tag(tag);
        }

        if let Some(data) = options.data {
            self = self.with_data(data);
        }

        if let Some(parents) = options.parents {
            self = self.with_parents(parents)?;
        }
        if let Some(allow_burning) = options.allow_burning {
            self = self.with_burning_allowed(allow_burning);
        }

        Ok(self)
    }

    /// Consume the builder and get the API result
    pub async fn finish(self) -> Result<Message> {
        // tagged_data payload requires an tagged_data tag
        if self.data.is_some() && self.tag.is_none() {
            return Err(Error::MissingParameter("tag"));
        }
        if self.inputs.is_some() && self.outputs.is_empty() {
            return Err(Error::MissingParameter("output"));
        }
        if !self.outputs.is_empty() {
            if self.secret_manager.is_none() && self.inputs.is_none() {
                return Err(Error::MissingParameter("Seed"));
            }
            // Send message with transaction
            let prepared_transaction_data = self.prepare_transaction().await?;
            let tx_payload = self.sign_transaction(prepared_transaction_data).await?;
            self.finish_message(Some(tx_payload)).await
        } else if self.tag.is_some() {
            // Send message with tagged_data payload
            self.finish_tagged_data().await
        } else {
            // Send message without payload
            self.finish_message(None).await
        }
    }

    /// Get output amount and address from an OutputDto, governance_transition for Alias Outputs so we get the unlock
    /// condition we're interested in
    pub fn get_output_amount_and_address(
        output: &OutputDto,
        governance_transition: Option<HashSet<AliasId>>,
    ) -> Result<(u64, Address)> {
        match output {
            OutputDto::Treasury(_) => Err(Error::OutputError("Treasury output is no supported")),
            OutputDto::Basic(ref r) => {
                for block in &r.unlock_conditions {
                    match block {
                        UnlockConditionDto::Address(e) => {
                            return Ok((
                                r.amount
                                    .parse::<u64>()
                                    .map_err(|_| crate::Error::InvalidAmount(r.amount.clone()))?,
                                Address::try_from(&e.address)?,
                            ));
                        }
                        _ => todo!(),
                    }
                }
                Err(Error::OutputError("Only Ed25519Address is implemented"))
            }
            OutputDto::Alias(ref r) => {
                let alias_id = AliasId::try_from(&r.alias_id)?;
                let mut is_governance_transition = false;
                if let Some(governance_transition) = governance_transition {
                    if governance_transition.contains(&alias_id) {
                        is_governance_transition = true;
                    }
                }
                for block in &r.unlock_conditions {
                    match block {
                        UnlockConditionDto::StateControllerAddress(e) => {
                            if is_governance_transition {
                                return Ok((
                                    r.amount
                                        .parse::<u64>()
                                        .map_err(|_| crate::Error::InvalidAmount(r.amount.clone()))?,
                                    Address::try_from(&e.address)?,
                                ));
                            }
                        }
                        UnlockConditionDto::GovernorAddress(e) => {
                            if !is_governance_transition {
                                return Ok((
                                    r.amount
                                        .parse::<u64>()
                                        .map_err(|_| crate::Error::InvalidAmount(r.amount.clone()))?,
                                    Address::try_from(&e.address)?,
                                ));
                            }
                        }
                        _ => todo!(),
                    }
                }
                Err(Error::OutputError("Only Ed25519Address is implemented"))
            }
            OutputDto::Foundry(ref r) => {
                for block in &r.unlock_conditions {
                    match block {
                        UnlockConditionDto::ImmutableAliasAddress(e) => {
                            return Ok((
                                r.amount
                                    .parse::<u64>()
                                    .map_err(|_| crate::Error::InvalidAmount(r.amount.clone()))?,
                                Address::try_from(&e.address)?,
                            ));
                        }
                        _ => todo!(),
                    }
                }
                Err(Error::OutputError("Only Ed25519Address is implemented"))
            }
            OutputDto::Nft(ref r) => {
                for block in &r.unlock_conditions {
                    match block {
                        UnlockConditionDto::Address(e) => {
                            return Ok((
                                r.amount
                                    .parse::<u64>()
                                    .map_err(|_| crate::Error::InvalidAmount(r.amount.clone()))?,
                                Address::try_from(&e.address)?,
                            ));
                        }
                        _ => todo!(),
                    }
                }
                Err(Error::OutputError("Only Ed25519Address is implemented"))
            }
        }
    }

    // If custom inputs are provided we check if they are unspent, get the balance and search the address for it,
    // governance_transition makes only a difference for alias outputs
    // Careful with setting `allow_burning` to `true`, native tokens can get easily burned by accident.
    async fn get_custom_inputs(
        &self,
        governance_transition: Option<HashSet<AliasId>>,
        byte_cost_config: &ByteCostConfig,
        allow_burning: bool,
    ) -> Result<SelectedTransactionData> {
        get_custom_inputs(self, governance_transition, byte_cost_config, allow_burning).await
    }

    // Searches inputs for an amount which a user wants to spend, also checks that it doesn't create dust
    async fn get_inputs(&self, byte_cost_config: &ByteCostConfig) -> Result<SelectedTransactionData> {
        get_inputs(self, byte_cost_config).await
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
    pub async fn finish_tagged_data(self) -> Result<Message> {
        let payload: Payload;
        {
            let index = &self.tag.as_ref();
            let empty_slice = &vec![];
            let data = &self.data.as_ref().unwrap_or(empty_slice);

            // build tagged_data
            let index = TaggedDataPayload::new(index.expect("No tagged_data tag").to_vec(), data.to_vec())
                .map_err(|e| Error::TaggedDataError(e.to_string()))?;
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
                let mut client_miner = ClientMinerBuilder::new().with_local_pow(self.client.get_local_pow().await);
                if let Some(worker_count) = self.client.pow_worker_count {
                    client_miner = client_miner.with_worker_count(worker_count);
                }
                do_pow(client_miner.finish(), min_pow_score, payload, parents)?
                    .1
                    .ok_or_else(|| Error::Pow("final message pow failed.".to_string()))?
            }
            None => finish_pow(self.client, payload).await?,
        };

        let msg_id = self.client.post_message(&final_message).await?;
        // Get message if we use remote PoW, because the node will change parents and nonce
        match self.client.get_local_pow().await {
            true => Ok(final_message),
            false => {
                // Request message multiple times because the node maybe didn't process it completely in this time
                // or a node balancer could be used which forwards the request to different node than we published
                for time in 1..3 {
                    if let Ok(message) = self.client.get_message_data(&msg_id).await {
                        return Ok(message);
                    }
                    #[cfg(not(feature = "wasm"))]
                    sleep(Duration::from_millis(time * 50)).await;
                    #[cfg(feature = "wasm")]
                    {
                        TimeoutFuture::new((time * 50).try_into().unwrap()).await;
                    }
                }
                self.client.get_message_data(&msg_id).await
            }
        }
    }
}
