// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Secret manager module enabling address generation and transaction essence signing.

#[cfg(feature = "ledger_nano")]
pub mod ledger_nano;
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
    unlock_block::{AliasUnlockBlock, NftUnlockBlock, ReferenceUnlockBlock, UnlockBlock},
};
pub use types::{GenerateAddressMetadata, LedgerStatus};

#[cfg(feature = "ledger_nano")]
use self::ledger_nano::LedgerSecretManager;
use self::mnemonic::MnemonicSecretManager;
#[cfg(feature = "stronghold")]
use self::stronghold::StrongholdSecretManager;
#[cfg(feature = "stronghold")]
use crate::secret::types::StrongholdDto;
use crate::{
    api::{PreparedTransactionData, RemainderData},
    secret::types::InputSigningData,
};

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
    async fn signature_unlock(
        &self,
        input: &InputSigningData,
        essence_hash: &[u8; 32],
        remainder: &Option<RemainderData>,
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
    async fn sign_transaction_essence(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
    ) -> crate::Result<Vec<UnlockBlock>>;
}

// Shared implementation for MnemonicSecretManager and StrongholdSecretManager
pub(crate) async fn default_sign_transaction_essence<'a>(
    secret_manager: &dyn SecretManage,
    prepared_transaction_data: &PreparedTransactionData,
) -> crate::Result<Vec<UnlockBlock>> {
    // The hashed_essence gets signed
    let hashed_essence = prepared_transaction_data.essence.hash();
    let mut unlock_blocks = Vec::new();
    let mut unlock_block_indexes = HashMap::<Address, usize>::new();

    for (current_block_index, input) in prepared_transaction_data.inputs_data.iter().enumerate() {
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
                Address::Nft(_nft) => unlock_blocks.push(UnlockBlock::Nft(NftUnlockBlock::new(*block_index as u16)?)),
            },
            None => {
                // We can only sign ed25519 addresses and unlock_block_indexes needs to contain the alias or nft
                // address already at this point, because the reference index needs to be lower
                // than the current block index
                if input_address.kind() != Ed25519Address::KIND {
                    return Err(crate::Error::MissingInputWithEd25519UnlockCondition);
                }

                let unlock_block = secret_manager
                    .signature_unlock(input, &hashed_essence, &prepared_transaction_data.remainder)
                    .await?;
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

/// Supported secret managers

// Boxes make this type clumsy to use.
#[allow(clippy::large_enum_variant)]

pub enum SecretManager {
    /// Secret manager that uses [`iota_stronghold`] as the backing storage.
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    Stronghold(StrongholdSecretManager),

    /// Secret manager that uses a Ledger hardware wallet.
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNano(LedgerSecretManager),

    /// Secret manager that uses a Ledger Speculos simulator.
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    LedgerNanoSimulator(LedgerSecretManager),

    /// Secret manager that uses only a mnemonic.
    Mnemonic(MnemonicSecretManager),
}

impl std::fmt::Debug for SecretManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "stronghold")]
            Self::Stronghold(_) => f.debug_tuple("Stronghold").field(&"...").finish(),
            #[cfg(feature = "ledger_nano")]
            Self::LedgerNano(_) => f.debug_tuple("LedgerNano").field(&"...").finish(),
            #[cfg(feature = "ledger_nano")]
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
    #[cfg(feature = "ledger_nano")]
    LedgerNano,
    /// Ledger Speculos Simulator
    #[cfg(feature = "ledger_nano")]
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

            #[cfg(feature = "ledger_nano")]
            SecretManagerDto::LedgerNano => Self::LedgerNano(LedgerSecretManager::new(false)),

            #[cfg(feature = "ledger_nano")]
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

            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(_) => Self::LedgerNano,

            #[cfg(feature = "ledger_nano")]
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
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(secret_manager) => {
                secret_manager
                    .generate_addresses(coin_type, account_index, address_indexes, internal, metadata)
                    .await
            }
            #[cfg(feature = "ledger_nano")]
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

    async fn signature_unlock(
        &self,
        input: &InputSigningData,
        essence_hash: &[u8; 32],
        metadata: &Option<RemainderData>,
    ) -> crate::Result<UnlockBlock> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNanoSimulator(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
            SecretManager::Mnemonic(secret_manager) => {
                secret_manager.signature_unlock(input, essence_hash, metadata).await
            }
        }
    }
}

#[async_trait]
impl SecretManageExt for SecretManager {
    async fn sign_transaction_essence(
        &self,
        prepared_transaction_data: &PreparedTransactionData,
    ) -> crate::Result<Vec<UnlockBlock>> {
        match self {
            #[cfg(feature = "stronghold")]
            SecretManager::Stronghold(secret_manager) => {
                secret_manager.sign_transaction_essence(prepared_transaction_data).await
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNano(secret_manager) => {
                secret_manager.sign_transaction_essence(prepared_transaction_data).await
            }
            #[cfg(feature = "ledger_nano")]
            SecretManager::LedgerNanoSimulator(secret_manager) => {
                secret_manager.sign_transaction_essence(prepared_transaction_data).await
            }
            SecretManager::Mnemonic(secret_manager) => {
                secret_manager.sign_transaction_essence(prepared_transaction_data).await
            }
        }
    }
}
