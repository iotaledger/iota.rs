// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Signing module to allow using different signer types for address generation and transaction essence signing

#[cfg(feature = "ledger")]
pub mod ledger;
/// Module for signing with a mnemonic or seed
pub mod mnemonic;
/// Module for signing with a Stronghold vault
#[cfg(feature = "stronghold")]
pub mod stronghold;
/// Signing related types
pub mod types;

#[cfg(feature = "ledger")]
pub use self::ledger::LedgerSigner;
#[cfg(feature = "stronghold")]
pub use self::stronghold::StrongholdSigner;
pub use self::{
    mnemonic::MnemonicSigner,
    types::{GenerateAddressMetadata, LedgerStatus, Network, SignMessageMetadata, SignerType},
};

use self::types::{InputSigningData, SignerTypeDto};
use async_trait::async_trait;
use bee_message::{
    address::{Address, AliasAddress, Ed25519Address, NftAddress},
    output::Output,
    payload::transaction::{TransactionEssence, TransactionPayload},
    signature::Signature,
    unlock_block::{AliasUnlockBlock, NftUnlockBlock, ReferenceUnlockBlock, UnlockBlock, UnlockBlocks},
};
#[cfg(feature = "stronghold")]
use std::path::PathBuf;
use std::{collections::HashMap, ops::Range, str::FromStr};

/// The interface for a signer capable to perform various cryptographic operations.
#[async_trait]
pub trait Signer: Send + Sync {
    /// Type of the signer.
    ///
    /// The current design still needs to distinguish between signers behind a trait object. To achieve this, signers
    /// need to provide their own type via this method.
    ///
    /// Providing signers that aren't listed in [`SignerType`] aren't currently supported.
    async fn signer_type(&self) -> SignerType;

    /// Initialize the signer.
    ///
    /// This should be called after the signer has been created to allow the signer to initialize itself (because not
    /// all tasks can be done upon `struct` construction).
    ///
    /// When `mnemonic` is supplied, the signer is prompted to store or use it. However, the exact behavior of signer
    /// is not defined. For example, [`StrongholdSigner`] will return an error if a mnemonic has been initialized, but
    /// [`MnemonicSigner`] will happily replace the mnemonic it stores instead.
    async fn signer_init(&mut self, mnemonic: Option<&str>) -> crate::Result<()>;

    /// Synchronize the signer state.
    ///
    /// This method may be called from time to time. The signer can do some chores here. For example, synchronizing the
    /// state in the memory to the disk.
    async fn signer_sync(&mut self) -> crate::Result<()>;

    /// Provide a password for the signer to perform any cryptographic action.
    ///
    /// The signer then stores and caches the password. Use [`Signer::signer_clear_password()`] to clear the cache.
    async fn signer_set_password(&mut self, password: &str);

    /// Purge the cached password from the memory.
    ///
    /// This is preferably be done using the [zeroize] crate.
    async fn signer_clear_password(&mut self);

    /// Generate addresses.
    ///
    /// For `coin_type`, see also <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>.
    async fn signer_gen_addrs(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        metadata: GenerateAddressMetadata,
    ) -> crate::Result<Vec<Address>>;

    /// Sign on `essence_hash`, unlock `input` by returning an [`UnlockBlock`].
    async fn signer_unlock<'a>(
        &self,
        input: &InputSigningData,
        essence_hash: &[u8; 32],
        metadata: &SignMessageMetadata<'a>,
    ) -> crate::Result<UnlockBlock>;

    /// Signs transaction essence.
    ///
    /// Signers usually don't implement this, as the default implementation has taken care of the placement of blocks
    /// (e.g. references between them). [Signer::signer_unlock()] will be invoked every time a necessary signing action
    /// needs to be performed.
    async fn sign_transaction_essence<'a>(
        &self,
        essence: &TransactionEssence,
        inputs: &[InputSigningData],
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

                    let unlock_block = self.signer_unlock(input, &hashed_essence, &metadata).await?;
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

    /// Get the status of a Ledger hardware.
    ///
    /// This is only meaningful for [`LedgerSigner`]; other signers don't implement this.
    async fn get_ledger_status(&self, _is_simulator: bool) -> LedgerStatus {
        LedgerStatus {
            app: None,
            connected: false,
            locked: false,
        }
    }
}

impl FromStr for Box<dyn Signer> {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let signer_type: SignerTypeDto = serde_json::from_str(s)?;

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

                Box::new(builder.build())
            }

            #[cfg(feature = "ledger")]
            SignerTypeDto::LedgerNano => Box::new(LedgerSigner::new(false)),

            #[cfg(feature = "ledger")]
            SignerTypeDto::LedgerNanoSimulator => Box::new(LedgerSigner::new(true)),

            SignerTypeDto::Mnemonic(mnemonic) => Box::new(MnemonicSigner::try_from_mnemonic(&mnemonic)?),
        })
    }
}

// todo use validation function from bee-ledger if possible
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
            ed25519_signature.is_valid(essence_hash, address)?
        }
        // todo handle other addresses
        Address::Alias(_address) => {}
        Address::Nft(_address) => {}
    };

    Ok(())
}
