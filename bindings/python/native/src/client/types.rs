// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::client::error::{Error, Result};
use core::convert::TryFrom;
use dict_derive::{FromPyObject as DeriveFromPyObject, IntoPyObject as DeriveIntoPyObject};
use iota_client::{
    api::{AddressIndexRecorder as RustAddressIndexRecorder, PreparedTransactionData as RustPreparedTransactionData},
    bee_message::prelude::{
        Address as RustAddress, Ed25519Address as RustEd25519Address, Ed25519Signature as RustEd25519Signature,
        Essence as RustEssence, IndexationPayload as RustIndexationPayload, Input as RustInput, Message as RustMessage,
        MigratedFundsEntry as RustMigratedFundsEntry, MilestonePayloadEssence as RustMilestonePayloadEssence,
        Output as RustOutput, Payload as RustPayload, ReferenceUnlock as RustReferenceUnlock,
        RegularEssence as RustRegularEssence,
        SignatureLockedDustAllowanceOutput as RustSignatureLockedDustAllowanceOutput,
        SignatureLockedSingleOutput as RustSignatureLockedSingleOutput, SignatureUnlock as RustSignatureUnlock,
        TransactionId as RustTransactionId, TransactionPayload as RustTransactionPayload,
        UnlockBlock as RustUnlockBlock, UnlockBlocks as RustUnlockBlocks, UtxoInput as RustUtxoInput,
    },
    bee_rest_api::types::{
        dtos::{
            AddressDto as RustAddressDto, Ed25519AddressDto as RustEd25519AddressDto, GossipDto as RustgossipDto,
            HeartbeatDto as RustheartbeatDto, InputDto as RustInputDto,
            LedgerInclusionStateDto as RustLedgerInclusionStateDto, MetricsDto as RustMetricsDto,
            MigratedFundsEntryDto as RustMigratedFundsEntryDto, OutputDto as RustOutputDto,
            PayloadDto as RustPayloadDto, PeerDto as RustPeerDto, ReceiptDto as RustReceiptDto,
            ReceiptPayloadDto as RustReceiptPayloadDto, RelationDto as RustRelationDto,
            SignatureLockedDustAllowanceOutputDto as RustSignatureLockedDustAllowanceOutputDto,
            SignatureLockedSingleOutputDto as RustSignatureLockedSingleOutputDto,
            TreasuryOutputDto as RustTreasuryOutputDto,
            TreasuryTransactionPayloadDto as RustTreasuryTransactionPayloadDto,
        },
        responses::{
            BalanceAddressResponse as RustBalanceAddressResponse,
            MessageMetadataResponse as RustMessageMetadataResponse, OutputResponse as RustOutputResponse,
            TreasuryResponse as RustTreasuryResponse, UtxoChangesResponse as RustUtxoChangesResponse,
        },
    },
    builder::NetworkInfo as RustNetworkInfo,
    client::{MilestoneResponse, NodeInfoWrapper as RustNodeInfoWrapper},
    crypto::keys::slip10::{Chain as RustChain, Segment as RustSegment},
    AddressOutputsOptions as RustAddressOutputsOptions, OutputType,
};

use std::{
    convert::{From, Into, TryInto},
    str::FromStr,
};

