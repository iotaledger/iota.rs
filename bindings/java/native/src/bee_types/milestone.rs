// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_rest_api::types::responses::UtxoChangesResponse as RustUtxoChangesResponse;
use getset::{CopyGetters, Getters};
use iota_client::{
    MilestoneResponse as RustMilestoneResponse,
    bee_message::MessageId,
};

#[derive(Getters, CopyGetters)]
pub struct MilestoneResponse {
    #[getset(get_copy = "pub")]
    pub index: u32,
    #[getset(get_copy = "pub")]
    pub message_id: MessageId,
    #[getset(get_copy = "pub")]
    pub timestamp: u64,
}

impl From<RustMilestoneResponse> for MilestoneResponse {
    fn from(milestone: RustMilestoneResponse) -> Self {
        Self {
            index: milestone.index.clone(),
            message_id: milestone.message_id.clone(),
            timestamp: milestone.timestamp,
        }
    }
}

#[derive(Getters, CopyGetters)]
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

impl From<RustUtxoChangesResponse> for MilestoneUtxoChangesResponse {
    fn from(changes: RustUtxoChangesResponse) -> Self {
        Self {
            index: changes.index.clone(),
            created_outputs: changes.created_outputs.clone(),
            consumed_outputs: changes.consumed_outputs.clone(),
        }
    }
}

// pub struct MilestonePayload {
// essence: MilestonePayloadEssenceRust,
// signatures: Vec<Box<[u8]>>,
// }
//
// impl MilestonePayload {
// pub fn new(essence: MilestonePayloadEssenceRust, signatures: Vec<Box<[u8]>>) -> MilestonePayload {
// MilestonePayload { essence, signatures }
// }
//
// pub fn essence(&self) -> MilestonePayloadEssence {
// MilestonePayloadEssence {
// essence: self.essence.clone(),
// }
// }
//
// pub fn signatures(&self) -> Vec<MilestoneSignature> {
// Vec of vec, or vec of box isnt implemented as a generatable type
// self.signatures
// .clone()
// .iter()
// .map(|signature| MilestoneSignature {
// signature: (*signature).to_vec(),
// })
// .collect()
// }
// }
//
// pub struct MilestoneSignature {
// signature: Vec<u8>,
// }
//
// impl MilestoneSignature {
// pub fn get_signature(&self) -> Vec<u8> {
// self.signature.clone()
// }
// }
//
// pub struct MilestonePayloadEssence {
// essence: MilestonePayloadEssenceRust,
// }
//
// impl MilestonePayloadEssence {
// pub fn index(&self) -> u32 {
// self.essence.index()
// }
//
// pub fn timestamp(&self) -> u64 {
// self.essence.timestamp()
// }
//
// pub fn parents(&self) -> Vec<MessageId> {
// self.essence.parents().iter().map(|e| e.clone()).collect()
// }
//
// pub fn merkle_proof(&self) -> Vec<u8> {
// self.essence.merkle_proof().to_vec()
// }
//
// pub fn next_pow_score(&self) -> u32 {
// self.essence.next_pow_score()
// }
//
// pub fn next_pow_score_milestone(&self) -> u32 {
// self.essence.next_pow_score_milestone_index()
// }
//
// pub fn public_keys(&self) -> Vec<PublicKey> {
// Vec of vec isnt implemented as a generatable type
// self.essence
// .public_keys()
// .iter()
// .map(|key| PublicKey {
// public_key: key.to_vec(),
// })
// .collect()
// }
//
// pub fn receipt(&self) -> Option<ReceiptPayload> {
// let option = self.essence.receipt();
// if let Some(payload) = option {
// if let MessagePayloadRust::Receipt(receipt) = payload {
// return Some((*receipt.clone()).into());
// }
// }
//
// None
// }
// }
//
// pub struct PublicKey {
// public_key: Vec<u8>,
// }
//
// impl PublicKey {
// pub fn get_public_key(&self) -> Vec<u8> {
// self.public_key.clone()
// }
// }
