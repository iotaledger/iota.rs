//! Types of several IOTA APIs related objects
use crate::Result;

use bee_message::{
    payload::milestone::{MilestoneEssence, MILESTONE_MERKLE_PROOF_LENGTH},
    prelude::*,
};
use serde::ser::Serializer;

use std::{
    convert::{From, TryFrom, TryInto},
    io::{BufReader, Read},
};

/// Marker trait for response
pub trait ResponseType {}

impl ResponseType for Message {}

/// Response from the Iota node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T: ResponseType> {
    pub(crate) data: T,
}

impl<T: ResponseType> Response<T> {
    /// Get data of the response.
    pub fn data(&self) -> &T {
        &self.data
    }
}

/// Response of GET /api/v1/info endpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Iota node Name
    pub name: String,
    /// Iota node version
    pub version: String,
    /// Connection status
    #[serde(rename = "isHealthy")]
    pub is_healthy: bool,
    /// coordinator public key
    #[serde(rename = "networkId")]
    pub network_id: String,
    /// latest milestone index
    #[serde(rename = "latestMilestoneIndex")]
    pub latest_milestone_index: usize,
    /// solid milestone index
    #[serde(rename = "solidMilestoneIndex")]
    pub solid_milestone_index: usize,
    /// pruning index
    #[serde(rename = "pruningIndex")]
    pub pruning_index: usize,
    /// features
    pub features: Vec<String>,
}

impl ResponseType for NodeInfo {}

/// Response of GET /api/v1/tips endpoint
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Tips {
    /// Message ID of tip 1
    #[serde(rename = "tip1MessageId")]
    pub(crate) tip1: String,
    /// Message ID of tip 2
    #[serde(rename = "tip2MessageId")]
    pub(crate) tip2: String,
}

impl ResponseType for Tips {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PostMessageId {
    #[serde(rename = "messageId")]
    pub(crate) message_id: String,
}

impl ResponseType for PostMessageId {}

/// Collection of meesage ID
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct MessageIds {
    #[serde(rename = "messageIds")]
    pub(crate) inner: Box<[String]>,
}

impl ResponseType for MessageIds {}

/// Response of GET /api/v1/messages/{messageId} endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// Message ID
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Message ID of parent1
    #[serde(rename = "parent1MessageId")]
    pub parent1: String,
    /// Message ID of parent2
    #[serde(rename = "parent2MessageId")]
    pub parent2: String,
    /// Solid status
    #[serde(rename = "isSolid")]
    pub is_solid: bool,
    /// Should promote
    #[serde(rename = "shouldPromote")]
    pub should_promote: Option<bool>,
    /// Should reattach
    #[serde(rename = "shouldReattach")]
    pub should_reattach: Option<bool>,
    /// Referenced by milestone index
    #[serde(rename = "referencedByMilestoneIndex")]
    pub referenced_by_milestone_index: Option<u64>,
    /// Ledger inclusion state
    #[serde(rename = "ledgerInclusionState")]
    pub ledger_inclusion_state: Option<String>,
}

impl ResponseType for MessageMetadata {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChildrenMessageIds {
    #[serde(rename = "childrenMessageIds")]
    pub(crate) inner: Box<[String]>,
}

impl ResponseType for ChildrenMessageIds {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AddressBalance {
    pub(crate) count: usize,
    pub(crate) balance: u64,
}

impl ResponseType for AddressBalance {}

/// Output raw data
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RawOutput {
    #[serde(rename = "messageId")]
    pub(crate) message_id: String,
    #[serde(rename = "transactionId")]
    pub(crate) transaction_id: String,
    #[serde(rename = "outputIndex")]
    pub(crate) output_index: u16,
    #[serde(rename = "isSpent")]
    pub(crate) is_spent: bool,
    pub(crate) output: SLS,
}

impl ResponseType for RawOutput {}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SLS {
    #[serde(rename = "type")]
    pub(crate) type_: u8,
    pub(crate) address: SLSAddress,
    pub(crate) amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SLSAddress {
    #[serde(rename = "type")]
    pub(crate) type_: u8,
    pub(crate) address: String,
}

fn serialize_as_hex<S>(x: &[u8], s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(hex::encode(x).as_str())
}

/// Output data
#[derive(Debug, Serialize)]
pub struct OutputMetadata {
    /// Message ID of the output
    #[serde(rename = "messageId", serialize_with = "serialize_as_hex")]
    pub message_id: Vec<u8>,
    /// Transaction ID of the output
    #[serde(rename = "transactionId", serialize_with = "serialize_as_hex")]
    pub transaction_id: Vec<u8>,
    /// Output index.
    #[serde(rename = "outputIndex")]
    pub output_index: u16,
    /// Spend status of the output
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    /// Corresponding address
    pub address: Address,
    /// Balance amount
    pub amount: u64,
}

/// Outputs that use a given address.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressOutputs {
    /// Outputs used by the address.
    #[serde(rename = "outputIds")]
    pub output_ids: Box<[String]>,
}

impl ResponseType for AddressOutputs {}

/// Milestone from Iota node
#[derive(Debug, Serialize, Deserialize)]
pub struct MilestoneMetadata {
    /// Milestone index
    #[serde(rename = "milestoneIndex")]
    pub milestone_index: u64,
    /// Milestone ID
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Timestamp
    pub timestamp: u64,
}

impl ResponseType for MilestoneMetadata {}

/// Address and the coresponding balance returned by the get_address_balances() API.
#[derive(Debug, Serialize)]
pub struct AddressBalancePair {
    /// Address
    pub address: Address,
    /// Balance in the address
    pub balance: u64,
}

/// Transfers structure
///
/// Users could use this to construct output address with amount of iota they want to get.
#[derive(Debug)]
pub struct Transfers(pub Vec<(Address, u64)>);

impl Transfers {
    /// Create Transfers starting with one address
    pub fn new(address: Address, amount: u64) -> Self {
        Self(vec![(address, amount)])
    }

