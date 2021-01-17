// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{
    builder::Network, Address as RustAddress, Api, Bech32Address as RustBech32Address,
    BrokerOptions as RustBrokerOptions, Client as RustClient, ClientMiner, Ed25519Address as RustEd25519Address,
    Ed25519Signature as RustEd25519Signature, IndexationPayload as RustIndexationPayload, Input as RustInput,
    Message as RustMessage, MessageBuilder as RustMessageBuilder, MessageId as RustMessageId, Output as RustOutput,
    Payload as RustPayload, ReferenceUnlock as RustReferenceUnlock, Seed as RustSeed,
    SignatureLockedSingleOutput as RustSignatureLockedSingleOutput, SignatureUnlock as RustSignatureUnlock,
    TransactionId as RustTransationId, TransactionPayload as RustTransactionPayload,
    TransactionPayloadEssence as RustTransactionPayloadEssence, UTXOInput as RustUTXOInput,
    UnlockBlock as RustUnlockBlock,
};

// use pyo3::conversion::IntoPy;
use dict_derive::{FromPyObject as DeriveFromPyObject, IntoPyObject as DeriveIntoPyObject};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::convert::{From, Into, TryInto};
use std::str::FromStr;
use std::time::Duration;
pub const MILESTONE_MERKLE_PROOF_LENGTH: usize = 32;
pub const MILESTONE_PUBLIC_KEY_LENGTH: usize = 32;
pub const MILESTONE_SIGNATURE_LENGTH: usize = 64;
#[pyclass]
struct Client {
    client: RustClient,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MessageMetadata {
    /// Message ID
    pub message_id: String,
    /// Message ID of parent1
    pub parent1: String,
    /// Message ID of parent2
    pub parent2: String,
    /// Solid status
    pub is_solid: bool,
    /// Should promote
    pub should_promote: Option<bool>,
    /// Should reattach
    pub should_reattach: Option<bool>,
    /// Referenced by milestone index
    pub referenced_by_milestone_index: Option<u64>,
    /// Ledger inclusion state
    pub ledger_inclusion_state: Option<String>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct AddressBalancePair {
    /// Address
    pub address: String,
    /// Balance in the address
    pub balance: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestoneMetadata {
    /// Milestone index
    pub index: u64,
    /// Milestone ID
    pub message_id: String,
    /// Timestamp
    pub timestamp: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct UTXOInput {
    transaction_id: Vec<u8>,
    index: u16,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct OutputMetadata {
    /// Message ID of the output
    pub message_id: Vec<u8>,
    /// Transaction ID of the output
    pub transaction_id: Vec<u8>,
    /// Output index.
    pub output_index: u16,
    /// Spend status of the output
    pub is_spent: bool,
    /// Corresponding address
    pub address: String,
    /// Balance amount
    pub amount: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Message {
    network_id: u64,
    parent1: String,
    parent2: String,
    payload: Option<Payload>,
    nonce: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Payload {
    transaction: Option<Vec<Transaction>>,
    milestone: Option<Vec<Milestone>>,
    indexation: Option<Vec<Indexation>>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Transaction {
    pub essence: TransactionPayloadEssence,
    pub unlock_blocks: Vec<UnlockBlock>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Milestone {
    essence: MilestonePayloadEssence,
    signatures: Vec<Vec<u8>>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestonePayloadEssence {
    index: u32,
    timestamp: u64,
    parent1: String,
    parent2: String,
    merkle_proof: [u8; MILESTONE_MERKLE_PROOF_LENGTH],
    public_keys: Vec<[u8; MILESTONE_PUBLIC_KEY_LENGTH]>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Indexation {
    index: String,
    data: Vec<u8>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct TransactionPayloadEssence {
    pub(crate) inputs: Vec<Input>,
    pub(crate) outputs: Vec<Output>,
    pub(crate) payload: Option<Payload>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Output {
    address: String,
    amount: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Input {
    transaction_id: String,
    index: u16,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct UnlockBlock {
    signature: Option<Ed25519Signature>,
    reference: Option<u16>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Ed25519Signature {
    public_key: [u8; 32],
    signature: Vec<u8>,
}

// TODO: Error Handling and split functions
impl From<RustMessage> for Message {
    fn from(msg: RustMessage) -> Self {
        let payload = msg.payload().as_ref();
        let payload = match payload {
            Some(RustPayload::Transaction(payload)) => Some(Payload {
                transaction: Some(vec![Transaction {
                    essence: TransactionPayloadEssence {
                        inputs: payload
                            .essence()
                            .inputs()
                            .iter()
                            .cloned()
                            .map(|input| {
                                if let RustInput::UTXO(input) = input {
                                    Input {
                                        transaction_id: input.output_id().transaction_id().to_string(),
                                        index: input.output_id().index(),
                                    }
                                } else {
                                    unreachable!()
                                }
                            })
                            .collect(),
                        outputs: payload
                            .essence()
                            .outputs()
                            .iter()
                            .cloned()
                            .map(|output| {
                                if let RustOutput::SignatureLockedSingle(output) = output {
                                    Output {
                                        address: output.address().to_bech32(),
                                        amount: output.amount(),
                                    }
                                } else {
                                    unreachable!()
                                }
                            })
                            .collect(),
                        payload: if payload.essence().payload().is_some() {
                            if let Some(RustPayload::Indexation(payload)) = payload.essence().payload() {
                                Some(Payload {
                                    transaction: None,
                                    milestone: None,
                                    indexation: Some(vec![Indexation {
                                        index: payload.index().to_string(),
                                        data: payload.data().try_into().unwrap(),
                                    }]),
                                })
                            } else {
                                unreachable!()
                            }
                        } else {
                            None
                        },
                    },
                    unlock_blocks: payload
                        .unlock_blocks()
                        .iter()
                        .cloned()
                        .map(|unlock_block| {
                            if let RustUnlockBlock::Signature(RustSignatureUnlock::Ed25519(signature)) = unlock_block {
                                UnlockBlock {
                                    signature: Some(Ed25519Signature {
                                        public_key: signature.public_key().to_vec().try_into().unwrap(),
                                        signature: signature.signature().to_vec(),
                                    }),
                                    reference: None,
                                }
                            } else if let RustUnlockBlock::Reference(signature) = unlock_block {
                                UnlockBlock {
                                    signature: None,
                                    reference: Some(signature.index()),
                                }
                            } else {
                                unreachable!()
                            }
                        })
                        .collect(),
                }]),
                milestone: None,
                indexation: None,
            }),
            Some(RustPayload::Indexation(payload)) => Some(Payload {
                transaction: None,
                milestone: None,
                indexation: Some(vec![Indexation {
                    index: payload.index().to_string(),
                    data: payload.data().try_into().unwrap(),
                }]),
            }),
            Some(RustPayload::Milestone(payload)) => Some(Payload {
                transaction: None,
                milestone: Some(vec![Milestone {
                    essence: MilestonePayloadEssence {
                        index: payload.essence().index(),
                        timestamp: payload.essence().timestamp(),
                        parent1: payload.essence().parent1().to_string(),
                        parent2: payload.essence().parent2().to_string(),
                        merkle_proof: payload.essence().merkle_proof().try_into().unwrap(),
                        public_keys: payload
                            .essence()
                            .public_keys()
                            .iter()
                            .map(|public_key| public_key.to_vec().try_into().unwrap())
                            .collect(),
                    },
                    signatures: payload
                        .signatures()
                        .iter()
                        .map(|signature| (*signature).to_vec())
                        .collect(),
                }]),
                indexation: None,
            }),
            _ => None,
        };

        Message {
            network_id: msg.network_id(),
            parent1: msg.parent1().to_string(),
            parent2: msg.parent2().to_string(),
            payload: payload,
            nonce: msg.nonce(),
        }
    }
}

// TODO: Error Handling
impl From<TransactionPayloadEssence> for RustTransactionPayloadEssence {
    fn from(essence: TransactionPayloadEssence) -> Self {
        let mut builder = RustTransactionPayloadEssence::builder();
        let inputs: Vec<RustInput> = essence
            .inputs
            .iter()
            .map(|input| {
                RustUTXOInput::new(
                    RustTransationId::from_str(&input.transaction_id[..]).unwrap(),
                    input.index,
                )
                .unwrap()
                .into()
            })
            .collect();
        for input in inputs {
            builder = builder.add_input(input);
        }

        let outputs: Vec<RustOutput> = essence
            .outputs
            .iter()
            .map(|output| {
                RustSignatureLockedSingleOutput::new(
                    RustAddress::from(
                        RustEd25519Address::from_str(&output.address[..])
                            .unwrap_or_else(|_| panic!("invalid output address: {}", output.address)),
                    ),
                    output.amount,
                )
                .unwrap()
                .into()
            })
            .collect();
        for output in outputs {
            builder = builder.add_output(output);
        }
        if let Some(indexation_payload) = &essence.payload {
            let index = RustIndexationPayload::new(
                indexation_payload.indexation.as_ref().unwrap()[0].index.clone(),
                &(indexation_payload.indexation.as_ref().unwrap()[0].data).clone(),
            )
            .unwrap();
            builder = builder.with_payload(RustPayload::from(index));
        }
        builder.finish().unwrap()
    }
}

// TODO: Error Handling
impl From<Ed25519Signature> for RustSignatureUnlock {
    fn from(signature: Ed25519Signature) -> Self {
        let mut public_key = [0u8; 32];
        hex::decode_to_slice(signature.public_key, &mut public_key).unwrap();
        let signature = hex::decode(signature.signature).unwrap().into_boxed_slice();
        RustEd25519Signature::new(public_key, signature).into()
    }
}

// TODO: Error Handling
impl From<UnlockBlock> for RustUnlockBlock {
    fn from(block: UnlockBlock) -> Self {
        if let Some(signature) = block.signature {
            let sig: RustSignatureUnlock = signature.try_into().unwrap();
            return sig.into();
        } else {
            let reference: RustReferenceUnlock = block.reference.unwrap().try_into().unwrap();
            return reference.into();
        }
    }
}

// TODO: Error Handling
impl From<Payload> for RustPayload {
    fn from(payload: Payload) -> Self {
        if let Some(transaction_payload) = &payload.transaction {
            let mut transaction = RustTransactionPayload::builder();
            transaction = transaction.with_essence(transaction_payload[0].essence.clone().try_into().unwrap());

            let unlock_blocks = transaction_payload[0].unlock_blocks.clone();
            for unlock_block in unlock_blocks {
                transaction = transaction.add_unlock_block(unlock_block.try_into().unwrap());
            }

            return RustPayload::Transaction(Box::new(transaction.finish().unwrap()));
        } else {
            let indexation = RustIndexationPayload::new(
                (&payload.indexation.as_ref().unwrap()[0].index.clone()).to_owned(),
                &payload.indexation.as_ref().unwrap()[0].data,
            )
            .unwrap();
            RustPayload::Indexation(Box::new(indexation))
        }
    }
}

#[derive(DeriveFromPyObject, DeriveIntoPyObject)]
struct BrokerOptions {
    /// automatic disconnect or not
    pub automatic_disconnect: bool,
    /// broker timeout in secs
    pub timeout: u64,
    /// use websockets or not
    pub use_ws: bool,
}

#[derive(DeriveFromPyObject, DeriveIntoPyObject)]
pub struct NodeInfo {
    /// Iota node Name
    pub name: String,
    /// Iota node version
    pub version: String,
    /// Connection status
    pub is_healthy: bool,
    /// coordinator public key
    pub network_id: String,
    /// minimum proof of work score
    pub min_pow_score: f64,
    /// latest milestone index
    pub latest_milestone_index: usize,
    /// solid milestone index
    pub solid_milestone_index: usize,
    /// pruning index
    pub pruning_index: usize,
    /// features
    pub features: Vec<String>,
}

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
        // node_pool_urls: Option<String>,
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
        // TODO: Reopen when the latest version is merged
        // if let Some(node_pool_urls) = node_pool_urls {
        //     client = client.with_pool_urls(node_pool_urls);
        // }
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
                    .with_output_hex(&output.address[..], output.amount)
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
            return rt
                .block_on(async {
                    send_builder
                        .with_seed(&RustSeed::from_ed25519_bytes(seed.as_bytes()).unwrap())
                        .finish()
                        .await
                        .unwrap()
                })
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
            self.client
                .get_unspent_address(&RustSeed::from_ed25519_bytes(seed.as_bytes()).unwrap())
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
        if get_all.unwrap_or(false) {
            let addresses = self
                .client
                .find_addresses(&&RustSeed::from_ed25519_bytes(seed.as_bytes()).unwrap())
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
                .find_addresses(&&RustSeed::from_ed25519_bytes(seed.as_bytes()).unwrap())
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
        rt.block_on(async {
            self.client
                .get_balance(&RustSeed::from_ed25519_bytes(seed.as_bytes()).unwrap())
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
        NodeInfo {
            name: node_info.name,
            version: node_info.version,
            is_healthy: node_info.is_healthy,
            network_id: node_info.network_id,
            min_pow_score: 0.0, //TODO: Change this
            latest_milestone_index: node_info.latest_milestone_index,
            solid_milestone_index: node_info.solid_milestone_index,
            pruning_index: node_info.pruning_index,
            features: node_info.features,
        }
    }
    fn get_tips(&self) -> (String, String) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let tips = rt.block_on(async { self.client.get_tips().await.unwrap() });
        (tips.0.to_string(), tips.1.to_string())
    }
    fn post_message(&self, msg: Message) -> String {
        let mut msg_builder = RustMessageBuilder::<ClientMiner>::new()
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
            index: milestone_metadata.index,
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
