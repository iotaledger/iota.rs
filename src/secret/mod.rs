// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Secret manager module enabling address generation and transaction essence signing.

#[cfg(feature = "ledger")]
pub mod ledger;
/// Module for signing with a mnemonic or seed
pub mod mnemonic;
/// Module for signing with a Stronghold vault
#[cfg(feature = "stronghold")]
pub mod stronghold;
/// Signing related types
pub mod types;

#[cfg(feature = "stronghold")]
use std::path::PathBuf;
use std::{collections::HashMap, ops::Range, str::FromStr};

use async_trait::async_trait;
use bee_message::{
    address::{Address, AliasAddress, Ed25519Address, NftAddress},
    output::Output,
    payload::transaction::TransactionEssence,
    unlock_block::{AliasUnlockBlock, NftUnlockBlock, ReferenceUnlockBlock, UnlockBlock},
};
pub use types::{GenerateAddressMetadata, LedgerStatus, Network, SignMessageMetadata};

#[cfg(feature = "ledger")]
use self::ledger::LedgerSecretManager;
use self::mnemonic::MnemonicSecretManager;
#[cfg(feature = "stronghold")]
use self::stronghold::StrongholdSecretManager;
use crate::secret::types::InputSigningData;
#[cfg(feature = "stronghold")]
use crate::secret::types::StrongholdDto;

/// The secret manager interface.
#[async_trait]
pub trait SecretManage: Send + Sync {
    /// Generates addresses.
    ///
    /// For `coin_type`, see also <https://github.com/satoshilabs/slips/blob/master/slip-0044.md>.
    async fn generate_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        metadata: GenerateAddressMetadata,
    ) -> crate::Result<Vec<Address>>;

    /// Sign on `essence`, unlock `input` by returning an [UnlockBlock].
    async fn signature_unlock<'a>(
        &self,
        input: &InputSigningData,
        essence_hash: &[u8; 32],
        metadata: &SignMessageMetadata<'a>,
    ) -> crate::Result<UnlockBlock>;
}

/// An extension to [`SecretManager`].
///
/// This trait is automatically implemented for any type that implements [`SecretManager`] - it contains methods for
/// internal use that are based on the methods in [`SecretManager`]. Secret managers don't implement this on their
/// sides.
#[async_trait]
pub trait SecretManageExt {
    /// Signs transaction essence.
    ///
    /// Secret managers usually don't implement this, as the default implementation has taken care of the placement of
    /// blocks (e.g. references between them). [SecretManager::signature_unlock()] will be invoked every time a
    /// necessary signing action needs to be performed.
    async fn sign_transaction_essence<'a>(
        &self,
        essence: &TransactionEssence,
        inputs: &[InputSigningData],
        metadata: SignMessageMetadata<'a>,
    ) -> crate::Result<Vec<UnlockBlock>>;
}

#[async_trait]
impl<S> SecretManageExt for S
where
    S: SecretManage,
{
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
            match &input.output {
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

/// Supported secret managers

// Boxes make this type clumsy to use.
#[allow(clippy::large_enum_variant)]

pub enum SecretManager {
    /// Secret manager that uses [`iota_stronghold`] as the backing storage.
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Stronghold(StrongholdSecretManager),

    /// Secret manager that uses a Ledger hardware wallet.
    #[cfg(feature = "ledger")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger")))]
    LedgerNano(LedgerSecretManager),

    /// Secret manager that uses a Ledger Speculos simulator.
    #[cfg(feature = "ledger")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger")))]
    LedgerNanoSimulator(LedgerSecretManager),

    /// Secret manager that uses only a mnemonic.
    Mnemonic(MnemonicSecretManager),
}

impl std::fmt::Debug for SecretManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(_) => f.debug_tuple("Stronghold").field(&"...").finish(),
            #[cfg(feature = "ledger")]
            Self::LedgerNano(_) => f.debug_tuple("LedgerNano").field(&"...").finish(),
            #[cfg(feature = "ledger")]
            Self::LedgerNanoSimulator(_) => f.debug_tuple("LedgerNanoSimulator").field(&"...").finish(),
            Self::Mnemonic(_) => f.debug_tuple("Mnemonic").field(&"...").finish(),
        }
    }
}

