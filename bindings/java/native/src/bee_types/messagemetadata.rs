// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use iota_client::bee_rest_api::types::{
    dtos::LedgerInclusionStateDto, responses::MessageMetadataResponse as RustMessageMetadata,
};

#[derive(Clone, Getters, CopyGetters, PartialEq)]
pub struct MessageMetadata {
    #[getset(get = "pub")]
    pub message_id: String,
    pub parent_message_ids: Vec<String>,
    #[getset(get_copy = "pub")]
    pub is_solid: bool,
    #[getset(get_copy = "pub")]
    pub referenced_by_milestone_index: Option<u32>,
    #[getset(get_copy = "pub")]
    pub milestone_index: Option<u32>,
    pub ledger_inclusion_state: Option<LedgerInclusionStateDto>,
    #[getset(get_copy = "pub")]
    pub conflict_reason: Option<u8>,
    #[getset(get_copy = "pub")]
    pub should_promote: Option<bool>,
    #[getset(get_copy = "pub")]
    pub should_reattach: Option<bool>,
}

impl core::fmt::Display for MessageMetadata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "message_id={}, parent_message_ids=({:?}), is_solid={}, referenced_by_milestone_index={:?}, milestone_index={:?}, ledger_inclusion_state={:?}, conflict_reason={:?}, should_promote={:?}, should_reattach={:?}",
            self.message_id,
            self.parent_message_ids,
            self.is_solid,
            self.referenced_by_milestone_index,
            self.milestone_index,
            self.ledger_inclusion_state,
            self.conflict_reason,
            self.should_promote,
            self.should_reattach
        )
    }
}

impl core::fmt::Debug for MessageMetadata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "MessageMetadata({self})")
    }
}

impl From<RustMessageMetadata> for MessageMetadata {
    fn from(info: RustMessageMetadata) -> Self {
        Self {
            message_id: info.message_id,
            is_solid: info.is_solid,
            parent_message_ids: info.parent_message_ids,
            referenced_by_milestone_index: info.referenced_by_milestone_index,
            milestone_index: info.milestone_index,
            ledger_inclusion_state: info.ledger_inclusion_state,
            conflict_reason: info.conflict_reason,
            should_promote: info.should_promote,
            should_reattach: info.should_reattach,
        }
    }
}

impl MessageMetadata {
    pub fn parent_message_ids(&self) -> Vec<String> {
        self.parent_message_ids.clone()
    }

    pub fn ledger_inclusion_state(&self) -> Option<LedgerInclusionStateDto> {
        self.ledger_inclusion_state.clone()
    }
}
