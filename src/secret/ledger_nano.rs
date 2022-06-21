// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`LedgerSecretManager`].
//!
//! Ledger status codes: <https://github.com/iotaledger/ledger-iota-app/blob/53c1f96d15f8b014ba8ba31a85f0401bb4d33e18/src/iota_io.h#L54>.

use std::{collections::HashMap, ops::Range};

use async_trait::async_trait;
use bee_block::{
    address::{Address, AliasAddress, NftAddress},
    output::Output,
    signature::Signature,
    unlock::{AliasUnlock, NftUnlock, ReferenceUnlock, Unlock, Unlocks},
};
use iota_ledger::{api::packable::Packable, LedgerBIP32Index};
use packable::PackableExt;
use tokio::sync::Mutex;

use super::{types::InputSigningData, GenerateAddressMetadata, SecretManage, SecretManageExt};
use crate::secret::{LedgerStatus, PreparedTransactionData, RemainderData};

/// Hardened const for the bip path.
///
/// See also: <https://wiki.trezor.io/Hardened_and_non-hardened_derivation>.
pub const HARDENED: u32 = 0x80000000;

/// Secret manager that uses a Ledger hardware wallet.
#[derive(Default)]
pub struct LedgerSecretManager {
    /// Specifies if a real Ledger hardware is used or only a simulator is used.
    pub is_simulator: bool,

    /// Mutex to prevent multiple simultaneous requests to a ledger.
    pub mutex: Mutex<()>,
}

// /// A record matching an Input with its address.
// #[derive(Debug)]
// struct AddressIndexRecorder {
//     /// the input
//     pub(crate) input: bee_block::input::Input,

//     /// bip32 index
//     pub(crate) bip32: LedgerBIP32Index,
// }

#[async_trait]
impl SecretManage for LedgerSecretManager {
    async fn generate_addresses(
        &self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        // current ledger app only supports IOTA_COIN_TYPE, SHIMMER_COIN_TYPE and TESTNET_COIN_TYPE
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        meta: GenerateAddressMetadata,
    ) -> crate::Result<Vec<Address>> {
        // lock the mutex to prevent multiple simultaneous requests to a ledger
        let _lock = self.mutex.lock().await;

        let bip32_account = account_index | HARDENED;

        let bip32 = iota_ledger::LedgerBIP32Index {
            bip32_index: address_indexes.start | HARDENED,
            bip32_change: if internal { 1 } else { 0 } | HARDENED,
        };
        // get ledger
        let ledger = iota_ledger::get_ledger(coin_type, bip32_account, self.is_simulator)?;

        // if it's not for syncing, then it's a new receiving / remainder address
        // that needs shown to the user
        let addresses = if !meta.syncing {
            // and generate a single address that is shown to the user
            ledger.get_addresses(true, bip32, address_indexes.len())?
        } else {
            ledger.get_addresses(false, bip32, address_indexes.len())?
        };

        let mut ed25519_addresses = Vec::new();
        for address in addresses {
            ed25519_addresses.push(bee_block::address::Address::Ed25519(
                bee_block::address::Ed25519Address::new(address),
            ));
        }
        Ok(ed25519_addresses)
    }

    // Ledger Nano will use `sign_transaction_essence`
    async fn signature_unlock(
        &self,
        _input: &InputSigningData,
        _essence_hash: &[u8; 32],
        _metadata: &Option<RemainderData>,
    ) -> crate::Result<Unlock> {
        panic!("signature_unlock is not supported with ledger")
    }
}