impl FromStr for SecretManager {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        SecretManager::try_from(&serde_json::from_str::<SecretManagerDto>(s)?)
    }
}

/// DTO for secret manager types with required data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretManagerDto {
    /// Stronghold
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Stronghold(StrongholdDto),
    /// Ledger Device
    #[cfg(feature = "ledger")]
    LedgerNano,
    /// Ledger Speculos Simulator
    #[cfg(feature = "ledger")]
    LedgerNanoSimulator,
    /// Mnemonic
    Mnemonic(String),
}

impl TryFrom<&SecretManagerDto> for SecretManager {
    type Error = crate::Error;
    fn try_from(value: &SecretManagerDto) -> crate::Result<Self> {
        Ok(match value {
            #[cfg(feature = "stronghold")]
            SecretManagerDto::Stronghold(stronghold_dto) => {
                let mut builder = StrongholdSecretManager::builder();

                if let Some(password) = &stronghold_dto.password {
                    builder = builder.password(password);
                }

                if let Some(snapshot_path) = &stronghold_dto.snapshot_path {
                    builder = builder.snapshot_path(PathBuf::from(snapshot_path));
                }

                Self::Stronghold(builder.build())
            }

            #[cfg(feature = "ledger")]
            SecretManagerDto::LedgerNano => Self::LedgerNano(LedgerSecretManager::new(false)),

            #[cfg(feature = "ledger")]
            SecretManagerDto::LedgerNanoSimulator => Self::LedgerNanoSimulator(LedgerSecretManager::new(true)),

            SecretManagerDto::Mnemonic(mnemonic) => Self::Mnemonic(MnemonicSecretManager::try_from_mnemonic(mnemonic)?),
        })
    }
}

impl From<&SecretManager> for SecretManagerDto {
    fn from(value: &SecretManager) -> Self {
        match value {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(stronghold_dto) => Self::Stronghold(StrongholdDto {
                password: None,
                snapshot_path: stronghold_dto
                    .snapshot_path
                    .as_ref()
                    .map(|s| s.clone().into_os_string().to_string_lossy().into()),
            }),

            #[cfg(feature = "ledger")]
            SecretManager::LedgerNano(_) => Self::LedgerNano,

            #[cfg(feature = "ledger")]
            SecretManager::LedgerNanoSimulator(_) => Self::LedgerNanoSimulator,

            // `MnemonicSecretManager(Seed)` doesn't have Debug or Display implemented and in the current use cases of
            // the client/wallet we also don't need to convert it in this direction with the mnemonic/seed, we only need
            // to know the type
            SecretManager::Mnemonic(_mnemonic) => Self::Mnemonic("...".to_string()),
        }
    }
}

#[async_trait]
impl SecretManage for SecretManager {
    async fn generate_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        metadata: GenerateAddressMetadata,
    ) -> crate::Result<Vec<Address>> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(secret_manager) => {
                secret_manager
                    .generate_addresses(coin_type, account_index, address_indexes, internal, metadata)
                    .await
            }
            #[cfg(feature = "ledger")]
            SecretManager::LedgerNano(secret_manager) => {
                secret_manager
                    .generate_addresses(coin_type, account_index, address_indexes, internal, metadata)
                    .await
            }
            #[cfg(feature = "ledger")]
            SecretManager::LedgerNanoSimulator(secret_manager) => {
                secret_manager
                    .generate_addresses(coin_type, account_index, address_indexes, internal, metadata)
                    .await
            }
            SecretManager::Mnemonic(secret_manager) => {
                secret_manager
                    .generate_addresses(coin_type, account_index, address_indexes, internal, metadata)
                    .await
            }
        }
    }

    async fn signature_unlock<'a>(
        &self,
        input: &InputSigningData,
        essence_hash: &[u8; 32],
        metadata: &SignMessageMetadata<'a>,
    ) -> crate::Result<UnlockBlock> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
            #[cfg(feature = "ledger")]
            SecretManager::LedgerNano(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
            #[cfg(feature = "ledger")]
            SecretManager::LedgerNanoSimulator(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
            SecretManager::Mnemonic(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
        }
    }
}
