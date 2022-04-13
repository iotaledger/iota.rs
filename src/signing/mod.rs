// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Signing module to allow using different signer types for address generation and transaction essence signing

#[cfg(feature = "ledger")]
use self::ledger::LedgerSigner;
#[cfg(feature = "stronghold")]
use self::stronghold::StrongholdSigner;
use crate::signing::{
    mnemonic::MnemonicSigner,
    types::{InputSigningData, SignerTypeDto},
};

use bee_message::{
    address::{Address, AliasAddress, Ed25519Address, NftAddress},
    output::Output,
    payload::transaction::TransactionEssence,
    unlock_block::{AliasUnlockBlock, NftUnlockBlock, ReferenceUnlockBlock, UnlockBlock},
};

#[cfg(feature = "wasm")]
use std::sync::Mutex;
#[cfg(not(feature = "wasm"))]
use tokio::sync::Mutex;

use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
    ops::{Deref, Range},
    sync::Arc,
};

#[cfg(feature = "stronghold")]
use std::path::PathBuf;

#[cfg(feature = "ledger")]
pub mod ledger;
/// Module for signing with a mnemonic or seed
pub mod mnemonic;
/// Module for signing with a Stronghold vault
#[cfg(feature = "stronghold")]
pub mod stronghold;
/// Signing related types
pub mod types;

pub use types::{GenerateAddressMetadata, LedgerStatus, Network, SignMessageMetadata, SignerType};

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
    /// Create a new SignerHandle from a serialized SignerTypeDto
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(data: &str) -> crate::Result<Self> {
        let signer_type: SignerTypeDto = serde_json::from_str(data)?;

        Ok(match signer_type {
            #[cfg(feature = "stronghold")]
            SignerTypeDto::Stronghold(stronghold_dto) => {
                let mut builder = StrongholdSigner::builder();

                if let Some(password) = &stronghold_dto.password {
                    builder = builder.password(password);
                }

                if let Some(snapshot_path) = &stronghold_dto.snapshot_path {
                    builder = builder.snapshot_path(PathBuf::from(snapshot_path));
                }

                builder.build().into()
            }

            #[cfg(feature = "ledger")]
            SignerTypeDto::LedgerNano => LedgerSigner::new(false),

            #[cfg(feature = "ledger")]
            SignerTypeDto::LedgerNanoSimulator => LedgerSigner::new(true),

            SignerTypeDto::Mnemonic(mnemonic) => MnemonicSigner::new(&mnemonic)?,
        })
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
pub trait Signer: Send + Sync {
    /// Get the ledger status.
    ///
    /// This is only meaningful for the Ledger hardware; other signers don't implement this.
    async fn get_ledger_status(&self, _: bool) -> LedgerStatus {
        LedgerStatus {
            app: None,
            connected: false,
            locked: false,
        }
    }

    /// Initialises a mnemonic.
    ///
    /// This is only meaningful for the Stronghold signer; other signers don't implement this.
    async fn store_mnemonic(&mut self, _: String) -> crate::Result<()> {
        Err(crate::Error::NoMnemonicWasStored)
    }

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

    /// Sign on `essence`, unlock `input` by returning an [UnlockBlock].
    async fn signature_unlock<'a>(
        &mut self,
        _input: &InputSigningData,
        _essence_hash: &[u8; 32],
        _metadata: &SignMessageMetadata<'a>,
    ) -> crate::Result<UnlockBlock> {
        // Return error unless implemented otherwise.
        Err(crate::Error::NoMnemonicWasStored)
    }

    /// Signs transaction essence.
    ///
    /// Signers usually don't implement this, as the default implementation has taken care of the placement of blocks
    /// (e.g. references between them). [Signer::signature_unlock()] will be invoked every time a necessary signing
    /// action needs to be performed.
    async fn sign_transaction_essence<'a>(
        &mut self,
        essence: &TransactionEssence,
        inputs: &mut Vec<InputSigningData>,
        metadata: SignMessageMetadata<'a>,
    ) -> crate::Result<Vec<UnlockBlock>> {
        // The hashed_essence gets signed
        let hashed_essence = essence.hash();
        let mut unlock_blocks = Vec::new();
        let mut unlock_block_indexes = HashMap::<Address, usize>::new();

        for (current_block_index, input) in inputs.iter().enumerate() {
            // Get the address that is required to unlock the input
            let (_, input_address) = Address::try_from_bech32(&input.bech32_address)?;

            // Check if we already added an [UnlockBlock] for this address
            match unlock_block_indexes.get(&input_address) {
                // If we already have an [UnlockBlock] for this address, add a [UnlockBlock] based on the address type
                Some(block_index) => match input_address {
                    Address::Alias(_alias) => {
                        unlock_blocks.push(UnlockBlock::Alias(AliasUnlockBlock::new(*block_index as u16)?))
                    }
                    Address::Ed25519(_ed25519) => {
                        unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlockBlock::new(*block_index as u16)?))
                    }
                    Address::Nft(_nft) => {
                        unlock_blocks.push(UnlockBlock::Nft(NftUnlockBlock::new(*block_index as u16)?))
                    }
                },
                None => {
                    // We can only sign ed25519 addresses and unlock_block_indexes needs to contain the alias or nft
                    // address already at this point, because the reference index needs to be lower
                    // than the current block index
                    if input_address.kind() != Ed25519Address::KIND {
                        return Err(crate::Error::MissingInputWithEd25519UnlockCondition);
                    }

                    let unlock_block = self.signature_unlock(input, &hashed_essence, &metadata).await?;
                    unlock_blocks.push(unlock_block);

                    // Add the ed25519 address to the unlock_block_indexes, so it gets referenced if further inputs have
                    // the same address in their unlock condition
                    unlock_block_indexes.insert(input_address, current_block_index);
                }
            }

            // When we have an alias or Nft output, we will add their alias or nft address to unlock_block_indexes,
            // because they can be used to unlock outputs via [UnlockBlock::Alias] or [UnlockBlock::Nft],
            // that have the corresponding alias or nft address in their unlock condition
            let output = Output::try_from(&input.output_response.output)?;
            match &output {
                Output::Alias(alias_output) => unlock_block_indexes.insert(
                    Address::Alias(AliasAddress::new(
                        alias_output.alias_id().or_from_output_id(input.output_id()?),
                    )),
                    current_block_index,
                ),
                Output::Nft(nft_output) => unlock_block_indexes.insert(
                    Address::Nft(NftAddress::new(
                        nft_output.nft_id().or_from_output_id(input.output_id()?),
                    )),
                    current_block_index,
                ),
                _ => None,
            };
        }
        Ok(unlock_blocks)
    }
}
