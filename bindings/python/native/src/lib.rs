use iota::{
    builder::Network, Address as RustAddress, AddressBalancePair as RustAddressBalancePair, Api,
    Bech32Address as RustBech32Address, BrokerOptions as RustBrokerOptions, Client as RustClient, ClientMiner,
    Ed25519Address as RustEd25519Address, Ed25519Signature as RustEd25519Signature,
    IndexationPayload as RustIndexationPayload, Input as RustInput, Message as RustMessage,
    MessageBuilder as RustMessageBuilder, MessageId as RustMessageId, NodeInfo as RustNodeInfo, Output as RustOutput,
    OutputMetadata as RustOutputMetadata, Payload as RustPayload, ReferenceUnlock as RustReferenceUnlock,
    Seed as RustSeed, SignatureLockedSingleOutput as RustSignatureLockedSingleOutput,
    SignatureUnlock as RustSignatureUnlock, TransactionId as RustTransationId,
    TransactionPayload as RustTransactionPayload, TransactionPayloadEssence as RustTransactionPayloadEssence,
    UTXOInput as RustUTXOInput, UnlockBlock as RustUnlockBlock,
};

// use pyo3::conversion::IntoPy;
use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::time::Duration;
// extern crate dict_derive;
use dict_derive::{FromPyObject as DeriveFromPyObject, IntoPyObject as DeriveIntoPyObject};
use pyo3::callback::IntoPyCallbackOutput;
use pyo3::types::PyDict;
use pyo3::{AsPyPointer, PyNativeType};
use std::convert::{From, Into, TryFrom, TryInto};
use std::str::FromStr;

#[pyclass]
struct Client {
    client: RustClient,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestoneMetadata {
    /// Milestone index
    pub milestone_index: u64,
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
    essence: String,
    signatures: Vec<Vec<u8>>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Indexation {
    index: String,
    data: Vec<u8>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct TransactionPayloadEssence {
    pub(crate) inputs: Vec<String>,
    pub(crate) outputs: Vec<Output>,
    pub(crate) payload: Option<Payload>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Output {
    address: String,
    amount: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct UnlockBlock {
    signature: Option<Ed25519Signature>,
    reference: Option<u16>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
struct Ed25519Signature {
    public_key: [u8; 32],
    signature: String,
}

// TODO: Error Handling
impl From<TransactionPayloadEssence> for RustTransactionPayloadEssence {
    fn from(essence: TransactionPayloadEssence) -> Self {
        let mut builder = RustTransactionPayloadEssence::builder();
        let inputs: Vec<RustInput> = essence
            .inputs
            .iter()
            .map(|input| {
                RustUTXOInput::from_str(&input[..])
                    .unwrap_or_else(|_| panic!("invalid input: {}", input))
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
    fn send(&self) {}
    fn get_message(&self) {}
    fn find_messages(&self) {}
    fn get_unspent_address(&self) {}
    fn find_addresses(&self) {}
    fn get_balance(&self) {}
    fn get_address_balances(&self) {}
    fn subscriber(&self) {}
    fn retry(&self) {}
    fn reattach(&self) {}
    fn promote(&self) {}
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
    // fn get_address_balance(&self) {} Duplicated w/ get_address_balances
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
    fn find_outputs(&self, output_id: Option<Vec<UTXOInput>>, addresses: Option<Vec<String>>) {}
    fn get_milestone(&self, index: u64) {}
}

/// A Python module implemented in Rust.
#[pymodule]
fn iota_client(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    Ok(())
}