/// needs_blindsigning
/// the Ledger Nano S(+)/X app can present the user a detailed view of the transaction before it
/// is signed but only with BasicOutputs, without extra-features and if the Essence is not too large.
/// If criteria are not met, blindsigning is needed.
/// This method finds out if we have to switch to blindsigning mode.
pub fn needs_blindsigning(prepared_transaction: &PreparedTransactionData, buffer_size: usize) -> bool {
    match &prepared_transaction.essence {
        bee_block::payload::transaction::TransactionEssence::Regular(essence) => {
            for output in essence.outputs().iter() {
                // only basic outputs allowed
                if let bee_block::output::Output::Basic(s) = output {
                    // no native tokens
                    // only one address unlock
                    // no features
                    if let ([], [bee_block::output::UnlockCondition::Address(_)], []) = (
                        s.native_tokens().as_ref(),
                        s.unlock_conditions().as_ref(),
                        s.features().as_ref(),
                    ) {
                        // all fine, continue with next output
                        continue;
                    }
                }
                // not fine, return
                return true;
            }
        }
    }
    // check if essence + bip32 indices fit into the buffer of the device
    let essence_bytes = prepared_transaction.essence.pack_to_vec();
    let total_size =
        LedgerBIP32Index::default().packed_len() * prepared_transaction.inputs_data.len() + essence_bytes.len();

    // return true if too large
    total_size > buffer_size
}

