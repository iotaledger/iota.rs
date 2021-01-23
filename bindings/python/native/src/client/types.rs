// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{
    builder::NetworkInfo as RustNetworkInfo, Address as RustAddress, Ed25519Address as RustEd25519Address,
    Ed25519Signature as RustEd25519Signature, IndexationPayload as RustIndexationPayload, Input as RustInput,
    Message as RustMessage, MessageMetadata as RustMessageMetadata, MilestoneMetadata as RustMilestoneMetadata,
    MilestonePayloadEssence as RustMilestonePayloadEssence, NodeInfo as RustNodeInfo, Output as RustOutput,
    OutputMetadata as RustOutputMetadata, Payload as RustPayload, ReferenceUnlock as RustReferenceUnlock,
    SignatureLockedSingleOutput as RustSignatureLockedSingleOutput, SignatureUnlock as RustSignatureUnlock,
    TransactionId as RustTransationId, TransactionPayload as RustTransactionPayload,
    TransactionPayloadEssence as RustTransactionPayloadEssence, UTXOInput as RustUTXOInput,
    UnlockBlock as RustUnlockBlock,
};

use dict_derive::{FromPyObject as DeriveFromPyObject, IntoPyObject as DeriveIntoPyObject};

use std::convert::{From, Into, TryInto};
use std::str::FromStr;
pub const MILESTONE_MERKLE_PROOF_LENGTH: usize = 32;
pub const MILESTONE_PUBLIC_KEY_LENGTH: usize = 32;
pub static mut BECH32_HRP: &str = "atoi1";

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
    pub milestone_index: u64,
    /// Milestone ID
    pub message_id: String,
    /// Timestamp
    pub timestamp: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct UTXOInput {
    pub transaction_id: Vec<u8>,
    pub index: u16,
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
pub struct Message {
    pub network_id: u64,
    pub parent1: String,
    pub parent2: String,
    pub payload: Option<Payload>,
    pub nonce: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
// TODO: Remove Vec wrapper
pub struct Payload {
    pub transaction: Option<Vec<Transaction>>,
    pub milestone: Option<Vec<Milestone>>,
    pub indexation: Option<Vec<Indexation>>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Transaction {
    pub essence: TransactionPayloadEssence,
    pub unlock_blocks: Vec<UnlockBlock>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Milestone {
    pub essence: MilestonePayloadEssence,
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestonePayloadEssence {
    pub index: u32,
    pub timestamp: u64,
    pub parent1: String,
    pub parent2: String,
    pub merkle_proof: [u8; MILESTONE_MERKLE_PROOF_LENGTH],
    pub public_keys: Vec<[u8; MILESTONE_PUBLIC_KEY_LENGTH]>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Indexation {
    pub index: String,
    pub data: Vec<u8>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct TransactionPayloadEssence {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub payload: Option<Payload>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Output {
    pub address: String,
    pub amount: u64,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Input {
    pub transaction_id: String,
    pub index: u16,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct UnlockBlock {
    pub signature: Option<Ed25519Signature>,
    pub reference: Option<u16>,
}

#[derive(Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Ed25519Signature {
    pub public_key: [u8; 32],
    pub signature: Vec<u8>,
}

#[derive(DeriveFromPyObject, DeriveIntoPyObject)]
pub struct BrokerOptions {
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

pub struct NetworkInfo {
    /// Network of the Iota nodes belong to
    pub network: String,
    /// Network ID
    pub network_id: u64,
    /// Bech32 HRP
    pub bech32_hrp: String,
    /// Mininum proof of work score
    pub min_pow_score: f64,
    /// Local proof of work
    pub local_pow: bool,
}

// TODO: Error Handling
impl From<RustNodeInfo> for NodeInfo {
    fn from(node_info: RustNodeInfo) -> Self {
        NodeInfo {
            name: node_info.name,
            version: node_info.version,
            is_healthy: node_info.is_healthy,
            network_id: node_info.network_id,
            min_pow_score: node_info.min_pow_score,
            latest_milestone_index: node_info.latest_milestone_index,
            solid_milestone_index: node_info.solid_milestone_index,
            pruning_index: node_info.pruning_index,
            features: node_info.features,
        }
    }
}

// TODO: Error Handling
impl From<RustNetworkInfo> for NetworkInfo {
    fn from(network_info: RustNetworkInfo) -> Self {
        NetworkInfo {
            network: network_info.network,
            network_id: network_info.network_id,
            bech32_hrp: network_info.bech32_hrp,
            min_pow_score: network_info.min_pow_score,
            local_pow: network_info.local_pow,
        }
    }
}

// TODO: Error Handling
impl From<RustMessageMetadata> for MessageMetadata {
    fn from(message_metadata: RustMessageMetadata) -> Self {
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
}

// TODO: Error Handling
impl From<RustOutputMetadata> for OutputMetadata {
    fn from(output_metadata: RustOutputMetadata) -> Self {
        OutputMetadata {
            message_id: output_metadata.message_id,
            transaction_id: output_metadata.transaction_id,
            output_index: output_metadata.output_index,
            is_spent: output_metadata.is_spent,
            address: unsafe { output_metadata.address.to_bech32(BECH32_HRP) },
            amount: output_metadata.amount,
        }
    }
}

// TODO: Error Handling
impl From<RustMilestoneMetadata> for MilestoneMetadata {
    fn from(milestone_metadata: RustMilestoneMetadata) -> Self {
        MilestoneMetadata {
            milestone_index: milestone_metadata.milestone_index,
            message_id: milestone_metadata.message_id,
            timestamp: milestone_metadata.timestamp,
        }
    }
}

// TODO: Error Handling
impl From<RustTransactionPayloadEssence> for TransactionPayloadEssence {
    fn from(essence: RustTransactionPayloadEssence) -> Self {
        TransactionPayloadEssence {
            inputs: essence
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
            outputs: essence
                .outputs()
                .iter()
                .cloned()
                .map(|output| {
                    if let RustOutput::SignatureLockedSingle(output) = output {
                        Output {
                            address: unsafe { output.address().to_bech32(BECH32_HRP) },
                            amount: output.amount(),
                        }
                    } else {
                        unreachable!()
                    }
                })
                .collect(),
            payload: if essence.payload().is_some() {
                if let Some(RustPayload::Indexation(payload)) = essence.payload() {
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
        }
    }
}

// TODO: Error Handling
impl From<RustMilestonePayloadEssence> for MilestonePayloadEssence {
    fn from(essence: RustMilestonePayloadEssence) -> Self {
        MilestonePayloadEssence {
            index: essence.index(),
            timestamp: essence.timestamp(),
            parent1: essence.parent1().to_string(),
            parent2: essence.parent2().to_string(),
            merkle_proof: essence.merkle_proof().try_into().unwrap(),
            public_keys: essence
                .public_keys()
                .iter()
                .map(|public_key| public_key.to_vec().try_into().unwrap())
                .collect(),
        }
    }
}

// TODO: Error Handling
impl From<RustUnlockBlock> for UnlockBlock {
    fn from(unlock_block: RustUnlockBlock) -> Self {
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
    }
}

// TODO: Error Handling and split functions
impl From<RustMessage> for Message {
    fn from(msg: RustMessage) -> Self {
        let payload = msg.payload().as_ref();
        let payload = match payload {
            Some(RustPayload::Transaction(payload)) => Some(Payload {
                transaction: Some(vec![Transaction {
                    essence: payload.essence().to_owned().into(),
                    unlock_blocks: payload
                        .unlock_blocks()
                        .iter()
                        .cloned()
                        .map(|unlock_block| unlock_block.into())
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
                    essence: payload.essence().to_owned().into(),
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
