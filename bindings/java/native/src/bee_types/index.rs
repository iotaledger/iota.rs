// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use crate::Result;
use anyhow::anyhow;

use iota_client::bee_message::prelude::TaggedPayload as RustTaggedPayload;

pub struct TaggedPayload {
    payload: RustTaggedPayload,
}

impl TaggedPayload {
    pub fn to_inner(self) -> RustTaggedPayload {
        self.payload
    }

    pub fn new(index: &[u8], data: &[u8]) -> Result<TaggedPayload> {
        let index = RustTaggedPayload::new(&index, &data);
        match index {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(i) => Ok(TaggedPayload { payload: i }),
        }
    }

    pub fn new_from_string(index: &str, data: &str) -> Result<TaggedPayload> {
        let index = RustTaggedPayload::new(index.as_bytes(), data.as_bytes());
        match index {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(i) => Ok(TaggedPayload { payload: i }),
        }
    }

    pub fn index(&self) -> &[u8] {
        self.payload.index()
    }

    pub fn data(&self) -> &[u8] {
        self.payload.data()
    }
}

impl core::fmt::Display for TaggedPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "payload={:?}", self.payload)
    }
}

impl core::fmt::Debug for TaggedPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "TaggedPayload({})", self)
    }
}