    /// Add more address to the Transfers
    pub fn add(&mut self, address: Address, amount: u64) {
        self.0.push((address, amount));
    }
}

/// JSON struct for Message
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageJson {
    #[serde(rename = "parent1MessageId")]
    parent1: String,
    #[serde(rename = "parent2MessageId")]
    parent2: String,
    payload: PayloadJson,
    nonce: String,
}

impl ResponseType for MessageJson {}

impl From<&Message> for MessageJson {
    fn from(i: &Message) -> Self {
        Self {
            parent1: i.parent1().to_string(),
            parent2: i.parent2().to_string(),
            payload: i.payload().as_ref().unwrap().into(),
            nonce: i.nonce().to_string(),
        }
    }
}

impl TryFrom<MessageJson> for Message {
    type Error = crate::Error;

    fn try_from(value: MessageJson) -> Result<Self> {
        let mut parent1 = [0u8; 32];
        hex::decode_to_slice(value.parent1, &mut parent1)?;
        let mut parent2 = [0u8; 32];
        hex::decode_to_slice(value.parent2, &mut parent2)?;
        let nonce = value.nonce;
        let parent1 = MessageId::new(parent1);
        let parent2 = MessageId::new(parent2);
        Ok(Message::builder()
            // TODO: make the newtwork id configurable
            .with_network_id(0)
            .with_parent1(parent1)
            .with_parent2(parent2)
            .with_payload(get_payload_from_json(
                value.payload,
                Some((parent1, parent2)),
            )?)
            .with_nonce(
                nonce
                    .parse()
                    .map_err(|_| crate::Error::InvalidParameter(format!("nonce {}", nonce)))?,
            )
            .finish()?)
    }
}

/// The JSON representation of the transaction payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPayloadJson {
    #[serde(rename = "type")]
    type_: u8,
    essence: TransactionEssenceJson,
    #[serde(rename = "unlockBlocks")]
    unlock_blocks: Box<[UnlockBlockJson]>,
}

/// The JSON representation of the milestone payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct MilestonePayloadJson {
    #[serde(rename = "type")]
    type_: u8,
    index: u32,
    #[serde(rename = "inclusionMerkleProof")]
    inclusion_merkle_proof: String,
    signatures: Vec<String>,
    timestamp: u64,
}

/// The JSON representation of the indexation payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexationPayloadJson {
    #[serde(rename = "type")]
    type_: u8,
    index: String,
    data: String,
}

/// Each of the possible payload types.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PayloadJson {
    /// The transaction payload.
    Transaction(TransactionPayloadJson),
    /// The indexation payload.
    Indexation(IndexationPayloadJson),
    /// The milestone payload.
    Milestone(MilestonePayloadJson),
}

