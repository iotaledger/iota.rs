// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod input_selection;
pub mod pow;
pub mod transaction;

use std::ops::Range;

use iota_types::block::{
    address::{Address, Ed25519Address},
    input::{dto::UtxoInputDto, UtxoInput, INPUT_COUNT_MAX},
    output::{
        dto::OutputDto,
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, Output, OUTPUT_COUNT_RANGE,
    },
    parent::Parents,
    payload::{Payload, TaggedDataPayload},
    Block, BlockId,
};
use packable::bounded::TryIntoBoundedU16Error;

pub use self::transaction::verify_semantic;
use crate::{
    api::block_builder::input_selection::Burn, constants::SHIMMER_COIN_TYPE, secret::SecretManager, Client, Error,
    Result,
};

/// Builder of the block API
#[must_use]
pub struct ClientBlockBuilder<'a> {
    client: &'a Client,
    secret_manager: Option<&'a SecretManager>,
    coin_type: u32,
    account_index: u32,
    initial_address_index: u32,
    inputs: Option<Vec<UtxoInput>>,
    input_range: Range<u32>,
    outputs: Vec<Output>,
    custom_remainder_address: Option<Address>,
    tag: Option<Vec<u8>>,
    data: Option<Vec<u8>>,
    parents: Option<Parents>,
    burn: Option<Burn>,
}

/// Block output address
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientBlockBuilderOutputAddress {
    /// Address
    pub address: String,
    /// Amount
    // Using a String to prevent overflow issues in other languages
    pub amount: String,
}

/// Options for generating block
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientBlockBuilderOptions {
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
    pub output: Option<ClientBlockBuilderOutputAddress>,
    /// Hex encoded output address and amount
    pub output_hex: Option<ClientBlockBuilderOutputAddress>,
    /// Outputs
    pub outputs: Option<Vec<OutputDto>>,
    /// Custom remainder address
    pub custom_remainder_address: Option<String>,
    /// Hex encoded tag
    pub tag: Option<String>,
    /// Hex encoded data
    pub data: Option<String>,
    /// Parents
    pub parents: Option<Vec<BlockId>>,
    /// Explicit burning of aliases, nfts, foundries and native tokens
    pub burn: Option<Burn>,
}

