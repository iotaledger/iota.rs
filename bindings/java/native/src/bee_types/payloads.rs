// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

use iota_client::bee_message::payload::Payload as RustPayload;

use crate::{
    bee_types::{IndexationPayload, MilestonePayload, ReceiptPayload, TransactionPayload, TreasuryPayload},
    Result,
};

pub enum MessagePayloadType {
    Transaction = 1,
    Milestone = 2,
    Indexation = 3,
    Receipt = 4,
    TreasuryTransaction = 5,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MessagePayload {
    payload: RustPayload,
}

impl From<RustPayload> for MessagePayload {
    fn from(payload: RustPayload) -> Self {
        Self { payload }
    }
}

impl Display for MessagePayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self.payload)
    }
}

impl MessagePayload {
    pub fn deserialize(serialised_data: &str) -> Result<MessagePayload> {
        let res = serde_json::from_str(serialised_data);

        match res {
            Ok(s) => Ok(MessagePayload { payload: s }),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(&self.payload);

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

    pub fn as_transaction(&self) -> Result<TransactionPayload> {
        if let RustPayload::Transaction(payload) = &self.payload {
            Ok((*payload.clone()).into())
        } else {
            Err(anyhow::anyhow!("Message is not of type Transaction"))
        }
    }

    pub fn as_indexation(&self) -> Result<IndexationPayload> {
        if let RustPayload::Indexation(index) = &self.payload {
            IndexationPayload::new(index.index(), index.data())
        } else {
            Err(anyhow::anyhow!("Message is not of type Indexation"))
        }
    }

    pub fn as_milestone(&self) -> Result<MilestonePayload> {
        if let RustPayload::Milestone(payload) = &self.payload {
            Ok(MilestonePayload::new(
                payload.essence().to_owned(),
                payload.signatures().to_owned(),
            ))
        } else {
            Err(anyhow::anyhow!("Message is not of type Milestone"))
        }
    }

    pub fn as_receipt(&self) -> Result<ReceiptPayload> {
        if let RustPayload::Receipt(payload) = &self.payload {
            Ok((*payload.clone()).into())
        } else {
            Err(anyhow::anyhow!("Message is not of type Receipt"))
        }
    }

    pub fn as_treasury(&self) -> Result<TreasuryPayload> {
        if let RustPayload::TreasuryTransaction(payload) = &self.payload {
            Ok((*payload.clone()).into())
        } else {
            Err(anyhow::anyhow!("Message is not of type Treasury"))
        }
    }
}