impl From<&Payload> for PayloadJson {
    fn from(i: &Payload) -> Self {
        match i {
            Payload::Transaction(i) => Self::Transaction(TransactionPayloadJson {
                type_: 0,
                essence: (i.essence()).into(),
                unlock_blocks: i.unlock_blocks().iter().map(|input| input.into()).collect(),
            }),
            Payload::Indexation(i) => Self::Indexation(IndexationPayloadJson {
                type_: 2,
                index: i.index().to_string(),
                data: hex::encode(i.data()),
            }),
            Payload::Milestone(m) => Self::Milestone(MilestonePayloadJson {
                type_: 1,
                index: m.essence().index(),
                inclusion_merkle_proof: hex::encode(m.essence().merkle_proof()),
                signatures: m.signatures().iter().map(hex::encode).collect(),
                timestamp: m.essence().timestamp(),
            }),
            _ => unimplemented!(),
        }
    }
}

fn get_payload_from_json(
    payload: PayloadJson,
    tips: Option<(MessageId, MessageId)>,
) -> Result<Payload> {
    match payload {
        PayloadJson::Transaction(transaction_payload) => {
            let mut transaction = Transaction::builder();
            transaction = transaction.with_essence(transaction_payload.essence.try_into()?);

            let unlock_blocks = transaction_payload.unlock_blocks.into_vec();
            for unlock_block in unlock_blocks {
                transaction = transaction.add_unlock_block(unlock_block.try_into()?);
            }

            Ok(Payload::Transaction(Box::new(transaction.finish()?)))
        }
        PayloadJson::Indexation(indexation_payload) => {
            let indexation = Indexation::new(
                indexation_payload.index,
                &hex::decode(indexation_payload.data)?,
            )
            .unwrap();
            Ok(Payload::Indexation(Box::new(indexation)))
        }
        PayloadJson::Milestone(milestone_payload) => {
            let merkle_proof = hex::decode(milestone_payload.inclusion_merkle_proof)?;
            let mut reader = BufReader::new(&merkle_proof[..]);
            let mut merkle_proof = [0u8; MILESTONE_MERKLE_PROOF_LENGTH];
            reader.read_exact(&mut merkle_proof)?;
            let milestone_essence = MilestoneEssence::new(
                milestone_payload.index,
                milestone_payload.timestamp,
                tips.unwrap().0,
                tips.unwrap().1,
                merkle_proof,
                vec![],
            );

            let mut signatures: Vec<Box<[u8]>> = vec![];
            for signature in milestone_payload.signatures {
                let signature = hex::decode(signature)?;
                let mut reader = BufReader::new(&signature[..]);
                let mut signature = [0; 64];
                reader.read_exact(&mut signature)?;
                signatures.push(Box::new(signature));
            }
            let milestone = Milestone::new(milestone_essence, signatures);
            Ok(Payload::Milestone(Box::new(milestone)))
        }
    }
}

/// JSON struct for TransactionEssence
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionEssenceJson {
    #[serde(rename = "type")]
    type_: u8,
    inputs: Box<[InputJson]>,
    outputs: Box<[OutputJson]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Box<PayloadJson>>,
}

impl From<&TransactionEssence> for TransactionEssenceJson {
    fn from(i: &TransactionEssence) -> Self {
        let indexation_payload = match i.payload().as_ref() {
            Some(r) => Some(Box::new(PayloadJson::from(r))),
            _ => None,
        };
        Self {
            type_: 0,
            inputs: i.inputs().iter().map(|input| input.into()).collect(),
            outputs: i.outputs().iter().map(|input| input.into()).collect(),
            payload: indexation_payload,
        }
    }
}

impl TryFrom<TransactionEssenceJson> for TransactionEssence {
    type Error = crate::Error;

    fn try_from(value: TransactionEssenceJson) -> Result<Self> {
        let mut builder = TransactionEssence::builder();

        let inputs: Vec<Input> = value
            .inputs
            .into_vec()
            .into_iter()
            .map(|input| input.try_into())
            .filter_map(|i| i.ok())
            .collect();
        for input in inputs {
            builder = builder.add_input(input);
        }

        let outputs: Vec<Output> = value
            .outputs
            .into_vec()
            .into_iter()
            .map(|output| output.try_into())
            .filter_map(|i| i.ok())
            .collect();
        for output in outputs {
            builder = builder.add_output(output);
        }

        builder = match value.payload {
            Some(indexation) => builder.with_payload(
                get_payload_from_json(*indexation, None)
                    .expect("Invalid indexation in TransactionEssenceJson"),
            ),
            _ => builder,
        };

        Ok(builder.finish()?)
    }
}

/// JSON struct for Input
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputJson {
    #[serde(rename = "type")]
    type_: u8,
    #[serde(rename = "transactionId")]
    transaction_id: String,
    #[serde(rename = "transactionOutputIndex")]
    transaction_output_index: u16,
}

