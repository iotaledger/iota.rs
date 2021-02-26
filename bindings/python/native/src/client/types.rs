// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::error::{Error, Result};
use core::convert::TryFrom;
use dict_derive::{FromPyObject as DeriveFromPyObject, IntoPyObject as DeriveIntoPyObject};
use iota::{
    bee_rest_api::{
        handlers::api::v1::{
            balance_ed25519::BalanceForAddressResponse as RustBalanceForAddressResponse,
            info::InfoResponse as RustInfoResponse,
            message_metadata::{
                LedgerInclusionStateDto as RustLedgerInclusionStateDto,
                MessageMetadataResponse as RustMessageMetadataResponse,
            },
            milestone_utxo_changes::MilestoneUtxoChanges as RustMilestoneUTXOChanges,
            output::OutputResponse as RustOutputResponse,
        },
        types::{
            AddressDto as RustAddressDto, Ed25519AddressDto as RustEd25519AddressDto, GossipDto as RustgossipDto,
            HeartbeatDto as RustheartbeatDto, MetricsDto as RustMetricsDto, OutputDto as RustOutputDto,
            PeerDto as RustPeerDto, RelationDto as RustRelationDto,
            SignatureLockedDustAllowanceOutputDto as RustSignatureLockedDustAllowanceOutputDto,
            SignatureLockedSingleOutputDto as RustSignatureLockedSingleOutputDto,
            TreasuryOutputDto as RustTreasuryOutputDto,
        },
    },
    builder::NetworkInfo as RustNetworkInfo,
    client::MilestoneResponse,
    Address as RustAddress, Ed25519Address as RustEd25519Address, Ed25519Signature as RustEd25519Signature,
    Essence as RustEssence, IndexationPayload as RustIndexationPayload, Input as RustInput, Message as RustMessage,
    MilestonePayloadEssence as RustMilestonePayloadEssence, Output as RustOutput, Payload as RustPayload,
    ReferenceUnlock as RustReferenceUnlock, RegularEssence as RustRegularEssence,
    SignatureLockedSingleOutput as RustSignatureLockedSingleOutput, SignatureUnlock as RustSignatureUnlock,
    TransactionId as RustTransationId, TransactionPayload as RustTransactionPayload, UTXOInput as RustUTXOInput,
    UnlockBlock as RustUnlockBlock,
};