/// The length of milestone Merkle tree.
pub const MILESTONE_MERKLE_PROOF_LENGTH: usize = 32;
/// The length of milestone public key.
pub const MILESTONE_PUBLIC_KEY_LENGTH: usize = 32;
/// The Bech32_human-readable part.
pub static mut BECH32_HRP: &str = "atoi1";

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The metadata of a message.
pub struct MessageMetadataResponse {
    /// Message ID
    pub message_id: String,
    /// Message ID of parents
    pub parent_message_ids: Vec<String>,
    /// Solid status
    pub is_solid: bool,
    /// The milestone index which refers to the message.
    pub referenced_by_milestone_index: Option<u32>,
    /// The milestone index.
    pub milestone_index: Option<u32>,
    /// The ledger inclusion state.
    pub ledger_inclusion_state: Option<LedgerInclusionStateDto>,
    /// The reason of conflict.
    pub conflict_reason: Option<u8>,
    /// The message should be promoted.
    pub should_promote: Option<bool>,
    /// The message should be reattached.
    pub should_reattach: Option<bool>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The balance of the address.
pub struct BalanceAddressResponse {
    /// The type of the address (1=Ed25519).
    pub address_type: u8,
    /// The hex encoded address.
    pub address: String,
    /// The balance of the address.
    pub balance: u64,
    /// Wether the address is dust allowed.
    pub dust_allowed: bool,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The address balance pair.
pub struct AddressBalancePair {
    /// The address string.
    pub address: String,
    /// The balance in the address.
    pub balance: u64,
    /// Whether this address allowed dust.
    pub dust_allowed: bool,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The structure describes milestone data transfer object.
pub struct MilestoneDto {
    /// The milestone index.
    pub index: u32,
    /// The timestamp of the milestone.
    pub timestamp: u64,
    /// The message id of the milestone.
    pub message_id: String,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The UTXO changes that happened at a specific milestone.
pub struct MilestoneUTXOChanges {
    /// The milestone index.
    pub index: u32,
    /// The created outputs.
    pub created_outputs: Vec<String>,
    /// The consumed outputs.
    pub consumed_outputs: Vec<String>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The input structure.
pub struct InputDto {
    /// The UTXO input.
    pub utxo: Option<UtxoInput>,
    /// The treasury Input.
    pub treasury: Option<TreasuryInput>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The UTXO input referencing an output.
pub struct UtxoInput {
    /// The transaction id.
    pub transaction_id: Vec<u8>,
    /// The index.
    pub index: u16,
}
#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The treasury input.
pub struct TreasuryInput {
    /// The input kind.
    pub kind: u8,
    /// The message id.
    pub message_id: String,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The output response.
pub struct OutputResponse {
    /// The message id.
    pub message_id: String,
    /// The transaction id.
    pub transaction_id: String,
    /// The output index.
    pub output_index: u16,
    /// Indicates whether the output is spent or not.
    pub is_spent: bool,
    /// The output.
    pub output: OutputDto,
    /// The ledger index.
    pub ledger_index: u32,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// Describes all the different output types.
pub struct OutputDto {
    /// The treasury output.
    pub treasury: Option<TreasuryOutputDto>,
    /// The signature locked single output.
    pub signature_locked_single: Option<SignatureLockedSingleOutputDto>,
    /// The signature locked dust-allowance output
    pub signature_locked_dust_allowance: Option<SignatureLockedDustAllowanceOutputDto>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The signature locked single output.
pub struct SignatureLockedSingleOutputDto {
    /// The output kind.
    pub kind: u8,
    /// The address.
    pub address: AddressDto,
    /// The token amount.
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The signature locked dust-allowance output.
pub struct SignatureLockedDustAllowanceOutputDto {
    /// The output kind.
    pub kind: u8,
    /// The address.
    pub address: AddressDto,
    /// The token amount.
    pub amount: u64,
}

#[derive(Clone, Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The treasury output.
pub struct TreasuryOutputDto {
    /// The output kind.
    pub kind: u8,
    /// The token amount.
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The structure stores address.
pub struct AddressDto {
    /// The Ed25519 address.
    pub ed25519: Ed25519AddressDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The Ed25519 address structure.
pub struct Ed25519AddressDto {
    /// The address kind.
    pub kind: u8,
    /// The address in string.
    pub address: String,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The structure contains information of a message.
pub struct Message {
    /// The message id.
    pub message_id: String,
    /// The network id.
    pub network_id: u64,
    /// The parents of the message.
    pub parents: Vec<String>,
    /// The messgae payload.
    pub payload: Option<Payload>,
    /// The nonce of the message.
    pub nonce: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The structure defines the payload in a message.
pub struct Payload {
    /// The transaction payload.
    pub transaction: Option<Vec<Transaction>>,
    /// The milestone payload.
    pub milestone: Option<Vec<Milestone>>,
    /// The indexation payload.
    pub indexation: Option<Vec<Indexation>>,
    /// The receipt payload.
    pub receipt: Option<Vec<Receipt>>,
    /// The treasury transaction payload.
    pub treasury_transaction: Option<Vec<TreasuryTransaction>>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The payload which defines the transaction.
pub struct Transaction {
    /// The transaction regular essence.
    pub essence: RegularEssence,
    /// The unlock blocks for the transaction.
    pub unlock_blocks: Vec<UnlockBlock>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// A payload which defines the inclusion set of other messages in the Tangle.
pub struct Milestone {
    /// The milestone essence.
    pub essence: MilestonePayloadEssence,
    /// The milestone signatures.
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The essence of the milestone payload.
pub struct MilestonePayloadEssence {
    /// The index.
    pub index: u32,
    /// The timestamp.
    pub timestamp: u64,
    /// The parents.
    pub parents: Vec<String>,
    /// The Merkle proof.
    pub merkle_proof: [u8; MILESTONE_MERKLE_PROOF_LENGTH],
    /// The next proof-of-work score.
    pub next_pow_score: u32,
    /// The next proof-of-work milestone index.
    pub next_pow_score_milestone_index: u32,
    /// The public keys.
    pub public_keys: Vec<[u8; MILESTONE_PUBLIC_KEY_LENGTH]>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The indexation payload.
pub struct Indexation {
    /// The index.
    pub index: String,
    /// The indexation data.
    pub data: Vec<u8>,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The receipt data transfer object.
pub struct ReceiptDto {
    /// The receipt.
    pub receipt: Receipt,
    /// The milestone index.
    pub milestone_index: u32,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The structure describes receipt.
pub struct Receipt {
    /// The receipt kind.
    pub kind: u32,
    /// The receipt index.
    pub index: u32,
    /// If the receipt is the last one.
    pub last: bool,
    /// The migrated fund entries
    pub funds: Vec<MigratedFundsEntry>,
    /// The transaction payload.
    pub transaction: Payload,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The migrated funds entry.
pub struct MigratedFundsEntry {
    /// The tail transaction hash.
    pub tail_transaction_hash: String,
    /// The signature locked single output.
    pub output: SignatureLockedSingleOutputDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The treasury transaction.
pub struct TreasuryTransaction {
    /// The transaction kind.
    pub kind: u32,
    /// The input.
    pub input: InputDto,
    /// The output.
    pub output: OutputDto,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The regular essence.
pub struct RegularEssence {
    /// The inputs.
    pub inputs: Vec<Input>,
    /// The transaction outputs.
    pub outputs: Vec<TransactionOutput>,
    /// The payload.
    pub payload: Option<Payload>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The output structure describes an address and the corresponding token amount.
pub struct Output {
    /// The address.
    pub address: String,
    /// The amount.
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The transaction output.
pub struct TransactionOutput {
    /// The signature locked single output.
    pub signature_locked_single: Option<SignatureLockedSingleOutput>,
    /// The signature locked dust-allowance output.
    pub signature_locked_dust_allowance: Option<SignatureLockedDustAllowanceOutput>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The signature locked single output.
pub struct SignatureLockedSingleOutput {
    /// The output address.
    pub address: String,
    /// The token amount.
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The signature locked dust-allowance output.
pub struct SignatureLockedDustAllowanceOutput {
    /// The output address.
    pub address: String,
    /// The token amount.
    pub amount: u64,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The transaction input.
pub struct Input {
    /// The transaction id.
    pub transaction_id: String,
    /// The index.
    pub index: u16,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The unlock block.
pub struct UnlockBlock {
    /// The Ed25519 Signature.
    pub signature: Option<Ed25519Signature>,
    /// The reference block id.
    pub reference: Option<u16>,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The Ed25519 Signature.
pub struct Ed25519Signature {
    /// The public key.
    pub public_key: [u8; 32],
    /// The signature.
    pub signature: Vec<u8>,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The broker options.
pub struct BrokerOptions {
    /// Enable automatic disconnect or not.
    pub automatic_disconnect: bool,
    /// The broker timeout in secs.
    pub timeout: u64,
    /// To use websocket instead of tcp,
    pub use_ws: bool,
    /// The port id.
    pub port: u16,
    /// The max number of reconnection attempts.
    pub max_reconnection_attempts: usize,
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The ledger inclusion state.
pub struct LedgerInclusionStateDto {
    /// The inclusion state.
    pub state: String,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// NodeInfo wrapper which contains the `NodeInfo` and the `url` from the node
/// This structure is useful when multiple nodes are used.
pub struct NodeInfoWrapper {
    /// The node information.
    pub nodeinfo: NodeInfo,
    /// The url of the node.
    pub url: String,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The node information.
pub struct NodeInfo {
    /// The node name.
    pub name: String,
    /// The node version.
    pub version: String,
    /// The node is healthy or not.
    pub is_healthy: bool,
    /// The network id of the node.
    pub network_id: String,
    /// The Bech32 human-readable part.
    pub bech32_hrp: String,
    /// The minimum proof-of-work score.
    pub min_pow_score: f64,
    /// The messages per seconds.
    pub messages_per_second: f64,
    /// The referenced messages per second.
    pub referenced_messages_per_second: f64,
    /// The referenced rate.
    pub referenced_rate: f64,
    /// The latest milestone timestamp.
    pub latest_milestone_timestamp: u64,
    /// The latest milestone index.
    pub latest_milestone_index: u32,
    /// The confirmed milestone index.
    pub confirmed_milestone_index: u32,
    /// The prunning index.
    pub pruning_index: u32,
    /// Describes the features of the node.
    pub features: Vec<String>,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The network information of the node.
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
/// The structure describes peer node.
pub struct PeerDto {
    /// The peer id.
    pub id: String,
    /// The peer addresses.
    pub multi_addresses: Vec<String>,
    /// The peer alias.
    pub alias: Option<String>,
    /// The relation with the peer.
    pub relation: RelationDto,
    /// Is connected to the peer.
    pub connected: bool,
    /// The gossip information with the peer.
    pub gossip: Option<GossipDto>,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// Describes the relation with the peer.
pub struct RelationDto {
    /// The relation with the peer.
    pub relation: String,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The metrics of all information about the gossip stream with the peer.
pub struct GossipDto {
    /// The heartbeat of a node.
    pub heartbeat: HeartbeatDto,
    /// The metrics of the gossip stream.
    pub metrics: MetricsDto,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The heartbeat of a node.
pub struct HeartbeatDto {
    /// The solid milestone index.
    pub solid_milestone_index: u32,
    /// The pruned milestone index.
    pub pruned_milestone_index: u32,
    /// The last milestone index.
    pub latest_milestone_index: u32,
    /// The connected neighbor count.
    pub connected_neighbors: u8,
    /// The synced neighbor count.
    pub synced_neighbors: u8,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The data transfer object for metrics of a gossip stream.
pub struct MetricsDto {
    /// The new message count.
    pub new_messages: u64,
    /// The received message count.
    pub received_messages: u64,
    /// The known message count.
    pub known_messages: u64,
    /// The received message request count.
    pub received_message_requests: u64,
    /// The received milestone request count.
    pub received_milestone_requests: u64,
    /// The received heartbeat count.
    pub received_heartbeats: u64,
    /// The sent message count.
    pub sent_messages: u64,
    /// The sent message request count.
    pub sent_message_requests: u64,
    /// The sent milestone request count.
    pub sent_milestone_requests: u64,
    /// The sent heartbeat count.
    pub sent_heartbeats: u64,
    /// The dropped packet count.
    pub dropped_packets: u64,
}

#[derive(Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// The treasury response.
pub struct TreasuryResponse {
    /// Milestone id.
    pub milestone_id: String,
    /// Token amount.
    pub amount: u64,
}

#[derive(Clone, Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// Helper struct for offline signing.
pub struct PreparedTransactionData {
    /// Transaction essence.AddressBalancePair
    pub essence: RegularEssence,
    /// Required address information for signing.
    pub address_index_recorders: Vec<AddressIndexRecorder>,
}

#[derive(Clone, Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// Structure for sorting of UnlockBlocks.
pub struct AddressIndexRecorder {
    /// The account index.
    account_index: usize,
    /// The input.
    input: Input,
    /// The output response.
    output: OutputResponse,
    /// The address index.
    address_index: usize,
    /// The `Segment` vector.
    chain: Vec<Segment>,
    /// Whether is the address internal.
    internal: bool,
    /// The Bech32 address.
    bech32_address: String,
}

#[derive(Clone, Debug, DeriveFromPyObject, DeriveIntoPyObject)]
/// Structure for sorting of UnlockBlocks
pub struct Segment {
    /// Whether is it hardened.
    hardened: bool,
    /// Bytes
    bs: [u8; 4],
}

impl Segment {
    /// Convert the Segment from its representation as a byte array in big endian.
    pub fn as_u32_be(&self) -> u32 {
        if self.hardened {
            u32::from_be_bytes(self.bs) | RustSegment::HARDEN_MASK
        } else {
            u32::from_be_bytes(self.bs)
        }
    }
}

impl TryFrom<UtxoInput> for RustUtxoInput {
    type Error = Error;
    fn try_from(input: UtxoInput) -> Result<Self> {
        let mut input_array: [u8; 32] = Default::default();
        input_array.copy_from_slice(&input.transaction_id[..32]);
        Ok(
            RustUtxoInput::new(RustTransactionId::from(input_array), input.index).unwrap_or_else(|_| {
                panic!(
                    "invalid UtxoInput transaction_id: {:?} with input index {}",
                    input.transaction_id, input.index
                )
            }),
        )
    }
}

impl From<RustAddressIndexRecorder> for AddressIndexRecorder {
    fn from(recorder: RustAddressIndexRecorder) -> Self {
        Self {
            account_index: recorder.account_index,
            input: if let RustInput::Utxo(input) = recorder.input {
                Input {
                    transaction_id: input.output_id().transaction_id().to_string(),
                    index: input.output_id().index(),
                }
            } else {
                unreachable!()
            },
            output: recorder.output.into(),
            address_index: recorder.address_index,
            chain: recorder
                .chain
                .segments()
                .iter()
                .cloned()
                .map(|segment| Segment {
                    hardened: segment.hardened(),
                    bs: segment.bs(),
                })
                .collect(),
            internal: recorder.internal,
            bech32_address: recorder.bech32_address,
        }
    }
}

impl TryFrom<AddressIndexRecorder> for RustAddressIndexRecorder {
    type Error = Error;
    fn try_from(recorder: AddressIndexRecorder) -> Result<Self> {
        Ok(Self {
            account_index: recorder.account_index,
            input: RustUtxoInput::new(
                RustTransactionId::from_str(&recorder.input.transaction_id[..]).unwrap_or_else(|_| {
                    panic!(
                        "invalid UtxoInput transaction_id: {} with input index {}",
                        recorder.input.transaction_id, recorder.input.index
                    )
                }),
                recorder.input.index,
            )
            .unwrap_or_else(|_| {
                panic!(
                    "invalid UtxoInput transaction_id: {} with input index {}",
                    recorder.input.transaction_id, recorder.input.index
                )
            })
            .into(),
            output: RustOutputResponse {
                message_id: recorder.output.message_id,
                transaction_id: recorder.output.transaction_id,
                output_index: recorder.output.output_index,
                is_spent: recorder.output.is_spent,
                output: recorder.output.output.into(),
                ledger_index: recorder.output.ledger_index,
            },
            address_index: recorder.address_index,
            chain: RustChain::from_u32(
                recorder
                    .chain
                    .iter()
                    .clone()
                    .map(|s| s.as_u32_be())
                    .collect::<Vec<u32>>(),
            ),
            internal: recorder.internal,
            bech32_address: recorder.bech32_address,
        })
    }
}

impl TryFrom<RustPreparedTransactionData> for PreparedTransactionData {
    type Error = Error;
    fn try_from(data: RustPreparedTransactionData) -> Result<Self> {
        Ok(PreparedTransactionData {
            essence: match data.essence {
                RustEssence::Regular(e) => e.try_into()?,
            },
            address_index_recorders: data
                .address_index_recorders
                .iter()
                .cloned()
                .map(|recorder| recorder.into())
                .collect(),
        })
    }
}

impl TryFrom<PreparedTransactionData> for RustPreparedTransactionData {
    type Error = Error;
    fn try_from(data: PreparedTransactionData) -> Result<Self> {
        Ok(Self {
            essence: RustEssence::Regular(data.essence.clone().try_into()?),
            address_index_recorders: data
                .address_index_recorders
                .iter()
                .map(|recorder| {
                    recorder
                        .clone()
                        .try_into()
                        .unwrap_or_else(|_| panic!("invalid AddressIndexRecorder {data:?}"))
                })
                .collect(),
        })
    }
}

impl From<RustTreasuryResponse> for TreasuryResponse {
    fn from(treasury: RustTreasuryResponse) -> Self {
        Self {
            milestone_id: treasury.milestone_id,
            amount: treasury.amount,
        }
    }
}

impl From<RustOutputResponse> for OutputResponse {
    fn from(output: RustOutputResponse) -> Self {
        Self {
            message_id: output.message_id,
            transaction_id: output.transaction_id,
            output_index: output.output_index,
            is_spent: output.is_spent,
            output: output.output.into(),
            ledger_index: output.ledger_index,
        }
    }
}

impl From<&RustMigratedFundsEntry> for MigratedFundsEntry {
    fn from(migrated_funds_entry: &RustMigratedFundsEntry) -> Self {
        Self {
            tail_transaction_hash: migrated_funds_entry.tail_transaction_hash().to_string(),
            output: migrated_funds_entry.output().clone().into(),
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

impl From<OutputDto> for RustOutputDto {
    fn from(output: OutputDto) -> Self {
        if let Some(treasury) = output.treasury {
            RustOutputDto::Treasury(RustTreasuryOutputDto {
                kind: treasury.kind,
                amount: treasury.amount,
            })
        } else if let Some(signature) = output.signature_locked_single {
            RustOutputDto::SignatureLockedSingle(RustSignatureLockedSingleOutputDto {
                kind: signature.kind,
                address: RustAddressDto::Ed25519(RustEd25519AddressDto {
                    kind: signature.address.ed25519.kind,
                    address: signature.address.ed25519.address,
                }),
                amount: signature.amount,
            })
        } else if let Some(signature) = output.signature_locked_dust_allowance {
            RustOutputDto::SignatureLockedDustAllowance(RustSignatureLockedDustAllowanceOutputDto {
                kind: signature.kind,
                address: RustAddressDto::Ed25519(RustEd25519AddressDto {
                    kind: signature.address.ed25519.kind,
                    address: signature.address.ed25519.address,
                }),
                amount: signature.amount,
            })
        } else {
            unreachable!()
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

impl From<RustSignatureLockedSingleOutput> for SignatureLockedSingleOutputDto {
    fn from(address: RustSignatureLockedSingleOutput) -> Self {
        let address_dto: AddressDto = (*address.address()).into();
        Self {
            kind: 0,
            address: address_dto,
            amount: address.amount(),
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

impl From<RustBalanceAddressResponse> for BalanceAddressResponse {
    fn from(balance_for_address_response: RustBalanceAddressResponse) -> Self {
        BalanceAddressResponse {
            address_type: balance_for_address_response.address_type,
            address: balance_for_address_response.address,
            balance: balance_for_address_response.balance,
            dust_allowed: balance_for_address_response.dust_allowed,
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
            ledger_inclusion_state: message_metadata_response
                .ledger_inclusion_state
                .map(|state| state.into()),
            conflict_reason: message_metadata_response.conflict_reason,
            should_promote: message_metadata_response.should_promote,
            should_reattach: message_metadata_response.should_reattach,
        }
    }
}

impl From<RustNodeInfoWrapper> for NodeInfoWrapper {
    fn from(info: RustNodeInfoWrapper) -> Self {
        NodeInfoWrapper {
            url: info.url,
            nodeinfo: NodeInfo {
                name: info.nodeinfo.name,
                version: info.nodeinfo.version,
                is_healthy: info.nodeinfo.is_healthy,
                network_id: info.nodeinfo.network_id,
                bech32_hrp: info.nodeinfo.bech32_hrp,
                min_pow_score: info.nodeinfo.min_pow_score,
                messages_per_second: info.nodeinfo.messages_per_second,
                referenced_messages_per_second: info.nodeinfo.referenced_messages_per_second,
                referenced_rate: info.nodeinfo.referenced_rate,
                latest_milestone_timestamp: info.nodeinfo.latest_milestone_timestamp,
                latest_milestone_index: info.nodeinfo.latest_milestone_index,
                confirmed_milestone_index: info.nodeinfo.confirmed_milestone_index,
                pruning_index: info.nodeinfo.pruning_index,
                features: info.nodeinfo.features,
            },
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

impl From<RustUtxoChangesResponse> for MilestoneUTXOChanges {
    fn from(milestone_utxo_changes: RustUtxoChangesResponse) -> Self {
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
        let gossip = peer.gossip.map(GossipDto::from);
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
            RustRelationDto::Autopeered => Self {
                relation: "autopeered".to_string(),
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
                    if let RustInput::Utxo(input) = input {
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
                .map(|output| match output {
                    RustOutput::SignatureLockedSingle(output) => TransactionOutput {
                        signature_locked_single: Some(SignatureLockedSingleOutput {
                            address: unsafe { output.address().to_bech32(BECH32_HRP) },
                            amount: output.amount(),
                        }),
                        signature_locked_dust_allowance: None,
                    },
                    RustOutput::SignatureLockedDustAllowance(output) => TransactionOutput {
                        signature_locked_single: None,
                        signature_locked_dust_allowance: Some(SignatureLockedDustAllowanceOutput {
                            address: unsafe { output.address().to_bech32(BECH32_HRP) },
                            amount: output.amount(),
                        }),
                    },
                    _ => {
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
            index: *essence.index(),
            timestamp: essence.timestamp(),
            parents: vec![essence.parents().iter().map(|m| m.to_string()).collect()],
            merkle_proof: essence.merkle_proof().try_into()?,
            next_pow_score: essence.next_pow_score(),
            next_pow_score_milestone_index: essence.next_pow_score_milestone_index(),
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

impl TryFrom<RustPayload> for Payload {
    type Error = Error;
    fn try_from(payload: RustPayload) -> Result<Self> {
        let payload = match payload {
            RustPayload::Transaction(payload) => {
                let essence = match payload.essence().to_owned() {
                    RustEssence::Regular(e) => e.try_into()?,
                };

                Payload {
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
                }
            }
            RustPayload::Indexation(payload) => Payload {
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
            },
            RustPayload::Milestone(payload) => Payload {
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
            },
            RustPayload::Receipt(payload) => {
                let essence = match payload.transaction() {
                    RustPayload::Transaction(transaction_payload) => match transaction_payload.essence().to_owned() {
                        RustEssence::Regular(e) => e.try_into()?,
                    },
                    _ => panic!("Missing transaction payload"),
                };
                Payload {
                    transaction: None,
                    milestone: None,
                    indexation: None,
                    receipt: Some(vec![Receipt {
                        kind: 3,
                        index: *payload.migrated_at(),
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
                }
            }
            RustPayload::TreasuryTransaction(payload) => Payload {
                transaction: None,
                milestone: None,
                indexation: None,
                receipt: None,
                treasury_transaction: Some(vec![RustTreasuryTransactionPayloadDto::from(&(*payload)).into()]),
            },
        };
        Ok(payload)
    }
}

impl TryFrom<RustMessage> for Message {
    type Error = Error;
    fn try_from(msg: RustMessage) -> Result<Self> {
        Ok(Message {
            message_id: msg.id().0.to_string(),
            network_id: msg.network_id(),
            parents: msg.parents().iter().map(|m| m.to_string()).collect(),
            payload: if let Some(payload) = msg.payload().as_ref() {
                Some((*payload).clone().try_into()?)
            } else {
                None
            },
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
                RustUtxoInput::new(
                    RustTransactionId::from_str(&input.transaction_id[..]).unwrap_or_else(|_| {
                        panic!(
                            "invalid UtxoInput transaction_id: {} with input index {}",
                            input.transaction_id, input.index
                        )
                    }),
                    input.index,
                )
                .unwrap_or_else(|_| {
                    panic!(
                        "invalid UtxoInput transaction_id: {} with input index {}",
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
                let mut converted_output = None;
                if let Some(output) = &output.signature_locked_single {
                    converted_output = Some(
                        RustSignatureLockedSingleOutput::new(
                            RustAddress::try_from_bech32(&output.address[..]).unwrap_or_else(|_| {
                                panic!(
                                    "invalid SignatureLockedSingleOutput with output address: {}",
                                    output.address
                                )
                            }),
                            output.amount,
                        )
                        .unwrap_or_else(|_| {
                            panic!(
                                "invalid SignatureLockedSingleOutput with output address: {}",
                                output.address
                            )
                        })
                        .into(),
                    )
                }
                if let Some(output) = &output.signature_locked_dust_allowance {
                    converted_output = Some(
                        RustSignatureLockedDustAllowanceOutput::new(
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
                                "invalid SignatureLockedDustAllowanceOutput with output address: {}",
                                output.address
                            )
                        })
                        .into(),
                    )
                }
                converted_output.unwrap_or_else(|| panic!("Invalid output type"))
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
                    .unwrap_or_else(|| panic!("Invalid IndexationPayload: {indexation_payload:?}"))[0]
                    .index
                    .clone()
                    .as_bytes(),
                &(indexation_payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid IndexationPayload: {indexation_payload:?}"))[0]
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
        Ok(RustEd25519Signature::new(
            signature.public_key,
            signature
                .signature
                .clone()
                .try_into()
                .unwrap_or_else(|_| panic!("Invalid Signature: {:?}", signature.signature)),
        )
        .into())
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
            let unlock_blocks: Result<Vec<RustUnlockBlock>> =
                unlock_blocks.iter().cloned().map(|u| u.try_into()).collect();

            transaction = transaction.with_unlock_blocks(RustUnlockBlocks::new(unlock_blocks?)?);

            Ok(RustPayload::Transaction(Box::new(transaction.finish()?)))
        } else {
            let indexation = RustIndexationPayload::new(
                payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid Payload: {payload:?}"))[0]
                    .index
                    .clone()
                    .as_bytes(),
                &payload
                    .indexation
                    .as_ref()
                    .unwrap_or_else(|| panic!("Invalid Payload: {payload:?}"))[0]
                    .data,
            )?;
            Ok(RustPayload::Indexation(Box::new(indexation)))
        }
    }
}

impl From<RustReceiptDto> for ReceiptDto {
    fn from(receipt: RustReceiptDto) -> Self {
        Self {
            receipt: receipt.receipt.into(),
            milestone_index: receipt.milestone_index,
        }
    }
}

impl From<RustReceiptPayloadDto> for Receipt {
    fn from(receipt: RustReceiptPayloadDto) -> Self {
        let payload = match receipt.transaction {
            RustPayloadDto::TreasuryTransaction(payload) => *payload,
            _ => panic!("Invalid payload"),
        };
        Self {
            kind: receipt.kind,
            index: receipt.migrated_at,
            last: receipt.last,
            funds: receipt.funds.into_iter().map(|r| r.into()).collect(),
            transaction: Payload {
                transaction: None,
                milestone: None,
                indexation: None,
                receipt: None,
                treasury_transaction: Some(vec![payload.into()]),
            },
        }
    }
}

impl From<RustMigratedFundsEntryDto> for MigratedFundsEntry {
    fn from(receipt: RustMigratedFundsEntryDto) -> Self {
        Self {
            tail_transaction_hash: receipt.tail_transaction_hash,
            output: SignatureLockedSingleOutputDto {
                kind: 0,
                address: receipt.address.into(),
                amount: receipt.deposit,
            },
        }
    }
}

impl From<RustTreasuryTransactionPayloadDto> for TreasuryTransaction {
    fn from(treasury: RustTreasuryTransactionPayloadDto) -> Self {
        let treasury_input = match treasury.input {
            RustInputDto::Treasury(t) => TreasuryInput {
                kind: t.kind,
                message_id: t.milestone_id,
            },
            RustInputDto::Utxo(_) => panic!("Invalid type"),
        };
        Self {
            kind: treasury.kind,
            input: InputDto {
                utxo: None,
                treasury: Some(treasury_input),
            },
            output: treasury.output.into(),
        }
    }
}

#[derive(Debug, Clone, DeriveFromPyObject, DeriveIntoPyObject)]
/// The outputs query options.
pub struct AddressOutputsOptions {
    include_spent: bool,
    output_type: Option<String>,
}

impl From<AddressOutputsOptions> for RustAddressOutputsOptions {
    fn from(value: AddressOutputsOptions) -> Self {
        Self {
            include_spent: value.include_spent,
            output_type: value.output_type.map(|o| match o.as_str() {
                "SignatureLockedSingle" => OutputType::SignatureLockedSingle,
                "SignatureLockedDustAllowance" => OutputType::SignatureLockedDustAllowance,
                _ => panic!("unexpected output type option"),
            }),
        }
    }
}
