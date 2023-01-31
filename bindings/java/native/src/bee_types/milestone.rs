// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use iota_client::{
    bee_message::{
        payload::{
            milestone::{
                MilestonePayload as RustMilestonePayload, MilestonePayloadEssence as RustMilestonePayloadEssence,
                MILESTONE_SIGNATURE_LENGTH,
            },
            Payload as RustPayload,
        },
        MessageId,
    },
    bee_rest_api::types::responses::UtxoChangesResponse as RustUtxoChangesResponse,
    MilestoneResponse as RustMilestoneResponse,
};

use crate::{bee_types::ReceiptPayload, ed25519::PublicKey, Result};

use std::convert::TryInto;

#[derive(Getters, CopyGetters, Eq, PartialEq, Debug)]
pub struct MilestoneResponse {
    #[getset(get_copy = "pub")]
    pub index: u32,
    #[getset(get_copy = "pub")]
    pub message_id: MessageId,
    #[getset(get_copy = "pub")]
    pub timestamp: u64,
}

impl core::fmt::Display for MilestoneResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} {} {}", self.index, self.message_id, self.timestamp)
    }
}

impl From<RustMilestoneResponse> for MilestoneResponse {
    fn from(milestone: RustMilestoneResponse) -> Self {
        Self {
            index: milestone.index,
            message_id: milestone.message_id,
            timestamp: milestone.timestamp,
        }
    }
}

#[derive(Getters, CopyGetters, Eq, PartialEq, Debug)]
pub struct MilestoneUtxoChangesResponse {
    #[getset(get_copy = "pub")]
    pub index: u32,
    pub created_outputs: Vec<String>,
    pub consumed_outputs: Vec<String>,
}

impl MilestoneUtxoChangesResponse {
    pub fn created_outputs(&self) -> Vec<String> {
        self.created_outputs.clone()
    }
    pub fn consumed_outputs(&self) -> Vec<String> {
        self.consumed_outputs.clone()
    }
}

impl core::fmt::Display for MilestoneUtxoChangesResponse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{} {:?} {:?}",
            self.index, self.created_outputs, self.consumed_outputs
        )
    }
}

impl From<RustUtxoChangesResponse> for MilestoneUtxoChangesResponse {
    fn from(changes: RustUtxoChangesResponse) -> Self {
        Self {
            index: changes.index,
            created_outputs: changes.created_outputs.clone(),
            consumed_outputs: changes.consumed_outputs,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct MilestonePayload {
    rust_milestone: RustMilestonePayload,
    essence: RustMilestonePayloadEssence,
    signatures: Vec<[u8; MILESTONE_SIGNATURE_LENGTH]>,
}

impl From<RustMilestonePayload> for MilestonePayload {
    fn from(payload: RustMilestonePayload) -> Self {
        Self::new(payload.essence().to_owned(), payload.signatures().to_owned())
    }
}

impl MilestonePayload {
    pub fn new(essence: RustMilestonePayloadEssence, box_signatures: Vec<Box<[u8]>>) -> MilestonePayload {
        let signatures: Vec<[u8; MILESTONE_SIGNATURE_LENGTH]> =
            box_signatures.iter().map(|s| s.to_vec().try_into().unwrap()).collect();
        MilestonePayload {
            rust_milestone: RustMilestonePayload::new(essence.clone(), signatures.clone()).unwrap(),
            essence,
            signatures,
        }
    }
    pub fn to_inner(self) -> RustMilestonePayload {
        RustMilestonePayload::new(self.essence.clone(), self.signatures).unwrap()
    }

    pub fn essence(&self) -> MilestonePayloadEssence {
        MilestonePayloadEssence {
            essence: self.essence.clone(),
        }
    }

    pub fn signatures(&self) -> Vec<MilestoneSignature> {
        // Vec of vec, or vec of box isnt implemented as a generatable type
        self.signatures
            .clone()
            .iter()
            .map(|signature| MilestoneSignature { signature: *signature })
            .collect()
    }

    pub fn validate(&self, applicable_public_keys: Vec<String>, min_threshold: usize) -> Result<()> {
        match self.rust_milestone.validate(&applicable_public_keys, min_threshold) {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow::anyhow!(format!("{e:?}"))),
        }
    }

    pub fn id(&self) -> String {
        self.rust_milestone.id().to_string()
    }

    pub fn deserialize(serialised_data: &str) -> Result<Self> {
        let res: Result<RustMilestonePayload, _> = serde_json::from_str(serialised_data);

        match res {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(&self.rust_milestone);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
}

impl core::fmt::Display for MilestonePayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "essence={:?} signatures=({:?})", self.essence, self.signatures)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct MilestoneSignature {
    signature: [u8; MILESTONE_SIGNATURE_LENGTH],
}

impl MilestoneSignature {
    pub fn get_signature(&self) -> Vec<u8> {
        self.signature.clone().to_vec()
    }
}

impl core::fmt::Display for MilestoneSignature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.signature)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct MilestonePayloadEssence {
    essence: RustMilestonePayloadEssence,
}

impl MilestonePayloadEssence {
    pub fn index(&self) -> u32 {
        *self.essence.index()
    }

    pub fn timestamp(&self) -> u64 {
        self.essence.timestamp()
    }

    pub fn parents(&self) -> Vec<MessageId> {
        self.essence.parents().iter().copied().collect()
    }

    pub fn merkle_proof(&self) -> Vec<u8> {
        self.essence.merkle_proof().to_vec()
    }

    pub fn next_pow_score(&self) -> u32 {
        self.essence.next_pow_score()
    }

    pub fn next_pow_score_milestone(&self) -> u32 {
        self.essence.next_pow_score_milestone_index()
    }

    pub fn public_keys(&self) -> Vec<PublicKey> {
        self.essence
            .public_keys()
            .iter()
            .map(|key| key.try_into().unwrap())
            .collect()
    }

    pub fn receipt(&self) -> Option<ReceiptPayload> {
        let option = self.essence.receipt();
        if let Some(RustPayload::Receipt(receipt)) = option {
            return Some((*receipt.clone()).into());
        }

        None
    }

    pub fn hash(&self) -> Vec<u8> {
        self.essence.hash().to_vec()
    }
}

impl core::fmt::Display for MilestonePayloadEssence {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.essence)
    }
}