impl<'a> ClientBlockBuilder<'a> {
    /// Create block builder
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
            burn: None,
        }
    }

    /// Sets explicit burning of aliases, nfts, foundries and native tokens.
    pub fn with_burn(mut self, burn: Burn) -> Self {
        self.burn.replace(burn);
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
    pub async fn with_output(mut self, address: &str, amount: u64) -> Result<ClientBlockBuilder<'a>> {
        let output = BasicOutputBuilder::new_with_amount(amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                Address::try_from_bech32(address)?.1,
            )))
            .finish_output(self.client.get_token_supply().await?)?;
        self.outputs.push(output);
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(crate::Error::Block(iota_types::block::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(self.outputs.len()),
            )));
        }
        Ok(self)
    }

    /// Set outputs to the builder
    pub fn with_outputs(mut self, outputs: Vec<Output>) -> Result<Self> {
        self.outputs.extend(outputs);
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(crate::Error::Block(iota_types::block::Error::InvalidOutputCount(
                TryIntoBoundedU16Error::Truncated(self.outputs.len()),
            )));
        }
        Ok(self)
    }

    /// Set a transfer to the builder, address needs to be hex encoded
    pub async fn with_output_hex(mut self, address: &str, amount: u64) -> Result<ClientBlockBuilder<'a>> {
        let output = BasicOutputBuilder::new_with_amount(amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                address.parse::<Ed25519Address>()?.into(),
            )))
            .finish_output(self.client.get_token_supply().await?)?;
        self.outputs.push(output);
        if !OUTPUT_COUNT_RANGE.contains(&(self.outputs.len() as u16)) {
            return Err(crate::Error::Block(iota_types::block::Error::InvalidOutputCount(
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
    pub fn with_tag(mut self, tag: Vec<u8>) -> Self {
        self.tag.replace(tag);
        self
    }

    /// Set data to the builder
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data.replace(data);
        self
    }

    /// Set 1-8 custom parent block ids
    pub fn with_parents(mut self, parent_ids: Vec<BlockId>) -> Result<Self> {
        self.parents.replace(Parents::new(parent_ids)?);
        Ok(self)
    }

    /// Set multiple options from client block builder options type
    /// Useful for bindings
    pub async fn set_options(mut self, options: ClientBlockBuilderOptions) -> Result<ClientBlockBuilder<'a>> {
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
            self = self
                .with_output(
                    &output.address,
                    output
                        .amount
                        .parse::<u64>()
                        .map_err(|_| Error::InvalidAmount(output.amount))?,
                )
                .await?;
        }

        if let Some(output_hex) = options.output_hex {
            self = self
                .with_output_hex(
                    &output_hex.address,
                    output_hex
                        .amount
                        .parse::<u64>()
                        .map_err(|_| Error::InvalidAmount(output_hex.amount))?,
                )
                .await?;
        }

        if let Some(outputs) = options.outputs {
            let token_supply = self.client.get_token_supply().await?;

            self = self.with_outputs(
                outputs
                    .iter()
                    .map(|o| Ok(Output::try_from_dto(o, token_supply)?))
                    .collect::<Result<Vec<Output>>>()?,
            )?;
        }

        if let Some(custom_remainder_address) = options.custom_remainder_address {
            self = self.with_custom_remainder_address(&custom_remainder_address)?;
        }

        if let Some(tag) = options.tag {
            self = self.with_tag(prefix_hex::decode(tag)?);
        }

        if let Some(data) = options.data {
            self = self.with_data(prefix_hex::decode(data)?);
        }

        if let Some(parents) = options.parents {
            self = self.with_parents(parents)?;
        }
        if let Some(burn) = options.burn {
            self = self.with_burn(burn);
        }

        Ok(self)
    }

    /// Consume the builder and get the API result
    pub async fn finish(self) -> Result<Block> {
        // tagged_data payload requires an tagged_data tag
        if self.data.is_some() && self.tag.is_none() {
            return Err(Error::MissingParameter("tag"));
        }
        if self.inputs.is_some() && self.outputs.is_empty() {
            return Err(Error::MissingParameter("output"));
        }
        if !self.outputs.is_empty() {
            if self.secret_manager.is_none() && self.inputs.is_none() {
                return Err(Error::MissingParameter("seed"));
            }
            // Send block with transaction
            let prepared_transaction_data = self.prepare_transaction().await?;
            let tx_payload = self.sign_transaction(prepared_transaction_data).await?;
            self.finish_block(Some(tx_payload)).await
        } else if self.tag.is_some() {
            // Send block with tagged_data payload
            self.finish_tagged_data().await
        } else {
            // Send block without payload
            self.finish_block(None).await
        }
    }

    /// Consume the builder and get the API result
    pub async fn finish_tagged_data(self) -> Result<Block> {
        let payload: Payload;
        {
            let index = &self.tag.as_ref();
            let empty_slice = &vec![];
            let data = &self.data.as_ref().unwrap_or(empty_slice);

            // build tagged_data
            let index = TaggedDataPayload::new(index.expect("no tagged_data tag").to_vec(), (*data).clone())
                .map_err(|e| Error::TaggedData(e.to_string()))?;
            payload = Payload::from(index);
        }

        // building block
        self.finish_block(Some(payload)).await
    }

    /// Builds the final block and posts it to the node
    pub async fn finish_block(self, payload: Option<Payload>) -> Result<Block> {
        // Do not replace parents with the latest tips if they are set explicitly,
        // necessary for block promotion.
        let final_block = self.client.finish_block_builder(self.parents, payload).await?;

        let block_id = self.client.post_block_raw(&final_block).await?;
        // Get block if we use remote PoW, because the node will change parents and nonce
        if self.client.get_local_pow() {
            Ok(final_block)
        } else {
            // Request block multiple times because the node maybe didn't process it completely in this time
            // or a node balancer could be used which forwards the request to different node than we published
            for time in 1..3 {
                if let Ok(block) = self.client.get_block(&block_id).await {
                    return Ok(block);
                }
                #[cfg(not(target_family = "wasm"))]
                tokio::time::sleep(std::time::Duration::from_millis(time * 50)).await;
                #[cfg(target_family = "wasm")]
                gloo_timers::future::TimeoutFuture::new((time * 50).try_into().unwrap()).await;
            }
            self.client.get_block(&block_id).await
        }
    }
}
