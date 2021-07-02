// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Serialize, Deserialize};

use iota_client::bee_message::payload::Payload as RustPayload;

use crate::{
    Result,
    bee_types::{IndexationPayload, MilestonePayload, ReceiptPayload, TransactionPayload, TreasuryPayload},
};

pub enum MessagePayloadType {
    Transaction = 1,
    Milestone = 2,
    Indexation = 3,
    Receipt = 4,
    TreasuryTransaction = 5,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MessagePayload {
    payload: RustPayload,
}

impl From<RustPayload> for MessagePayload {
    fn from(payload: RustPayload) -> Self {
        Self { payload }
    }
}

impl MessagePayload {
    pub fn deserialize(serialised_data: &str) -> Result<MessagePayload> {
        let res = serde_json::from_str(&serialised_data);
        
        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.payload)
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(self);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn to_inner(self) -> RustPayload {
        self.payload
    }

    pub fn payload_type(&self) -> MessagePayloadType {
        match self.payload {
            RustPayload::Transaction(_) => MessagePayloadType::Transaction,
            RustPayload::Milestone(_) => MessagePayloadType::Milestone,
            RustPayload::Indexation(_) => MessagePayloadType::Indexation,
            RustPayload::Receipt(_) => MessagePayloadType::Receipt,
            RustPayload::TreasuryTransaction(_) => MessagePayloadType::TreasuryTransaction,
        }
    }

    pub fn get_as_indexation(&self) -> Option<IndexationPayload> {
        if let RustPayload::Indexation(index) = &self.payload {
            match IndexationPayload::new(index.index(), index.data()) {
                Ok(i) => Some(i),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn get_as_transaction(&self) -> Option<TransactionPayload> {
        if let RustPayload::Transaction(payload) = &self.payload {
            Some((*payload.clone()).into())
        } else {
            None
        }
    }

    pub fn get_as_treasury(&self) -> Option<TreasuryPayload> {
        if let RustPayload::TreasuryTransaction(payload) = &self.payload {
            Some((*payload.clone()).into())
        } else {
            None
        }
    }

    pub fn get_as_milestone(&self) -> Option<MilestonePayload> {
        if let RustPayload::Milestone(payload) = &self.payload {
            Some(MilestonePayload::new(
                payload.essence().to_owned(),
                payload.signatures().to_owned(),
            ))
        } else {
            None
        }
    }

    pub fn get_as_receipt(&self) -> Option<ReceiptPayload> {
        if let RustPayload::Receipt(payload) = &self.payload {
            Some((*payload.clone()).into())
        } else {
            None
        }
    }
}
