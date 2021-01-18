// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
mod types;
use iota::{
    builder::Network, Api, Bech32Address as RustBech32Address, BrokerOptions as RustBrokerOptions,
    Client as RustClient, ClientMiner as RustClientMiner, MessageBuilder as RustMessageBuilder,
    MessageId as RustMessageId, Seed as RustSeed, TransactionId as RustTransationId, UTXOInput as RustUTXOInput,
};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::str::FromStr;
use std::time::Duration;
use types::*;

/// An instance of the client using IRI URI.
#[pymethods]
impl Client {
    #[new]
    /// The constructor of the client instance.
    fn new(
        network: Option<&str>,
        node: Option<&str>,
        nodes: Option<Vec<&str>>,
        node_sync_interval: Option<u64>,
        node_sync_disabled: Option<bool>,
        node_pool_urls: Option<Vec<String>>,
        request_timeout: Option<u64>,
        api_timeout: Option<HashMap<&str, u64>>,
        local_pow: Option<bool>,
        mqtt_broker_options: Option<BrokerOptions>,
    ) -> Self {
        let mut client = RustClient::build();
        if let Some(network) = network {
            match network {
                "Mainnet" => client = client.with_network(Network::Mainnet),
                "Testnet" => client = client.with_network(Network::Testnet),
                _ => (),
            }
        }
        if let Some(node) = node {
            client = client.with_node(node).unwrap();
        }
        if let Some(nodes) = nodes {
            client = client.with_nodes(&nodes).unwrap();
        }
        if let Some(node_sync_interval) = node_sync_interval {
            client = client.with_node_sync_interval(Duration::from_millis(node_sync_interval));
        }
        if let Some(node_sync_disabled) = node_sync_disabled {
            if node_sync_disabled {
                client = client.with_node_sync_disabled();
            }
        }
        if let Some(node_pool_urls) = node_pool_urls {
            client = client.with_node_pool_urls(&node_pool_urls).unwrap();
        }
        if let Some(timeout) = request_timeout {
            client = client.with_request_timeout(Duration::from_millis(timeout));
        }
        if let Some(api_timeout) = api_timeout {
            for (api, timeout) in api_timeout {
                match api {
                    "GetHealth" => client = client.with_api_timeout(Api::GetHealth, Duration::from_millis(timeout)),
                    "GetInfo" => client = client.with_api_timeout(Api::GetInfo, Duration::from_millis(timeout)),
                    "GetTips" => client = client.with_api_timeout(Api::GetTips, Duration::from_millis(timeout)),
                    "PostMessage" => client = client.with_api_timeout(Api::PostMessage, Duration::from_millis(timeout)),
                    "GetOutput" => client = client.with_api_timeout(Api::GetOutput, Duration::from_millis(timeout)),
                    "GetMilestone" => {
                        client = client.with_api_timeout(Api::GetMilestone, Duration::from_millis(timeout))
                    }
                    _ => (),
                }
            }
        }
        if let Some(local_pow) = local_pow {
            client = client.with_local_pow(local_pow);
        }
        if let Some(broker_options) = mqtt_broker_options {
            let rust_broker_options = RustBrokerOptions::new()
                .automatic_disconnect(broker_options.automatic_disconnect)
                .timeout(Duration::from_millis(broker_options.timeout))
                .use_websockets(broker_options.use_ws);
            client = client.with_mqtt_broker_options(rust_broker_options);
        }
        Client {
            client: client.finish().unwrap(),
        }
    }
}

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
        MessageMetadata {
            message_id: message_metadata.message_id,
            parent1: message_metadata.parent1,
            parent2: message_metadata.parent2,
            is_solid: message_metadata.is_solid,
            should_promote: message_metadata.should_promote,
            should_reattach: message_metadata.should_reattach,
            referenced_by_milestone_index: message_metadata.referenced_by_milestone_index,
            ledger_inclusion_state: message_metadata.ledger_inclusion_state,
        }
    }
    /// Get the message data from the message_id.
    ///
    /// Parameters
    /// ----------
    /// message_id : &str
    ///     The identifier of message.
    ///
    /// Returns
    /// ----------
    /// message : Message
    ///     Return a Message object.
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