use std::{
    convert::{From, Into, TryInto},
    str::FromStr,
};
pub const MILESTONE_MERKLE_PROOF_LENGTH: usize = 32;
pub const MILESTONE_PUBLIC_KEY_LENGTH: usize = 32;
pub static mut BECH32_HRP: &str = "atoi1";

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MessageMetadataResponse {
    /// Message ID
    pub message_id: String,
    /// Message ID of parents
    pub parent_message_ids: Vec<String>,
    /// Solid status
    pub is_solid: bool,
    pub referenced_by_milestone_index: Option<u32>,
    pub milestone_index: Option<u32>,
    pub ledger_inclusion_state: Option<LedgerInclusionStateDto>,
    pub conflict_reason: Option<u8>,
    pub should_promote: Option<bool>,
    pub should_reattach: Option<bool>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct BalanceForAddressResponse {
    // The type of the address (1=Ed25519).
    pub address_type: u8,
    // hex encoded address
    pub address: String,
    pub balance: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct AddressBalancePair {
    /// Address
    pub address: String,
    /// Balance in the address
    pub balance: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestoneDto {
    pub index: u32,
    pub timestamp: u64,
    pub message_id: String,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestoneUTXOChanges {
    pub index: u32,
    pub created_outputs: Vec<String>,
    pub consumed_outputs: Vec<String>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct UTXOInput {
    pub transaction_id: Vec<u8>,
    pub index: u16,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct OutputResponse {
    pub message_id: String,
    pub transaction_id: String,
    pub output_index: u16,
    pub is_spent: bool,
    pub output: OutputDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct OutputDto {
    treasury: Option<TreasuryOutputDto>,
    signature_locked_single: Option<SignatureLockedSingleOutputDto>,
    signature_locked_dust_allowance: Option<SignatureLockedDustAllowanceOutputDto>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct SignatureLockedSingleOutputDto {
    pub kind: u8,
    pub address: AddressDto,
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct SignatureLockedDustAllowanceOutputDto {
    pub kind: u8,
    pub address: AddressDto,
    pub amount: u64,
}

#[derive(Clone, Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct TreasuryOutputDto {
    pub kind: u8,
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct AddressDto {
    ed25519: Ed25519AddressDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Ed25519AddressDto {
    pub kind: u8,
    pub address: String,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Message {
    pub message_id: String,
    pub network_id: u64,
    pub parents: Vec<String>,
    pub payload: Option<Payload>,
    pub nonce: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Payload {
    pub transaction: Option<Vec<Transaction>>,
    pub milestone: Option<Vec<Milestone>>,
    pub indexation: Option<Vec<Indexation>>,
    pub receipt: Option<Vec<Receipt>>,
    pub treasury_transaction: Option<Vec<TreasuryTransaction>>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Transaction {
    pub essence: RegularEssence,
    pub unlock_blocks: Vec<UnlockBlock>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Milestone {
    pub essence: MilestonePayloadEssence,
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MilestonePayloadEssence {
    pub index: u32,
    pub timestamp: u64,
    pub parents: Vec<String>,
    pub merkle_proof: [u8; MILESTONE_MERKLE_PROOF_LENGTH],
    pub public_keys: Vec<[u8; MILESTONE_PUBLIC_KEY_LENGTH]>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Indexation {
    pub index: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Receipt {
    pub kind: u32,
    pub index: u32,
    pub last: bool,
    pub funds: Vec<MigratedFundsEntry>,
    pub transaction: Payload,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MigratedFundsEntry {
    tail_transaction_hash: Vec<u8>,
    address: AddressDto,
    amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct TreasuryTransaction {
    pub kind: u32,
    pub input: Input,
    pub output: Output,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct RegularEssence {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub payload: Option<Payload>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Output {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Input {
    pub transaction_id: String,
    pub index: u16,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct UnlockBlock {
    pub signature: Option<Ed25519Signature>,
    pub reference: Option<u16>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct Ed25519Signature {
    pub public_key: [u8; 32],
    pub signature: Vec<u8>,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct BrokerOptions {
    /// automatic disconnect or not
    pub automatic_disconnect: bool,
    /// broker timeout in secs
    pub timeout: u64,
    /// use websockets or not
    pub use_ws: bool,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct LedgerInclusionStateDto {
    pub state: String,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub is_healthy: bool,
    pub network_id: String,
    pub bech32_hrp: String,
    pub latest_milestone_index: u32,
    pub solid_milestone_index: u32,
    pub pruning_index: u32,
    pub features: Vec<String>,
    pub min_pow_score: f64,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
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
    /// Tips interval
    pub tips_interval: u64,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct PeerDto {
    pub id: String,
    pub multi_addresses: Vec<String>,
    pub alias: Option<String>,
    pub relation: RelationDto,
    pub connected: bool,
    pub gossip: Option<GossipDto>,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct RelationDto {
    pub relation: String,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct GossipDto {
    pub heartbeat: HeartbeatDto,
    pub metrics: MetricsDto,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct HeartbeatDto {
    pub solid_milestone_index: u32,
    pub pruned_milestone_index: u32,
    pub latest_milestone_index: u32,
    pub connected_neighbors: u8,
    pub synced_neighbors: u8,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
pub struct MetricsDto {
    pub new_messages: u64,
    pub received_messages: u64,
    pub known_messages: u64,
    pub received_message_requests: u64,
    pub received_milestone_requests: u64,
    pub received_heartbeats: u64,
    pub sent_messages: u64,
    pub sent_message_requests: u64,
    pub sent_milestone_requests: u64,
    pub sent_heartbeats: u64,
    pub dropped_packets: u64,
}

impl From<RustOutputResponse> for OutputResponse {
    fn from(output: RustOutputResponse) -> Self {
        Self {
            message_id: output.message_id,
            transaction_id: output.transaction_id,
            output_index: output.output_index,
            is_spent: output.is_spent,
            output: output.output.into(),
        }
    }
}

impl From<&iota::MigratedFundsEntry> for MigratedFundsEntry {
    fn from(migrated_funds_entry: &iota::MigratedFundsEntry) -> Self {
        Self {
            tail_transaction_hash: migrated_funds_entry.tail_transaction_hash().to_vec(),
            address: migrated_funds_entry
                .address()
                .clone()
                .try_into()
                .expect("Can't convert address"),
            amount: migrated_funds_entry.amount(),
        }
    }
}

impl From<RustOutputDto> for OutputDto {
    fn from(output: RustOutputDto) -> Self {
        match output {
            RustOutputDto::Treasury(t) => OutputDto {
                treasury: Some(t.into()),
                signature_locked_single: None,
                signature_locked_dust_allowance: None,
            },
            RustOutputDto::SignatureLockedSingle(signature) => OutputDto {
                treasury: None,
                signature_locked_single: Some(signature.into()),
                signature_locked_dust_allowance: None,
            },
            RustOutputDto::SignatureLockedDustAllowance(signature) => OutputDto {
                treasury: None,
                signature_locked_single: None,
                signature_locked_dust_allowance: Some(signature.into()),
            },
        }
    }
}

impl From<RustEd25519AddressDto> for Ed25519AddressDto {
    fn from(address: RustEd25519AddressDto) -> Self {
        Self {
            kind: address.kind,
            address: address.address,
        }
    }
}

impl From<RustTreasuryOutputDto> for TreasuryOutputDto {
    fn from(treasury: RustTreasuryOutputDto) -> Self {
        Self {
            kind: treasury.kind,
            amount: treasury.amount,
        }
    }
}

impl From<RustSignatureLockedSingleOutputDto> for SignatureLockedSingleOutputDto {
    fn from(address: RustSignatureLockedSingleOutputDto) -> Self {
        Self {
            kind: address.kind,
            address: address.address.into(),
            amount: address.amount,
        }
    }
}

impl From<RustSignatureLockedDustAllowanceOutputDto> for SignatureLockedDustAllowanceOutputDto {
    fn from(address: RustSignatureLockedDustAllowanceOutputDto) -> Self {
        Self {
            kind: address.kind,
            address: address.address.into(),
            amount: address.amount,
        }
    }
}

impl From<RustAddressDto> for AddressDto {
    fn from(address: RustAddressDto) -> Self {
        Self {
            ed25519: match address {
                RustAddressDto::Ed25519(ed25519) => ed25519.into(),
            },
        }
    }
}

impl From<RustAddress> for AddressDto {
    fn from(address: RustAddress) -> Self {
        let address = RustAddressDto::try_from(&address).unwrap();
        Self {
            ed25519: match address {
                RustAddressDto::Ed25519(ed25519) => ed25519.into(),
            },
        }
    }
}

impl From<RustBalanceForAddressResponse> for BalanceForAddressResponse {
    fn from(balance_for_address_response: RustBalanceForAddressResponse) -> Self {
        BalanceForAddressResponse {
            address_type: balance_for_address_response.address_type,
            address: balance_for_address_response.address,
            balance: balance_for_address_response.balance,
        }
    }
}

impl From<RustMessageMetadataResponse> for MessageMetadataResponse {
    fn from(message_metadata_response: RustMessageMetadataResponse) -> Self {
        Self {
            message_id: message_metadata_response.message_id,
            parent_message_ids: message_metadata_response.parent_message_ids,
            is_solid: message_metadata_response.is_solid,
            referenced_by_milestone_index: message_metadata_response.referenced_by_milestone_index,
            milestone_index: message_metadata_response.milestone_index,
            ledger_inclusion_state: {
                if let Some(state) = message_metadata_response.ledger_inclusion_state {
                    Some(state.into())
                } else {
                    None
                }
            },
            conflict_reason: message_metadata_response.conflict_reason,
            should_promote: message_metadata_response.should_promote,
            should_reattach: message_metadata_response.should_reattach,
        }
    }
}

impl From<RustInfoResponse> for InfoResponse {
    fn from(info: RustInfoResponse) -> Self {
        InfoResponse {
            name: info.name,
            version: info.version,
            is_healthy: info.is_healthy,
            network_id: info.network_id,
            bech32_hrp: info.bech32_hrp,
            latest_milestone_index: info.latest_milestone_index,
            solid_milestone_index: info.solid_milestone_index,
            pruning_index: info.pruning_index,
            features: info.features,
            min_pow_score: info.min_pow_score,
        }
    }
}

impl From<RustNetworkInfo> for NetworkInfo {
    fn from(network_info: RustNetworkInfo) -> Self {
        NetworkInfo {
            network: network_info.network.unwrap_or_else(|| "undefined".to_string()),
            network_id: network_info.network_id.unwrap_or(0),
            bech32_hrp: network_info.bech32_hrp,
            min_pow_score: network_info.min_pow_score,
            local_pow: network_info.local_pow,
            tips_interval: network_info.tips_interval,
        }
    }
}

impl From<MilestoneResponse> for MilestoneDto {
    fn from(milestone_dto: MilestoneResponse) -> Self {
        Self {
            message_id: milestone_dto.message_id.to_string(),
            index: milestone_dto.index,
            timestamp: milestone_dto.timestamp,
        }
    }
}

impl From<RustMilestoneUTXOChanges> for MilestoneUTXOChanges {
    fn from(milestone_utxo_changes: RustMilestoneUTXOChanges) -> Self {
        Self {
            index: milestone_utxo_changes.index,
            created_outputs: milestone_utxo_changes.created_outputs,
            consumed_outputs: milestone_utxo_changes.consumed_outputs,
        }
    }
}

impl From<RustLedgerInclusionStateDto> for LedgerInclusionStateDto {
    fn from(state: RustLedgerInclusionStateDto) -> Self {
        match state {
            RustLedgerInclusionStateDto::Conflicting => Self {
                state: "Conflicting".to_string(),
            },
            RustLedgerInclusionStateDto::Included => Self {
                state: "Included".to_string(),
            },
            RustLedgerInclusionStateDto::NoTransaction => Self {
                state: "NoTransaction".to_string(),
            },
        }
    }
}

impl From<RustPeerDto> for PeerDto {
    fn from(peer: RustPeerDto) -> Self {
        let gossip = match peer.gossip {
            Some(g) => Some(GossipDto::from(g)),
            None => None,
        };
        Self {
            id: peer.id,
            multi_addresses: peer.multi_addresses,
            alias: peer.alias,
            relation: RelationDto::from(peer.relation),
            connected: peer.connected,
            gossip,
        }
    }
}

impl From<RustRelationDto> for RelationDto {
    fn from(relation: RustRelationDto) -> Self {
        match relation {
            RustRelationDto::Known => Self {
                relation: "known".to_string(),
            },
            RustRelationDto::Unknown => Self {
                relation: "unknown".to_string(),
            },
            RustRelationDto::Discovered => Self {
                relation: "discovered".to_string(),
            },
        }
    }
}

impl From<RustgossipDto> for GossipDto {
    fn from(gossip: RustgossipDto) -> Self {
        Self {
            heartbeat: HeartbeatDto::from(gossip.heartbeat),
            metrics: MetricsDto::from(gossip.metrics),
        }
    }
}

impl From<RustheartbeatDto> for HeartbeatDto {
    fn from(heartbeat: RustheartbeatDto) -> Self {
        Self {
            solid_milestone_index: heartbeat.solid_milestone_index,
            pruned_milestone_index: heartbeat.pruned_milestone_index,
            latest_milestone_index: heartbeat.latest_milestone_index,
            connected_neighbors: heartbeat.connected_neighbors,
            synced_neighbors: heartbeat.synced_neighbors,
        }
    }
}

impl From<RustMetricsDto> for MetricsDto {
    fn from(metrics: RustMetricsDto) -> Self {
        Self {
            new_messages: metrics.new_messages,
            received_messages: metrics.received_messages,
            known_messages: metrics.known_messages,
            received_message_requests: metrics.received_message_requests,
            received_milestone_requests: metrics.received_milestone_requests,
            received_heartbeats: metrics.received_heartbeats,
            sent_messages: metrics.sent_messages,
            sent_message_requests: metrics.sent_message_requests,
            sent_milestone_requests: metrics.sent_milestone_requests,
            sent_heartbeats: metrics.sent_heartbeats,
            dropped_packets: metrics.dropped_packets,
        }
    }
}

impl TryFrom<RustRegularEssence> for RegularEssence {
    type Error = Error;
    fn try_from(essence: RustRegularEssence) -> Result<Self> {
        Ok(RegularEssence {
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
                            index: hex::encode(payload.index()),
                            data: payload.data().try_into().unwrap_or_else(|_| {
                                panic!(
                                    "invalid Indexation Payload {:?} with data: {:?}",
                                    essence,
                                    payload.data()
                                )
                            }),
                        }]),
                        receipt: None,
                        treasury_transaction: None,
                    })
                } else {
                    unreachable!()
                }
            } else {
                None
            },
        })
    }
}

impl TryFrom<RustMilestonePayloadEssence> for MilestonePayloadEssence {
    type Error = Error;
    fn try_from(essence: RustMilestonePayloadEssence) -> Result<Self> {
        Ok(MilestonePayloadEssence {
            index: essence.index(),
            timestamp: essence.timestamp(),
            parents: vec![essence.parents().iter().map(|m| m.to_string()).collect()],
            merkle_proof: essence.merkle_proof().try_into()?,
            public_keys: essence
                .public_keys()
                .iter()
                .map(|public_key| {
                    public_key.to_vec().try_into().unwrap_or_else(|_| {
                        panic!(
                            "invalid MilestonePayloadEssence {:?} with public key: {:?}",
                            essence,
                            essence.public_keys()
                        )
                    })
                })
                .collect(),
        })
    }
}

impl TryFrom<RustUnlockBlock> for UnlockBlock {
    type Error = Error;
    fn try_from(unlock_block: RustUnlockBlock) -> Result<Self> {
        if let RustUnlockBlock::Signature(RustSignatureUnlock::Ed25519(signature)) = unlock_block {
            Ok(UnlockBlock {
                signature: Some(Ed25519Signature {
                    public_key: signature.public_key().to_vec().try_into().unwrap_or_else(|_| {
                        panic!(
                            "invalid Ed25519Signature {:?} with public key: {:?}",
                            signature,
                            signature.public_key()
                        )
                    }),
                    signature: signature.signature().to_vec(),
                }),
                reference: None,
            })
        } else if let RustUnlockBlock::Reference(signature) = unlock_block {
            Ok(UnlockBlock {
                signature: None,
                reference: Some(signature.index()),
            })
        } else {
            unreachable!()
        }
    }
}

impl TryFrom<RustMessage> for Message {
    type Error = Error;
    fn try_from(msg: RustMessage) -> Result<Self> {
        let payload = msg.payload().as_ref();
        let payload = match payload {
            Some(RustPayload::Transaction(payload)) => {
                let essence = match payload.essence().to_owned() {
                    RustEssence::Regular(e) => e.try_into()?,
                    _ => panic!("Unexisting essence."),
                };

                Some(Payload {
                    transaction: Some(vec![Transaction {
                        essence,
                        unlock_blocks: payload
                            .unlock_blocks()
                            .iter()
                            .cloned()
                            .map(|unlock_block| unlock_block.try_into().expect("Invalid UnlockBlock"))
                            .collect(),
                    }]),
                    milestone: None,
                    indexation: None,
                    receipt: None,
                    treasury_transaction: None,
                })
            }
            Some(RustPayload::Indexation(payload)) => Some(Payload {
                transaction: None,
                milestone: None,
                indexation: Some(vec![Indexation {
                    index: hex::encode(payload.index()),
                    data: payload.data().try_into().unwrap_or_else(|_| {
                        panic!(
                            "invalid Indexation Payload {:?} with data: {:?}",
                            payload,
                            payload.data()
                        )
                    }),
                }]),
                receipt: None,
                treasury_transaction: None,
            }),
            Some(RustPayload::Milestone(payload)) => Some(Payload {
                transaction: None,
                milestone: Some(vec![Milestone {
                    essence: payload.essence().to_owned().try_into()?,
                    signatures: payload
                        .signatures()
                        .iter()
                        .map(|signature| (*signature).to_vec())
                        .collect(),
                }]),
                indexation: None,
                receipt: None,
                treasury_transaction: None,
            }),
            Some(RustPayload::Receipt(payload)) => {
                let essence = match payload.transaction() {
                    RustPayload::Transaction(transaction_payload) => match transaction_payload.essence().to_owned() {
                        RustEssence::Regular(e) => e.try_into()?,
                        _ => panic!("Unexisting essence."),
                    },
                    _ => panic!("Missing transaction payload"),
                };
                Some(Payload {
                    transaction: None,
                    milestone: None,
                    indexation: None,
                    receipt: Some(vec![Receipt {
                        kind: 3,
                        index: payload.index(),
                        last: payload.last(),
                        funds: payload
                            .funds()
                            .iter()
                            .map(|m| m.try_into().expect("Couldn't convert funds"))
                            .collect::<Vec<MigratedFundsEntry>>(),
                        transaction: Payload {
                            transaction: Some(vec![Transaction {
                                essence,
                                unlock_blocks: match payload.transaction() {
                                    RustPayload::Transaction(transaction_payload) => transaction_payload
                                        .unlock_blocks()
                                        .iter()
                                        .cloned()
                                        .map(|unlock_block| unlock_block.try_into().expect("Invalid UnlockBlock"))
                                        .collect(),
                                    _ => panic!("Missing transaction payload"),
                                },
                            }]),
                            milestone: None,
                            indexation: None,
                            receipt: None,
                            treasury_transaction: None,
                        },
                    }]),
                    treasury_transaction: None,
                })
            }
            _ => None,
        };

        Ok(Message {
            message_id: msg.id().0.to_string(),
            network_id: msg.network_id(),
            parents: msg.parents().iter().map(|m| m.to_string()).collect(),
            payload,
            nonce: msg.nonce(),
        })
    }
}

impl TryFrom<RegularEssence> for RustRegularEssence {
    type Error = Error;
    fn try_from(essence: RegularEssence) -> Result<Self> {
        let mut builder = RustRegularEssence::builder();
        let inputs: Vec<RustInput> = essence
            .inputs
            .iter()
            .map(|input| {
                RustUTXOInput::new(
                    RustTransationId::from_str(&input.transaction_id[..]).unwrap_or_else(|_| {
                        panic!(
                            "invalid UTXOInput transaction_id: {} with input index {}",
                            input.transaction_id, input.index
                        )
                    }),
                    input.index,
                )
                .unwrap_or_else(|_| {
                    panic!(
                        "invalid UTXOInput transaction_id: {} with input index {}",
                        input.transaction_id, input.index
                    )
                })
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
                    RustAddress::from(RustEd25519Address::from_str(&output.address[..]).unwrap_or_else(|_| {
                        panic!(
                            "invalid SignatureLockedSingleOutput with output address: {}",
                            output.address
                        )
                    })),
                    output.amount,
                )
                .unwrap_or_else(|_| {
                    panic!(
                        "invalid SignatureLockedSingleOutput with output address: {}",
                        output.address
                    )
                })
                .into()
            })
            .collect();
        for output in outputs {
            builder = builder.add_output(output);
        }
        if let Some(indexation_payload) = &essence.payload {
            let index = RustIndexationPayload::new(
                indexation_payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid IndexationPayload: {:?}", indexation_payload))[0]
                    .index
                    .clone()
                    .as_bytes(),
                &(indexation_payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid IndexationPayload: {:?}", indexation_payload))[0]
                    .data)
                    .clone(),
            )
            .unwrap();
            builder = builder.with_payload(RustPayload::from(index));
        }
        Ok(builder.finish()?)
    }
}

impl TryFrom<Ed25519Signature> for RustSignatureUnlock {
    type Error = Error;
    fn try_from(signature: Ed25519Signature) -> Result<Self> {
        let mut public_key = [0u8; 32];
        hex::decode_to_slice(signature.public_key, &mut public_key)?;
        let signature = hex::decode(signature.signature)?.into_boxed_slice();
        Ok(RustEd25519Signature::new(public_key, signature).into())
    }
}

impl TryFrom<UnlockBlock> for RustUnlockBlock {
    type Error = Error;
    fn try_from(block: UnlockBlock) -> Result<Self> {
        if let Some(signature) = block.signature {
            let sig: RustSignatureUnlock = signature.try_into()?;
            Ok(sig.into())
        } else {
            let reference: RustReferenceUnlock = block
                .reference
                .unwrap()
                .try_into()
                .unwrap_or_else(|_| panic!("Invalid ReferenceUnlock: {:?}", block.reference));
            Ok(reference.into())
        }
    }
}

impl TryFrom<Payload> for RustPayload {
    type Error = Error;
    fn try_from(payload: Payload) -> Result<Self> {
        if let Some(transaction_payload) = &payload.transaction {
            let mut transaction = RustTransactionPayload::builder();
            transaction =
                transaction.with_essence(RustEssence::Regular(transaction_payload[0].essence.clone().try_into()?));

            let unlock_blocks = transaction_payload[0].unlock_blocks.clone();
            for unlock_block in unlock_blocks {
                transaction = transaction.add_unlock_block(unlock_block.try_into()?);
            }

            Ok(RustPayload::Transaction(Box::new(transaction.finish()?)))
        } else {
            let indexation = RustIndexationPayload::new(
                (&payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid Payload: {:?}", payload))[0]
                    .index
                    .clone())
                    .to_owned()
                    .as_bytes(),
                &payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid Payload: {:?}", payload))[0]
                    .data,
            )?;
            Ok(RustPayload::Indexation(Box::new(indexation)))
        }
    }
}
