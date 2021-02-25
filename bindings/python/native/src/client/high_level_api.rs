// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::{
    error::{Error, Result},
    AddressBalancePair, Client, Input, Message, MessageMetadataResponse, Output,
};
use iota::{
    Bech32Address as RustBech32Address, MessageId as RustMessageId, Seed as RustSeed,
    TransactionId as RustTransationId, UTXOInput as RustUTXOInput,
};
use pyo3::{exceptions, prelude::*};
use std::{
    convert::{From, Into, TryInto},
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
        inputs: Option<Vec<Input>>,
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
                send_builder = send_builder.with_input(RustUTXOInput::new(
                    RustTransationId::from_str(&input.transaction_id[..])?,
                    input.index,
                )?);
            }
        }

        if let (Some(input_range_begin), Some(input_range_end)) = (input_range_begin, input_range_end) {
            send_builder = send_builder.with_input_range(input_range_begin..input_range_end);
        }

        if let Some(outputs) = outputs {
            for output in outputs {
                send_builder = send_builder.with_output(&output.address[..].into(), output.amount)?;
            }
        }
        if let Some(dust_allowance_outputs) = dust_allowance_outputs {
            for output in dust_allowance_outputs {
                send_builder = send_builder.with_dust_allowance_output(&output.address[..].into(), output.amount)?;
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
        let rt = tokio::runtime::Runtime::new()?;
        if let Some(seed) = seed {
            let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
            rt.block_on(async { send_builder.with_seed(&seed).finish().await })?
                .try_into()
        } else {
            rt.block_on(async { send_builder.finish().await })?.try_into()
        }
    }
    /// Get the message data from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     message_metadata (dict): The returned MessageMetadataResponse dict.
    fn get_message_metadata(&self, message_id: &str) -> Result<MessageMetadataResponse> {
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt
            .block_on(async {
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
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
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
        let rt = tokio::runtime::Runtime::new()?;
        Ok(rt.block_on(async {
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
        let rt = tokio::runtime::Runtime::new()?;
        let children = rt.block_on(async {
            self.client
                .get_message()
                .children(&RustMessageId::from_str(message_id)?)
                .await
        })?;
        Ok(children.iter().map(|child| hex::encode(child.as_ref())).collect())
    }
    /// Get the list of message indices from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     message_indices ([str]): The returned list of message indices.
    fn get_message_index(&self, index: &str) -> Result<Vec<String>> {
        let rt = tokio::runtime::Runtime::new()?;
        let indices = rt.block_on(async { self.client.get_message().index(index).await })?;
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
        let rt = tokio::runtime::Runtime::new()?;
        let messages = rt.block_on(async {
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
        let rt = tokio::runtime::Runtime::new()?;
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        let address_index = rt.block_on(async {
            self.client
                .get_unspent_address(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_initial_address_index(initial_address_index.unwrap_or(0))
                .get()
                .await
        })?;
        Ok((address_index.0 .0, address_index.1))
    }
    fn get_addresses(
        &self,
        seed: String,
        account_index: Option<usize>,
        input_range_begin: Option<usize>,
        input_range_end: Option<usize>,
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
            let rt = tokio::runtime::Runtime::new()?;
            let addresses = rt.block_on(async {
                self.client
                    .get_addresses(&seed)
                    .with_account_index(account_index.unwrap_or(0))
                    .with_range(begin..end)
                    .get_all()
                    .await
            })?;
            Ok(addresses
                .iter()
                .map(|address_changed| (address_changed.0.to_string(), Some(address_changed.1)))
                .collect())
        } else {
            let rt = tokio::runtime::Runtime::new()?;
            let addresses = rt.block_on(async {
                self.client
                    .get_addresses(&seed)
                    .with_account_index(account_index.unwrap_or(0))
                    .with_range(begin..end)
                    .finish()
                    .await
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
    ) -> Result<u64> {
        let rt = tokio::runtime::Runtime::new()?;
        let seed = RustSeed::from_bytes(&hex::decode(&seed[..])?);
        let balance = rt.block_on(async {
            self.client
                .get_balance(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_initial_address_index(initial_address_index.unwrap_or(0))
                .finish()
                .await
        })?;
        Ok(balance)
    }
    fn get_address_balances(&self, addresses: Vec<String>) -> Result<Vec<AddressBalancePair>> {
        let rt = tokio::runtime::Runtime::new()?;
        let bench32_addresses: Vec<RustBech32Address> = addresses
            .iter()
            .map(|address| RustBech32Address::from(&address[..]))
            .collect();

        let address_balances = rt.block_on(async { self.client.get_address_balances(&bench32_addresses[..]).await })?;

        Ok(address_balances
            .iter()
            .map(|address_balance| AddressBalancePair {
                address: address_balance.address.clone(),
                balance: address_balance.balance,
            })
            .collect())
    }
    fn retry(&self, message_id: String) -> Result<(String, Message)> {
        let rt = tokio::runtime::Runtime::new()?;
        let message_id_message =
            rt.block_on(async { self.client.retry(&RustMessageId::from_str(&message_id)?).await })?;
        Ok((message_id_message.0.to_string(), message_id_message.1.try_into()?))
    }
    fn reattach(&self, message_id: String) -> Result<(String, Message)> {
        let rt = tokio::runtime::Runtime::new()?;
        let message_id_message =
            rt.block_on(async { self.client.reattach(&RustMessageId::from_str(&message_id)?).await })?;
        Ok((message_id_message.0.to_string(), message_id_message.1.try_into()?))
    }
    fn promote(&self, message_id: String) -> Result<(String, Message)> {
        let rt = tokio::runtime::Runtime::new()?;
        let message_id_message =
            rt.block_on(async { self.client.promote(&RustMessageId::from_str(&message_id)?).await })?;
        Ok((message_id_message.0.to_string(), message_id_message.1.try_into()?))
    }
}