/// Full node API
#[pymethods]
impl Client {
    fn get_health(&self) -> bool {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.client.get_health().await.unwrap() })
    }
    fn get_info(&self) -> NodeInfo {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let node_info = rt.block_on(async { self.client.get_info().await.unwrap() });
        node_info.into()
    }
    fn get_tips(&self) -> (String, String) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let tips = rt.block_on(async { self.client.get_tips().await.unwrap() });
        (tips.0.to_string(), tips.1.to_string())
    }
    fn post_message(&self, msg: Message) -> String {
        let mut msg_builder = RustMessageBuilder::<RustClientMiner>::new()
            .with_network_id(msg.network_id)
            .with_parent1(RustMessageId::from_str(&msg.parent1).unwrap())
            .with_parent2(RustMessageId::from_str(&msg.parent1).unwrap())
            .with_nonce_provider(self.client.get_pow_provider(), 4000f64);
        if let Some(payload) = msg.payload {
            msg_builder = msg_builder.with_payload(payload.into());
        }
        let msg = msg_builder.finish().unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let message_id = rt.block_on(async { self.client.post_message(&msg).await.unwrap() });
        message_id.to_string()
    }
    fn get_output(&self, output_id: String) -> OutputMetadata {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let output_metadata = rt.block_on(async {
            self.client
                .get_output(&RustUTXOInput::from_str(&output_id).unwrap())
                .await
                .unwrap()
        });
        OutputMetadata {
            message_id: output_metadata.message_id,
            transaction_id: output_metadata.transaction_id,
            output_index: output_metadata.output_index,
            is_spent: output_metadata.is_spent,
            address: output_metadata.address.to_bech32(),
            amount: output_metadata.amount,
        }
    }
    fn get_address_balance(&self, address: &str) -> u64 {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            self.client
                .get_address()
                .balance(&RustBech32Address::from(address))
                .await
                .unwrap()
        })
    }
    fn get_address_outputs(&self, address: &str) -> Vec<UTXOInput> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let outputs = rt.block_on(async {
            self.client
                .get_address()
                .outputs(&RustBech32Address::from(address))
                .await
                .unwrap()
        });
        (*outputs)
            .to_vec()
            .iter()
            .map(|output| UTXOInput {
                transaction_id: output.output_id().transaction_id().as_ref().to_vec(),
                index: output.output_id().index(),
            })
            .collect()
    }
    fn find_outputs(&self, output_ids: Option<Vec<String>>, addresses: Option<Vec<String>>) -> Vec<OutputMetadata> {
        let output_ids: Vec<RustUTXOInput> = output_ids
            .unwrap_or(vec![])
            .iter()
            .map(|input| RustUTXOInput::from_str(input).unwrap_or_else(|_| panic!("invalid input: {}", input)))
            .collect();
        let addresses: Vec<RustBech32Address> = addresses
            .unwrap_or(vec![])
            .iter()
            .map(|address| RustBech32Address::from(&address[..]))
            .collect();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let output_metadata_vec =
            rt.block_on(async { self.client.find_outputs(&output_ids[..], &addresses[..]).await.unwrap() });
        output_metadata_vec
            .iter()
            .map(|metadata| OutputMetadata {
                message_id: metadata.message_id.clone(),
                transaction_id: metadata.transaction_id.clone(),
                output_index: metadata.output_index,
                is_spent: metadata.is_spent,
                address: metadata.address.to_bech32(),
                amount: metadata.amount,
            })
            .collect()
    }
    fn get_milestone(&self, index: u64) -> MilestoneMetadata {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let milestone_metadata = rt.block_on(async { self.client.get_milestone(index).await.unwrap() });
        MilestoneMetadata {
            milestone_index: milestone_metadata.milestone_index,
            message_id: milestone_metadata.message_id,
            timestamp: milestone_metadata.timestamp,
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn iota_client(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    Ok(())
}
