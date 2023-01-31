// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::{
    error::{Error, Result},
    AddressBalancePair, AddressDto, Client, Message, MessageMetadataResponse, Output, OutputDto, Payload,
    PreparedTransactionData, UtxoInput,
};
use iota_client::{
    api::{
        search_address, ClientMessageBuilder as RustClientMessageBuilder,
        PreparedTransactionData as RustPreparedTransactionData,
    },
    bee_message::prelude::{
        Address as RustAddress, Message as RustMessage, MessageId as RustMessageId, Payload as RustPayload,
        TransactionId as RustTransactionId, UtxoInput as RustUtxoInput,
    },
    bee_rest_api::types::dtos::MessageDto as BeeMessageDto,
    Client as RustClient, Seed as RustSeed,
};
use pyo3::{exceptions, prelude::*};
use std::{
    convert::{Into, TryFrom, TryInto},
    str::FromStr,
};

/// General high level APIs
#[pymethods]
impl Client {
    #[allow(clippy::too_many_arguments)]
    fn message(
        &self,
        seed: Option<String>,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
        inputs: Option<Vec<UtxoInput>>,
        input_range_begin: Option<usize>,
        input_range_end: Option<usize>,
        outputs: Option<Vec<Output>>,
        dust_allowance_outputs: Option<Vec<Output>>,
        index: Option<&str>,
        index_raw: Option<&[u8]>,
        data: Option<Vec<u8>>,
        data_str: Option<String>,
        parents: Option<Vec<&str>>,
    ) -> Result<Message> {
        if input_range_begin.is_some() ^ input_range_end.is_some() {
            return Err(Error {
                error: PyErr::new::<exceptions::PyValueError, _>(
                    "input_range_begin and input_range_end need to be assigned together!",
                ),
            });
        }
        let mut send_builder = self.client.message();
        if let Some(account_index) = account_index {
            send_builder = send_builder.with_account_index(account_index);
        }
        if let Some(initial_address_index) = initial_address_index {
            send_builder = send_builder.with_initial_address_index(initial_address_index);
        }
        if let Some(inputs) = inputs {
            for input in inputs {
                send_builder = send_builder.with_input(input.try_into()?);
            }
        }

        if let (Some(input_range_begin), Some(input_range_end)) = (input_range_begin, input_range_end) {
            send_builder = send_builder.with_input_range(input_range_begin..input_range_end);
        }

        if let Some(outputs) = outputs {
            for output in outputs {
                send_builder = send_builder.with_output(&output.address[..], output.amount)?;
            }
        }
        if let Some(dust_allowance_outputs) = dust_allowance_outputs {
            for output in dust_allowance_outputs {
                send_builder = send_builder.with_dust_allowance_output(&output.address[..], output.amount)?;
            }
        }
        if let Some(index) = index {
            send_builder = send_builder.with_index(index.as_bytes());
        }
        if let Some(index_raw) = index_raw {
            send_builder = send_builder.with_index(index_raw);
        }
        if let Some(data) = data {
            send_builder = send_builder.with_data(data);
        }
        if let Some(data_str) = data_str {
            send_builder = send_builder.with_data(data_str.as_bytes().to_vec());
        }
        if let Some(parents) = parents {
            let mut parent_ids = Vec::new();
            for parent in parents {
                parent_ids.push(RustMessageId::from_str(parent)?);
            }
            send_builder = send_builder.with_parents(parent_ids)?;
        }
        if let Some(seed) = seed {
            let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
            crate::block_on(async { send_builder.with_seed(&seed).finish().await })?.try_into()
        } else {
            crate::block_on(async { send_builder.finish().await })?.try_into()
        }
    }
    fn get_output_amount_and_address(&self, output: OutputDto) -> Result<(u64, AddressDto, bool)> {
        let (output_amount, address, single) = RustClientMessageBuilder::get_output_amount_and_address(&output.into())?;
        Ok((output_amount, address.into(), single))
    }
    fn prepare_transaction(&self, inputs: Vec<UtxoInput>, outputs: Vec<Output>) -> Result<PreparedTransactionData> {
        let mut prepare_transaction_builder = self.client.message();
        for input in inputs {
            prepare_transaction_builder = prepare_transaction_builder.with_input(RustUtxoInput::new(
                RustTransactionId::new(input.transaction_id.try_into().unwrap()),
                input.index,
            )?);
        }
        for output in outputs {
            prepare_transaction_builder =
                prepare_transaction_builder.with_output(&output.address[..], output.amount)?;
        }
        crate::block_on(async { prepare_transaction_builder.prepare_transaction().await })?.try_into()
    }
    fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
        seed: String,
        start_index: usize,
        end_index: usize,
    ) -> Result<Payload> {
        let sign_transaction_builder = self.client.message();
        let data: RustPreparedTransactionData = prepared_transaction_data.try_into()?;
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        crate::block_on(async {
            sign_transaction_builder
                .sign_transaction(data, Some(&seed), Some(start_index..end_index))
                .await
        })?
        .try_into()
    }
    fn finish_message(&self, payload: Payload) -> Result<Message> {
        let payload: RustPayload = payload.try_into()?;
        crate::block_on(async { self.client.message().finish_message(Some(payload)).await })?.try_into()
    }
    /// Get the message data from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     message_metadata (dict): The returned MessageMetadataResponse dict.
    fn get_message_metadata(&self, message_id: &str) -> Result<MessageMetadataResponse> {
        Ok(crate::block_on(async {
            self.client
                .get_message()
                .metadata(&RustMessageId::from_str(message_id)?)
                .await
        })?
        .into())
    }
    /// Get the message data from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     message (dict): The returned message dict.
    fn get_message_data(&self, message_id: &str) -> Result<Message> {
        crate::block_on(async {
            self.client
                .get_message()
                .data(&RustMessageId::from_str(message_id)?)
                .await
        })?
        .try_into()
    }
    /// Get the message raw string from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     raw (str): The returned message string.
    fn get_message_raw(&self, message_id: &str) -> Result<String> {
        Ok(crate::block_on(async {
            self.client
                .get_message()
                .raw(&RustMessageId::from_str(message_id)?)
                .await
        })?)
    }
    /// Get the message children from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     children ([str]): The returned list of children string.
    fn get_message_children(&self, message_id: &str) -> Result<Vec<String>> {
        let children = crate::block_on(async {
            self.client
                .get_message()
                .children(&RustMessageId::from_str(message_id)?)
                .await
        })?;
        Ok(children.iter().map(|child| hex::encode(child.as_ref())).collect())
    }
    /// Get the message id from the payload string.
    ///
    /// Args:
    ///     payload_str (str): The message payload string.
    ///
    /// Returns:
    ///     message_id (str): The identifier of message.
    fn get_message_id(&self, payload_str: &str) -> Result<String> {
        // Try BeeMessageDto and if it fails Message
        let message = match serde_json::from_str::<BeeMessageDto>(payload_str) {
            Ok(message_dto) => RustMessage::try_from(&message_dto).expect("invalid message"),
            Err(_) => serde_json::from_str::<RustMessage>(payload_str).expect("invalid message"),
        };
        let message_id = message.id().0.to_string();
        Ok(message_id)
    }
    /// Get the transaction id from a transaction payload.
    ///
    /// Args:
    ///     payload (str): The transaction payload.
    ///
    /// Returns:
    ///     transaction_id (str): The identifier of a transaction.
    fn get_transaction_id(&self, payload: Payload) -> Result<String> {
        let payload: RustPayload = payload.try_into()?;
        let transaction = match payload {
            RustPayload::Transaction(tx) => *tx,
            _ => panic!("no transaction payload"),
        };
        let transaction_id = transaction.id().to_string();
        Ok(transaction_id)
    }
    /// Get the list of message indices from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     message_indices ([str]): The returned list of message indices.
    fn get_message_index(&self, index: &str) -> Result<Vec<String>> {
        let indices = crate::block_on(async { self.client.get_message().index(index).await })?;
        Ok(indices.iter().map(|index| hex::encode(index.as_ref())).collect())
    }
    /// Find all messages by provided message IDs.
    ///
    /// Args:
    ///     indexation_keys ([str]): The identifier of message.
    ///     message_ids ([str]): The identifier of message.
    ///
    /// Returns:
    ///     messages ([str]): The returned list of message dict.
    fn find_messages(
        &self,
        indexation_keys: Option<Vec<String>>,
        message_ids: Option<Vec<String>>,
    ) -> Result<Vec<Message>> {
        let message_ids: Vec<RustMessageId> = message_ids
            .unwrap_or_default()
            .iter()
            .map(|id| RustMessageId::from_str(&id[..]).unwrap())
            .collect();
        let messages = crate::block_on(async {
            self.client
                .find_messages(&indexation_keys.unwrap_or_default()[..], &message_ids[..])
                .await
        })?;
        messages.into_iter().map(|message| message.try_into()).collect()
    }
    fn get_unspent_address(
        &self,
        seed: String,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
    ) -> Result<(String, usize)> {
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        let address_index = crate::block_on(async {
            self.client
                .get_unspent_address(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_initial_address_index(initial_address_index.unwrap_or(0))
                .get()
                .await
        })?;
        Ok((address_index.0, address_index.1))
    }
    fn get_addresses(
        &self,
        seed: String,
        account_index: Option<usize>,
        input_range_begin: Option<usize>,
        input_range_end: Option<usize>,
        bech32_hrp: Option<String>,
        get_all: Option<bool>,
    ) -> Result<Vec<(String, Option<bool>)>> {
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        if input_range_begin.is_some() ^ input_range_end.is_some() {
            return Err(Error {
                error: PyErr::new::<exceptions::PyValueError, _>(
                    "input_range_begin and input_range_end need to be assigned together!",
                ),
            });
        }
        let begin: usize = input_range_begin.unwrap_or(0);
        let end: usize = input_range_end.unwrap_or(0);
        if get_all.unwrap_or(false) {
            let addresses = crate::block_on(async {
                if let Some(bech32_hrp) = bech32_hrp {
                    self.client
                        .get_addresses(&seed)
                        .with_account_index(account_index.unwrap_or(0))
                        .with_range(begin..end)
                        .with_bech32_hrp(bech32_hrp)
                        .get_all()
                        .await
                } else {
                    self.client
                        .get_addresses(&seed)
                        .with_account_index(account_index.unwrap_or(0))
                        .with_range(begin..end)
                        .get_all()
                        .await
                }
            })?;
            Ok(addresses
                .iter()
                .map(|address_changed| (address_changed.0.to_string(), Some(address_changed.1)))
                .collect())
        } else {
            let addresses = crate::block_on(async {
                if let Some(bech32_hrp) = bech32_hrp {
                    self.client
                        .get_addresses(&seed)
                        .with_account_index(account_index.unwrap_or(0))
                        .with_range(begin..end)
                        .with_bech32_hrp(bech32_hrp)
                        .finish()
                        .await
                } else {
                    self.client
                        .get_addresses(&seed)
                        .with_account_index(account_index.unwrap_or(0))
                        .with_range(begin..end)
                        .finish()
                        .await
                }
            })?;
            Ok(addresses
                .iter()
                .map(|addresses| (addresses.to_string(), None))
                .collect())
        }
    }
    fn get_balance(
        &self,
        seed: String,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
        gap_limit: Option<usize>,
    ) -> Result<u64> {
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        let balance = crate::block_on(async {
            self.client
                .get_balance(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_initial_address_index(initial_address_index.unwrap_or(0))
                .with_gap_limit(gap_limit.unwrap_or(20))
                .finish()
                .await
        })?;
        Ok(balance)
    }
    fn get_address_balances(&self, addresses: Vec<String>) -> Result<Vec<AddressBalancePair>> {
        let address_balances = crate::block_on(async { self.client.get_address_balances(&addresses[..]).await })?;

        Ok(address_balances
            .iter()
            .map(|address_balance| AddressBalancePair {
                address: crate::block_on(async { self.client.hex_to_bech32(&address_balance.address, None).await })
                    .unwrap_or_else(|_| panic!("invalid bech32 address: {}", address_balance.address)),
                balance: address_balance.balance,
                dust_allowed: address_balance.dust_allowed,
            })
            .collect())
    }
    fn generate_mnemonic(&self) -> Result<String> {
        Ok(RustClient::generate_mnemonic()?)
    }
    fn mnemonic_to_hex_seed(&self, mnemonic: &str) -> Result<String> {
        Ok(RustClient::mnemonic_to_hex_seed(mnemonic)?)
    }
    fn find_inputs(&self, addresses: Vec<String>, amount: u64) -> Result<Vec<UtxoInput>> {
        let inputs = crate::block_on(async { self.client.find_inputs(addresses, amount).await })?;
        Ok((*inputs)
            .to_vec()
            .iter()
            .map(|input| UtxoInput {
                transaction_id: input.output_id().transaction_id().as_ref().to_vec(),
                index: input.output_id().index(),
            })
            .collect())
    }
    fn bech32_to_hex(&self, hex: &str) -> Result<String> {
        Ok(RustClient::bech32_to_hex(hex)?)
    }
    fn hex_to_bech32(&self, hex: &str, bech32_hrp: Option<&str>) -> Result<String> {
        Ok(crate::block_on(async {
            self.client.hex_to_bech32(hex, bech32_hrp).await
        })?)
    }
    fn hex_public_key_to_bech32_address(&self, hex: &str, bech32_hrp: Option<&str>) -> Result<String> {
        Ok(crate::block_on(async {
            self.client.hex_public_key_to_bech32_address(hex, bech32_hrp).await
        })?)
    }
    fn is_address_valid(&self, address: &str) -> bool {
        RustClient::is_address_valid(address)
    }
    fn retry(&self, message_id: String) -> Result<(String, Message)> {
        let message_id_message =
            crate::block_on(async { self.client.retry(&RustMessageId::from_str(&message_id)?).await })?;
        Ok((message_id_message.0.to_string(), message_id_message.1.try_into()?))
    }
    fn retry_until_included(
        &self,
        message_id: String,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> Result<Vec<(String, Message)>> {
        let messages = crate::block_on(async {
            self.client
                .retry_until_included(&RustMessageId::from_str(&message_id)?, interval, max_attempts)
                .await
        })?;
        let mut res = Vec::new();
        for msg in messages {
            res.push((msg.0.to_string(), msg.1.try_into()?));
        }
        Ok(res)
    }
    fn consolidate_funds(
        &self,
        seed: String,
        account_index: usize,
        start_index: usize,
        end_index: usize,
    ) -> Result<String> {
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        let address = crate::block_on(async {
            self.client
                .consolidate_funds(&seed, account_index, start_index..end_index)
                .await
        })?;
        Ok(address)
    }
    fn search_address(
        &self,
        seed: String,
        bech32_hrp: String,
        account_index: usize,
        start_index: usize,
        end_index: usize,
        address: String,
    ) -> Result<(usize, bool)> {
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        let address =
            RustAddress::try_from_bech32(&address).unwrap_or_else(|_| panic!("invalid Input Address: {address}"));
        let result = crate::block_on(async {
            search_address(&seed, &bech32_hrp, account_index, start_index..end_index, &address).await
        })?;
        Ok(result)
    }
    fn reattach(&self, message_id: String) -> Result<(String, Message)> {
        let message_id_message =
            crate::block_on(async { self.client.reattach(&RustMessageId::from_str(&message_id)?).await })?;
        Ok((message_id_message.0.to_string(), message_id_message.1.try_into()?))
    }
    fn promote(&self, message_id: String) -> Result<(String, Message)> {
        let message_id_message =
            crate::block_on(async { self.client.promote(&RustMessageId::from_str(&message_id)?).await })?;
        Ok((message_id_message.0.to_string(), message_id_message.1.try_into()?))
    }
}
