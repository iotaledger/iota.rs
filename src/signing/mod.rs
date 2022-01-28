// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Signing module to allow using different signer types for address generation and transaction essence signing

use bee_message::{
    address::Address,
    payload::transaction::{TransactionEssence, TransactionPayload},
    signature::Signature,
    unlock_block::{UnlockBlock, UnlockBlocks},
};

#[cfg(not(feature = "wasm"))]
use tokio::sync::Mutex;

#[cfg(feature = "wasm")]
use std::sync::Mutex;
use std::{
    fmt::{Debug, Formatter, Result},
    ops::{Deref, Range},
    path::Path,
    sync::Arc,
};

#[cfg(feature = "ledger")]
pub mod ledger;
/// Module for signing with a mnemonic or seed
pub mod mnemonic;
/// Signing related types
pub mod types;
pub use types::{GenerateAddressMetadata, LedgerStatus, Network, SignMessageMetadata, SignerType, TransactionInput};

/// SignerHandle, possible signers are mnemonic, Stronghold and Ledger
#[derive(Clone)]
pub struct SignerHandle {
    pub(crate) signer: Arc<Mutex<Box<dyn Signer + Sync + Send>>>,
    /// SignerType
    pub signer_type: SignerType,
}

impl Debug for SignerHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.signer_type)
    }
}

impl SignerHandle {
    /// Create a new SignerHandle
    pub fn new(signer_type: SignerType, signer: Box<dyn Signer + Sync + Send>) -> Self {
        Self {
            signer_type,
            signer: Arc::new(Mutex::new(signer)),
        }
    }
}

impl Deref for SignerHandle {
    type Target = Arc<Mutex<Box<dyn Signer + Sync + Send>>>;
    fn deref(&self) -> &Self::Target {
        &self.signer
    }
}

/// Signer interface.
#[async_trait::async_trait]
pub trait Signer {
    /// Get the ledger status.
    async fn get_ledger_status(&self, is_simulator: bool) -> LedgerStatus;
    /// Initialises a mnemonic.
    async fn store_mnemonic(&mut self, storage_path: &Path, mnemonic: String) -> crate::Result<()>;
    /// Generates an address.
    async fn generate_addresses(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        metadata: GenerateAddressMetadata,
    ) -> crate::Result<Vec<Address>>;
    /// Signs transaction essence.
    async fn sign_transaction_essence<'a>(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        coin_type: u32,
        account_index: u32,
        essence: &TransactionEssence,
        inputs: &mut Vec<TransactionInput>,
        metadata: SignMessageMetadata<'a>,
    ) -> crate::Result<Vec<UnlockBlock>>;
}

/// Verify unlock blocks of a transaction
pub fn verify_unlock_blocks(transaction_payload: &TransactionPayload, inputs: Vec<Address>) -> crate::Result<()> {
    let essence_hash = transaction_payload.essence().hash();
    let unlock_blocks = transaction_payload.unlock_blocks();
    for (index, address) in inputs.iter().enumerate() {
        verify_signature(address, unlock_blocks, index, &essence_hash)?;
    }
    Ok(())
}

fn verify_signature(
    address: &Address,
    unlock_blocks: &UnlockBlocks,
    index: usize,
    essence_hash: &[u8; 32],
) -> crate::Result<()> {
    let signature_unlock_block = match unlock_blocks.get(index) {
        Some(unlock_block) => match unlock_block {
            UnlockBlock::Signature(b) => b,
            UnlockBlock::Reference(b) => match unlock_blocks.get(b.index().into()) {
                Some(UnlockBlock::Signature(unlock_block)) => unlock_block,
                _ => return Err(crate::Error::MissingUnlockBlock),
            },
            UnlockBlock::Alias(b) => match unlock_blocks.get(b.index().into()) {
                Some(UnlockBlock::Signature(unlock_block)) => unlock_block,
                _ => return Err(crate::Error::MissingUnlockBlock),
            },
            UnlockBlock::Nft(b) => match unlock_blocks.get(b.index().into()) {
                Some(UnlockBlock::Signature(unlock_block)) => unlock_block,
                _ => return Err(crate::Error::MissingUnlockBlock),
            },
        },
        None => return Err(crate::Error::MissingUnlockBlock),
    };
    match address {
        Address::Ed25519(address) => {
            let Signature::Ed25519(ed25519_signature) = signature_unlock_block.signature();
            address.verify(essence_hash, ed25519_signature)?
        }
        // todo handle other addresses
        Address::Alias(_address) => {}
        Address::Nft(_address) => {}
    };

    Ok(())
}