impl From<&Input> for InputJson {
    fn from(i: &Input) -> Self {
        match i {
            Input::UTXO(i) => Self {
                type_: 0,
                transaction_id: i.output_id().to_string()[..64].to_string(),
                transaction_output_index: i.output_id().index(),
            },
            _ => todo!(),
        }
    }
}

impl TryFrom<InputJson> for Input {
    type Error = crate::Error;

    fn try_from(value: InputJson) -> Result<Self> {
        let mut id = [0u8; 32];
        hex::decode_to_slice(value.transaction_id, &mut id)?;
        let input = UTXOInput::new(TransactionId::from(id), value.transaction_output_index)?;
        Ok(input.into())
    }
}

/// JSON struct for Output
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputJson {
    #[serde(rename = "type")]
    type_: u8,
    address: AddressJson,
    amount: u64,
}

impl From<&Output> for OutputJson {
    fn from(i: &Output) -> Self {
        match i {
            Output::SignatureLockedSingle(s) => Self {
                type_: 0,
                address: s.address().into(),
                amount: s.amount().get(),
            },
            _ => todo!(),
        }
    }
}

impl TryFrom<OutputJson> for Output {
    type Error = crate::Error;

    fn try_from(value: OutputJson) -> Result<Self> {
        let output = SignatureLockedSingleOutput::new(
            value.address.try_into()?,
            value
                .amount
                .try_into()
                .expect("Output amount cannot be zero."),
        );
        Ok(output.into())
    }
}

/// JSON struct for Address
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressJson {
    #[serde(rename = "type")]
    type_: u8,
    address: String,
}

impl From<&Address> for AddressJson {
    fn from(i: &Address) -> Self {
        match i {
            Address::Ed25519(a) => Self {
                type_: 1,
                address: a.to_string(),
            },
            _ => panic!("This library doesn't support WOTS."),
        }
    }
}

impl TryFrom<AddressJson> for Address {
    type Error = crate::Error;

    fn try_from(value: AddressJson) -> Result<Self> {
        match value.type_ {
            1 => {
                let mut address = [0u8; 32];
                hex::decode_to_slice(value.address, &mut address)?;
                let address = Ed25519Address::from(address);
                Ok(address.into())
            }
            _ => unreachable!(),
        }
    }
}

/// JSON struct for UnlockBlock
#[derive(Debug, Serialize, Deserialize)]
pub struct UnlockBlockJson {
    #[serde(rename = "type")]
    type_: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<SignatureUnlockJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<u16>,
}

impl From<&UnlockBlock> for UnlockBlockJson {
    fn from(i: &UnlockBlock) -> Self {
        match i {
            UnlockBlock::Signature(s) => Self {
                type_: 0,
                signature: Some(s.into()),
                reference: None,
            },
            UnlockBlock::Reference(s) => Self {
                type_: 1,
                signature: None,
                reference: Some(s.index()),
            },
            _ => todo!(),
        }
    }
}

impl TryFrom<UnlockBlockJson> for UnlockBlock {
    type Error = crate::Error;

    fn try_from(value: UnlockBlockJson) -> Result<Self> {
        match value.type_ {
            0 => {
                let sig: SignatureUnlock = value
                    .signature
                    .expect("Must contain signature.")
                    .try_into()?;
                Ok(sig.into())
            }
            1 => {
                let reference: ReferenceUnlock = value
                    .reference
                    .expect("Must contain reference.")
                    .try_into()?;
                Ok(reference.into())
            }
            _ => unreachable!(),
        }
    }
}

/// JSON struct for SignatureUnlock
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureUnlockJson {
    #[serde(rename = "type")]
    type_: u8,
    #[serde(rename = "publicKey")]
    publickey: String,
    signature: String,
}

impl From<&SignatureUnlock> for SignatureUnlockJson {
    fn from(i: &SignatureUnlock) -> Self {
        match i {
            SignatureUnlock::Ed25519(a) => Self {
                type_: 1,
                publickey: hex::encode(a.public_key()),
                signature: hex::encode(a.signature()),
            },
            _ => panic!("This library doesn't support WOTS."),
        }
    }
}

impl TryFrom<SignatureUnlockJson> for SignatureUnlock {
    type Error = crate::Error;

    fn try_from(value: SignatureUnlockJson) -> Result<Self> {
        let mut public_key = [0u8; 32];
        hex::decode_to_slice(value.publickey, &mut public_key)?;
        let signature = hex::decode(value.signature)?.into_boxed_slice();
        Ok(Ed25519Signature::new(public_key, signature).into())
    }
}