#[async_trait]
impl SecretManageExt for LedgerSecretManager {
    async fn sign_transaction_essence(&self, prepared_transaction: &PreparedTransactionData) -> crate::Result<Unlocks> {
        // lock the mutex to prevent multiple simultaneous requests to a ledger
        let _lock = self.mutex.lock().await;

        let mut input_bip32_indices: Vec<LedgerBIP32Index> = Vec::new();
        let mut coin_type: Option<u32> = None;
        let mut account_index: Option<u32> = None;

        let input_len = prepared_transaction.inputs_data.len();

        for input in &prepared_transaction.inputs_data {
            let bip32_indices: Vec<u32> = match &input.chain {
                Some(chain) => {
                    chain
                        .segments()
                        .iter()
                        // XXX: "ser32(i)". RTFSC: [crypto::keys::slip10::Segment::from_u32()]
                        .map(|seg| u32::from_be_bytes(seg.bs()))
                        .collect()
                }
                None => return Err(crate::Error::NoInputs),
            };
            // coin_type and account_index should be the same in each output
            if (coin_type.is_some() && coin_type != Some(bip32_indices[0]))
                || (account_index.is_some() && account_index != Some(bip32_indices[1]))
            {
                return Err(crate::Error::InvalidBIP32ChainData);
            }

            coin_type = Some(bip32_indices[1]);
            account_index = Some(bip32_indices[2]);
            input_bip32_indices.push(iota_ledger::LedgerBIP32Index {
                bip32_change: bip32_indices[3] | HARDENED,
                bip32_index: bip32_indices[4] | HARDENED,
            });
        }

        if coin_type.is_none() || account_index.is_none() {
            return Err(crate::Error::NoInputs);
        }

        // unwrap values
        let coin_type = coin_type.unwrap() & !HARDENED;
        let bip32_account = account_index.unwrap() | HARDENED;

        // pack essence and hash into vec
        let essence_bytes = prepared_transaction.essence.pack_to_vec();
        let essence_hash = prepared_transaction.essence.hash().to_vec();

        let ledger = iota_ledger::get_ledger(coin_type, bip32_account, self.is_simulator)?;
        let blindsigning = needs_blindsigning(prepared_transaction, ledger.get_buffer_size());

        // if essence + bip32 input indices are larger than the buffer size or the essence contains
        // features / types that are not supported blindsigning will be needed
        if blindsigning {
            // prepare signing
            log::debug!("[LEDGER] prepare blindsigning");
            log::debug!("[LEDGER] {:?} {:?}", input_bip32_indices, essence_hash);
            ledger.prepare_blindsigning(input_bip32_indices, essence_hash)?;
        } else {
            // figure out the remainder address and bip32 index (if there is one)
            let (has_remainder, remainder_address, remainder_bip32): (
                bool,
                Option<&bee_block::address::Address>,
                iota_ledger::LedgerBIP32Index,
            ) = match &prepared_transaction.remainder {
                Some(a) => {
                    let remainder_bip32_indices: Vec<u32> = match &a.chain {
                        Some(chain) => {
                            chain
                                .segments()
                                .iter()
                                // XXX: "ser32(i)". RTFSC: [crypto::keys::slip10::Segment::from_u32()]
                                .map(|seg| u32::from_be_bytes(seg.bs()))
                                .collect()
                        }
                        None => return Err(crate::Error::InvalidBIP32ChainData),
                    };
                    (
                        true,
                        Some(&a.address),
                        iota_ledger::LedgerBIP32Index {
                            bip32_change: remainder_bip32_indices[3] | HARDENED,
                            bip32_index: remainder_bip32_indices[4] | HARDENED,
                        },
                    )
                }
                None => (false, None, iota_ledger::LedgerBIP32Index::default()),
            };

            let mut remainder_index = 0u16;
            if has_remainder {
                match &prepared_transaction.essence {
                    bee_block::payload::transaction::TransactionEssence::Regular(essence) => {
                        // find the index of the remainder in the essence
                        // this has to be done because outputs in essences are sorted
                        // lexically and therefore the remainder is not always the last output.
                        // The index within the essence and the bip32 index will be validated
                        // by the hardware wallet.
                        // The outputs in the essence already are sorted
                        // at this place, so we can rely on their order and don't have to sort it again.
                        'essence_outputs: for output in essence.outputs().iter() {
                            match output {
                                bee_block::output::Output::Basic(s) => {
                                    for block in s.unlock_conditions().iter() {
                                        if let bee_block::output::UnlockCondition::Address(e) = block {
                                            if *remainder_address.unwrap() == *e.address() {
                                                break 'essence_outputs;
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    log::debug!("[LEDGER] unsupported output");
                                    return Err(crate::Error::LedgerMiscError);
                                }
                            }
                            remainder_index += 1;
                        }

                        // was index found?
                        if remainder_index as usize == essence.outputs().len() {
                            log::debug!("[LEDGER] remainder_index not found");
                            return Err(crate::Error::LedgerMiscError);
                        }
                    }
                }
            }

            // prepare signing
            log::debug!("[LEDGER] prepare signing");
            log::debug!(
                "[LEDGER] {:?} {:02x?} {} {} {:?}",
                input_bip32_indices,
                essence_bytes,
                has_remainder,
                remainder_index,
                remainder_bip32
            );
            ledger.prepare_signing(
                input_bip32_indices,
                essence_bytes,
                has_remainder,
                remainder_index,
                remainder_bip32,
            )?;
        }

        // show essence to user
        // if denied by user, it returns with `DeniedByUser` Error
        log::debug!("[LEDGER] await user confirmation");
        ledger.user_confirm()?;

        // sign
        let signature_bytes = ledger.sign(input_len as u16)?;
        let mut readable = &mut &*signature_bytes;

        // unpack signature to unlocks
        let mut unlocks = Vec::new();
        for _ in 0..input_len {
            let unlock = Unlock::unpack_verified(&mut readable).map_err(|_| crate::Error::PackableError)?;
            unlocks.push(unlock);
        }

        // With blindsigning the ledger only returns SignatureUnlocks, so we might have to merge them
        // Alias/Nft/Reference unlocks
        if blindsigning {
            unlocks = merge_unlocks(prepared_transaction, unlocks.into_iter()).await?;
        }

        Ok(Unlocks::new(unlocks)?)
    }
}

impl LedgerSecretManager {
    /// Creates a [`LedgerSecretManager`].
    ///
    /// To use a Ledger Speculos simulator, pass `true` to `is_simulator`; `false` otherwise.
    pub fn new(is_simulator: bool) -> Self {
        Self {
            is_simulator,
            mutex: Mutex::new(()),
        }
    }

    /// Get Ledger hardware status.
    pub async fn get_ledger_status(&self) -> LedgerStatus {
        log::info!("ledger get_opened_app");
        // lock the mutex
        let _lock = self.mutex.lock().await;
        let transport_type = match self.is_simulator {
            true => iota_ledger::TransportTypes::TCP,
            false => iota_ledger::TransportTypes::NativeHID,
        };

        let app = match iota_ledger::get_opened_app(&transport_type) {
            Ok((name, version)) => Some(crate::secret::types::LedgerApp { name, version }),
            _ => None,
        };

        log::info!("get_app_config");
        // if IOTA or Shimmer app is opened, the call will always succeed, returning information like
        // device, debug-flag, version number, lock-state but here we only are interested in a
        // successful call and the locked-flag
        let (connected_, locked, blindsigning_enabled, device) = match iota_ledger::get_app_config(&transport_type) {
            Ok(config) => (
                true,
                // locked flag
                config.flags & (1 << 0) != 0,
                // blindsigning enabled flag
                config.flags & (1 << 1) != 0,
                Some(crate::secret::types::LedgerDeviceType::from(config.device)),
            ),
            Err(_) => (false, false, false, None),
        };

        log::info!("get_buffer_size");
        // get buffer size of connected device
        let buffer_size = match iota_ledger::get_buffer_size(&transport_type) {
            Ok(size) => Some(size),
            Err(_) => None,
        };

        // We get the app info also if not the iota app is open, but another one
        // connected_ is in this case false, even tough the ledger is connected, that's why we always return true if we
        // got the app
        let connected = if app.is_some() { true } else { connected_ };
        LedgerStatus {
            connected,
            locked,
            blindsigning_enabled,
            app,
            device,
            buffer_size,
        }
    }
}

// Merge signature unlocks with Alias/Nft/Reference unlocks
async fn merge_unlocks(
    prepared_transaction_data: &PreparedTransactionData,
    mut unlocks: impl Iterator<Item = Unlock>,
) -> crate::Result<Vec<Unlock>> {
    // The hashed_essence gets signed
    let hashed_essence = prepared_transaction_data.essence.hash();

    let mut merged_unlocks = Vec::new();
    let mut block_indexes = HashMap::<Address, usize>::new();

    for (current_block_index, input) in prepared_transaction_data.inputs_data.iter().enumerate() {
        // Get the address that is required to unlock the input
        let (_, input_address) = Address::try_from_bech32(&input.bech32_address)?;

        // Check if we already added an [Unlock] for this address
        match block_indexes.get(&input_address) {
            // If we already have an [Unlock] for this address, add a [Unlock] based on the address type
            Some(block_index) => match input_address {
                Address::Alias(_alias) => merged_unlocks.push(Unlock::Alias(AliasUnlock::new(*block_index as u16)?)),
                Address::Ed25519(_ed25519) => {
                    merged_unlocks.push(Unlock::Reference(ReferenceUnlock::new(*block_index as u16)?))
                }
                Address::Nft(_nft) => merged_unlocks.push(Unlock::Nft(NftUnlock::new(*block_index as u16)?)),
            },
            None => {
                // We can only sign ed25519 addresses and block_indexes needs to contain the alias or nft
                // address already at this point, because the reference index needs to be lower
                // than the current block index
                if !input_address.is_ed25519() {
                    return Err(crate::Error::MissingInputWithEd25519UnlockCondition);
                }

                let unlock = unlocks
                    .next()
                    .ok_or(crate::Error::MissingInputWithEd25519UnlockCondition)?;

                if let Unlock::Signature(signature_unlock) = &unlock {
                    let Signature::Ed25519(ed25519_signature) = signature_unlock.signature();
                    let ed25519_address = match input_address {
                        Address::Ed25519(ed25519_address) => ed25519_address,
                        _ => return Err(crate::Error::MissingInputWithEd25519UnlockCondition),
                    };
                    ed25519_signature.is_valid(&hashed_essence, &ed25519_address)?;
                }

                merged_unlocks.push(unlock);

                // Add the ed25519 address to the block_indexes, so it gets referenced if further inputs have
                // the same address in their unlock condition
                block_indexes.insert(input_address, current_block_index);
            }
        }

        // When we have an alias or Nft output, we will add their alias or nft address to block_indexes,
        // because they can be used to unlock outputs via [Unlock::Alias] or [Unlock::Nft],
        // that have the corresponding alias or nft address in their unlock condition
        match &input.output {
            Output::Alias(alias_output) => block_indexes.insert(
                Address::Alias(AliasAddress::new(
                    alias_output.alias_id().or_from_output_id(input.output_id()?),
                )),
                current_block_index,
            ),
            Output::Nft(nft_output) => block_indexes.insert(
                Address::Nft(NftAddress::new(
                    nft_output.nft_id().or_from_output_id(input.output_id()?),
                )),
                current_block_index,
            ),
            _ => None,
        };
    }
    Ok(merged_unlocks)
}
