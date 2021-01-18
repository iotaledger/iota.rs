// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::{AddressBalancePair, Client, Input, Message, MessageMetadata, Output};
use iota::{
    Bech32Address as RustBech32Address, MessageId as RustMessageId, Seed as RustSeed,
    TransactionId as RustTransationId, UTXOInput as RustUTXOInput,
};
use pyo3::prelude::*;

use std::convert::{From, Into};
use std::str::FromStr;

// use types::*;

/// General high level APIs
#[pymethods]
impl Client {
    fn send(
        &self,
        seed: Option<String>,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
        inputs: Option<Vec<Input>>,
        outputs: Option<Vec<Output>>,
        index: Option<&str>,
        data: Option<Vec<u8>>,
        parent: Option<&str>,
        network_id: Option<u64>,
    ) -> String {
        let mut send_builder = self.client.send();
        if let Some(account_index) = account_index {
            send_builder = send_builder.with_account_index(account_index);
        }
        if let Some(initial_address_index) = initial_address_index {
            send_builder = send_builder.with_initial_address_index(initial_address_index);
        }
        if let Some(inputs) = inputs {
            for input in inputs {
                send_builder = send_builder.with_input(
                    RustUTXOInput::new(
                        RustTransationId::from_str(&input.transaction_id[..]).unwrap(),
                        input.index,
                    )
                    .unwrap(),
                );
            }
        }
        if let Some(outputs) = outputs {
            for output in outputs {
                send_builder = send_builder
                    .with_output(&output.address[..].into(), output.amount)
                    .unwrap();
            }
        }
        if let Some(index) = index {
            send_builder = send_builder.with_index(index);
        }
        if let Some(data) = data {
            send_builder = send_builder.with_data(data);
        }
        if let Some(parent) = parent {
            send_builder = send_builder.with_parent(RustMessageId::from_str(parent).unwrap());
        }
        if let Some(network_id) = network_id {
            send_builder = send_builder.with_network_id(network_id);
        }
        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Some(seed) = seed {
            let seed = RustSeed::from_ed25519_bytes(&hex::decode(&seed[..]).unwrap()).unwrap();
            return rt
                .block_on(async { send_builder.with_seed(&seed).finish().await.unwrap() })
                .to_string();
        } else {
            return rt.block_on(async { send_builder.finish().await.unwrap() }).to_string();
        }
    }
    fn get_message_metadata(&self, message_id: &str) -> MessageMetadata {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let message_metadata = rt.block_on(async {
            self.client
                .get_message()
                .metadata(&RustMessageId::from_str(message_id).unwrap())
                .await
                .unwrap()
        });
        message_metadata.into()
    }
    /// Get the message data from the message_id.
    ///
    /// Args:
    ///     message_id (str): The identifier of message.
    ///
    /// Returns:
    ///     message (Message): The returned message object.
    ///     Return a Message object.
    ///
    fn get_message_data(&self, message_id: &str) -> Message {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let message = rt.block_on(async {
            self.client
                .get_message()
                .data(&RustMessageId::from_str(message_id).unwrap())
                .await
                .unwrap()
        });
        message.into()
    }
    fn get_message_raw(&self, message_id: &str) -> String {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let raw_data = rt.block_on(async {
            self.client
                .get_message()
                .raw(&RustMessageId::from_str(message_id).unwrap())
                .await
                .unwrap()
        });
        raw_data
    }
    fn get_message_children(&self, message_id: &str) -> Vec<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let children = rt.block_on(async {
            self.client
                .get_message()
                .children(&RustMessageId::from_str(message_id).unwrap())
                .await
                .unwrap()
        });
        children
            .into_iter()
            .map(|child| String::from_utf8(child.as_ref().to_vec()).unwrap())
            .collect()
    }
    fn get_message_index(&self, index: &str) -> Vec<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let indices = rt.block_on(async { self.client.get_message().index(index).await.unwrap() });
        indices
            .into_iter()
            .map(|index| String::from_utf8(index.as_ref().to_vec()).unwrap())
            .collect()
    }
    fn find_messages(&self, indexation_keys: Option<Vec<String>>, message_ids: Option<Vec<String>>) -> Vec<Message> {
        let message_ids: Vec<RustMessageId> = message_ids
            .unwrap_or(vec![])
            .iter()
            .map(|id| RustMessageId::from_str(&id[..]).unwrap())
            .collect();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let messages = rt.block_on(async {
            self.client
                .find_messages(&indexation_keys.unwrap_or(vec![])[..], &message_ids[..])
                .await
                .unwrap()
        });
        messages.into_iter().map(|message| message.into()).collect()
    }
    fn get_unspent_address(
        &self,
        seed: String,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
    ) -> (String, usize) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let address_index = rt.block_on(async {
            let seed = RustSeed::from_ed25519_bytes(&hex::decode(&seed[..]).unwrap()).unwrap();
            self.client
                .get_unspent_address(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_initial_address_index(initial_address_index.unwrap_or(0))
                .get()
                .await
                .unwrap()
        });
        (address_index.0 .0, address_index.1)
    }
    fn find_addresses(
        &self,
        seed: String,
        account_index: Option<usize>,
        begin: Option<usize>,
        end: Option<usize>,
        get_all: Option<bool>,
    ) -> Vec<(String, Option<bool>)> {
        let seed = RustSeed::from_ed25519_bytes(&hex::decode(&seed[..]).unwrap()).unwrap();
        if get_all.unwrap_or(false) {
            let addresses = self
                .client
                .find_addresses(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_range(begin.unwrap_or(0)..end.unwrap_or(20))
                .get_all()
                .unwrap();
            return addresses
                .iter()
                .map(|address_changed| (address_changed.0.to_string(), Some(address_changed.1)))
                .collect();
        } else {
            let addresses = self
                .client
                .find_addresses(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_range(begin.unwrap_or(0)..end.unwrap_or(20))
                .finish()
                .unwrap();
            return addresses
                .iter()
                .map(|addresses| (addresses.to_string(), None))
                .collect();
        }
    }
    fn get_balance(&self, seed: String, account_index: Option<usize>, initial_address_index: Option<usize>) -> u64 {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let seed = RustSeed::from_ed25519_bytes(&hex::decode(&seed[..]).unwrap()).unwrap();
        rt.block_on(async {
            self.client
                .get_balance(&seed)
                .with_account_index(account_index.unwrap_or(0))
                .with_initial_address_index(initial_address_index.unwrap_or(0))
                .finish()
                .await
                .unwrap()
        })
    }
    fn get_address_balances(&self, addresses: Vec<String>) -> Vec<AddressBalancePair> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let bench32_addresses: Vec<RustBech32Address> = addresses
            .iter()
            .map(|address| RustBech32Address::from(&address[..]))
            .collect();

        let address_balances =
            rt.block_on(async { self.client.get_address_balances(&bench32_addresses[..]).await.unwrap() });

        address_balances
            .iter()
            .map(|address_balance| AddressBalancePair {
                address: address_balance.address.0.clone(),
                balance: address_balance.balance,
            })
            .collect()
    }
    // TODO MQTT subscriber
    fn subscriber(&self) {}
    fn retry(&self, message_id: String) -> (String, Message) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let message_id_message = rt.block_on(async {
            self.client
                .retry(&RustMessageId::from_str(&message_id).unwrap())
                .await
                .unwrap()
        });
        (message_id_message.0.to_string(), message_id_message.1.into())
    }
    fn reattach(&self, message_id: String) -> (String, Message) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let message_id_message = rt.block_on(async {
            self.client
                .reattach(&RustMessageId::from_str(&message_id).unwrap())
                .await
                .unwrap()
        });
        (message_id_message.0.to_string(), message_id_message.1.into())
    }
    fn promote(&self, message_id: String) -> (String, Message) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let message_id_message = rt.block_on(async {
            self.client
                .promote(&RustMessageId::from_str(&message_id).unwrap())
                .await
                .unwrap()
        });
        (message_id_message.0.to_string(), message_id_message.1.into())
    }
}
