// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;

use anyhow::anyhow;

use iota_client::bee_message::{
    signature::{Ed25519Signature, SignatureUnlock as RustSignatureUnlock},
    unlock::{
        ReferenceUnlock as RustReferenceUnlock, UnlockBlock as RustUnlockBlock, UnlockBlocks as RustUnlockBlocks,
    },
};
use std::{
    convert::TryInto,
    fmt::{Display, Formatter},
};

pub enum UnlockBlockKind {
    Reference = 0,
    Ed25519 = 1,
}

#[derive(Clone, Debug)]
pub struct UnlockBlock(RustUnlockBlock);

impl UnlockBlock {
    pub fn kind(&self) -> UnlockBlockKind {
        match self.0 {
            RustUnlockBlock::Signature(_) => UnlockBlockKind::Ed25519,
            RustUnlockBlock::Reference(_) => UnlockBlockKind::Reference,
        }
    }

    pub fn to_inner(&self) -> RustUnlockBlock {
        self.0.clone()
    }

    pub fn as_signature(&self) -> Result<SignatureUnlock> {
        if let RustUnlockBlock::Signature(unlock) = &self.0 {
            Ok((unlock.clone()).into())
        } else {
            Err(anyhow::anyhow!("UnlockBlock is not of type Signature"))
        }
    }
    pub fn as_reference(&self) -> Result<ReferenceUnlock> {
        if let RustUnlockBlock::Reference(unlock) = &self.0 {
            Ok((unlock.clone()).into())
        } else {
            Err(anyhow::anyhow!("UnlockBlock is not of type Reference"))
        }
    }
}
impl Display for UnlockBlock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}
impl From<RustUnlockBlock> for UnlockBlock {
    fn from(unlock: RustUnlockBlock) -> Self {
        Self(unlock)
    }
}

#[derive(Clone, Debug)]
pub struct UnlockBlocks(RustUnlockBlocks);

impl UnlockBlocks {
    pub fn from(unlock_blocks: Vec<UnlockBlock>) -> Result<Self> {
        match RustUnlockBlocks::new(unlock_blocks.iter().map(|b| b.to_inner()).collect()) {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(u) => Ok(UnlockBlocks(u)),
        }
    }

    pub fn get(&self, index: usize) -> Option<UnlockBlock> {
        self.0.get(index).map(|u| UnlockBlock(u.clone()))
    }

    pub fn to_inner(&self) -> RustUnlockBlocks {
        self.0.clone()
    }
}

impl Display for UnlockBlocks {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

pub struct ReferenceUnlock(RustReferenceUnlock);

impl ReferenceUnlock {
    /// Creates a new `ReferenceUnlock`.
    pub fn from(index: u16) -> Result<ReferenceUnlock> {
        match RustReferenceUnlock::new(index) {
            Ok(e) => Ok(Self(e)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    /// Return the index of a `ReferenceUnlock`.
    pub fn index(&self) -> u16 {
        self.0.index()
    }
}

impl Display for ReferenceUnlock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}
impl From<RustReferenceUnlock> for ReferenceUnlock {
    fn from(unlock: RustReferenceUnlock) -> Self {
        Self(unlock)
    }
}

pub struct SignatureUnlock(RustSignatureUnlock);

impl SignatureUnlock {
    pub fn from(public_key: Vec<u8>, signature: Vec<u8>) -> SignatureUnlock {
        Self(RustSignatureUnlock::Ed25519(Ed25519Signature::new(
            (*public_key).try_into().unwrap(),
            (*signature).try_into().unwrap(),
        )))
    }

    pub fn to_inner_ref(&self) -> &RustSignatureUnlock {
        &self.0
    }
}

impl Display for SignatureUnlock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}
impl From<RustSignatureUnlock> for SignatureUnlock {
    fn from(unlock: RustSignatureUnlock) -> Self {
        Self(unlock)
    }
}
