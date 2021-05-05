// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
/*
use iota_wallet::message::MessagePayload as MessagePayloadRust;

use crate::types::{index::*, milestone::*, receipt::*, transaction::*, treasury::*};

pub enum MessagePayloadType {
    Transaction = 1,
    Milestone = 2,
    Indexation = 3,
    Receipt = 4,
    TreasuryTransaction = 5,
}

pub struct MessagePayload {
    payload: MessagePayloadRust,
}

impl From<MessagePayloadRust> for MessagePayload {
    fn from(payload: MessagePayloadRust) -> Self {
        Self { payload }
    }
}

impl MessagePayload {
    pub fn to_inner(self) -> MessagePayloadRust {
        self.payload
    }

    pub fn payload_type(&self) -> MessagePayloadType {
        match self.payload {
            MessagePayloadRust::Transaction(_) => MessagePayloadType::Transaction,
            MessagePayloadRust::Milestone(_) => MessagePayloadType::Milestone,
            MessagePayloadRust::Indexation(_) => MessagePayloadType::Indexation,
            MessagePayloadRust::Receipt(_) => MessagePayloadType::Receipt,
            MessagePayloadRust::TreasuryTransaction(_) => MessagePayloadType::TreasuryTransaction,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.payload)
    }

    pub fn get_as_transaction(&self) -> Option<MessageTransactionPayload> {
        if let MessagePayloadRust::Transaction(payload) = &self.payload {
            Some(payload.into())
        } else {
            None
        }
    }

    pub fn get_as_indexation(&self) -> Option<IndexationPayload> {
        if let MessagePayloadRust::Indexation(index) = &self.payload {
            match IndexationPayload::new(index.index(), index.data()) {
                Ok(i) => Some(i),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn get_as_milestone(&self) -> Option<MilestonePayload> {
        if let MessagePayloadRust::Milestone(payload) = &self.payload {
            Some(MilestonePayload::new(
                payload.essence().to_owned(),
                payload.signatures().to_owned(),
            ))
        } else {
            None
        }
    }

    pub fn get_as_receipt(&self) -> Option<ReceiptPayload> {
        if let MessagePayloadRust::Receipt(payload) = &self.payload {
            Some((*payload.clone()).into())
        } else {
            None
        }
    }

    pub fn get_as_treasury(&self) -> Option<TreasuryTransactionPayload> {
        if let MessagePayloadRust::TreasuryTransaction(payload) = &self.payload {
            Some((*payload.clone()).into())
        } else {
            None
        }
    }
}
*/