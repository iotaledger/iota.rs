// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`LedgerSecretManager`].
//!
//! Ledger status codes: <https://github.com/iotaledger/ledger-iota-app/blob/53c1f96d15f8b014ba8ba31a85f0401bb4d33e18/src/iota_io.h#L54>.

use std::{collections::HashMap, ops::Range};

use async_trait::async_trait;
use crypto::keys::slip10::Chain;
use iota_ledger_nano::{
    get_app_config, get_buffer_size, get_ledger, get_opened_app, LedgerBIP32Index, Packable as LedgerNanoPackable,
    TransportTypes,
};
use iota_types::block::{
    address::{Address, AliasAddress, Ed25519Address, NftAddress},
    output::Output,
    payload::transaction::TransactionEssence,
    signature::{Ed25519Signature, Signature},
    unlock::{AliasUnlock, NftUnlock, ReferenceUnlock, Unlock, Unlocks},
};
use packable::{unpacker::SliceUnpacker, Packable, PackableExt};
use tokio::sync::Mutex;

use super::{types::InputSigningData, GenerateAddressOptions, SecretManage, SecretManageExt};
use crate::{
    api::input_selection::Error as InputSelectionError,
    secret::{
        is_alias_transition,
        types::{LedgerApp, LedgerDeviceType},
        LedgerNanoStatus, PreparedTransactionData, RemainderData,
    },
    unix_timestamp_now, Error, Result,
};

/// Hardened const for the bip path.
///
/// See also: <https://wiki.trezor.io/Hardened_and_non-hardened_derivation>.
pub const HARDENED: u32 = 0x8000_0000;

/// Secret manager that uses a Ledger hardware wallet.
#[derive(Default)]
pub struct LedgerSecretManager {
    /// Specifies if a real Ledger hardware is used or only a simulator is used.
    pub is_simulator: bool,

    /// Mutex to prevent multiple simultaneous requests to a ledger.
    pub mutex: Mutex<()>,
}

impl TryFrom<u8> for LedgerDeviceType {
    type Error = Error;
    fn try_from(device: u8) -> Result<Self> {
        match device {
            0 => Ok(Self::LedgerNanoS),
            1 => Ok(Self::LedgerNanoX),
            2 => Ok(Self::LedgerNanoSPlus),
            _ => Err(Error::LedgerMiscError),
        }
    }
}

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
        options: Option<GenerateAddressOptions>,
    ) -> crate::Result<Vec<Address>> {
        let bip32_account = account_index | HARDENED;

        let bip32 = LedgerBIP32Index {
            bip32_index: address_indexes.start | HARDENED,
            bip32_change: u32::from(internal) | HARDENED,
        };

        // lock the mutex to prevent multiple simultaneous requests to a ledger
        let lock = self.mutex.lock().await;

        // get ledger
        let ledger = get_ledger(coin_type, bip32_account, self.is_simulator)?;

        let addresses = if options.unwrap_or_default().ledger_nano_prompt {
            // and generate a single address that is shown to the user
            ledger.get_addresses(true, bip32, address_indexes.len())?
        } else {
            ledger.get_addresses(false, bip32, address_indexes.len())?
        };

        drop(lock);

        let mut ed25519_addresses = Vec::new();
        for address in addresses {
            ed25519_addresses.push(Address::Ed25519(Ed25519Address::new(address)));
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

    async fn sign_ed25519(&self, _msg: &[u8], _chain: &Chain) -> crate::Result<Ed25519Signature> {
        panic!("sign_ed25519 is not supported with ledger")
    }
}

/// needs_blind_signing
/// the Ledger Nano S(+)/X app can present the user a detailed view of the transaction before it
/// is signed but only with BasicOutputs, without extra-features and if the Essence is not too large.
/// If criteria are not met, blind signing is needed.
/// This method finds out if we have to switch to blind signing mode.
pub fn needs_blind_signing(prepared_transaction: &PreparedTransactionData, buffer_size: usize) -> bool {
    match &prepared_transaction.essence {
        TransactionEssence::Regular(essence) => {
            for output in essence.outputs().iter() {
                // only basic outputs allowed
                if let Output::Basic(output) = output {
                    if output.simple_deposit_address().is_some() {
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
    async fn sign_transaction_essence(
        &self,
        prepared_transaction: &PreparedTransactionData,
        time: Option<u32>,
    ) -> crate::Result<Unlocks> {
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
                None => return Err(InputSelectionError::NoAvailableInputsProvided)?,
            };
            // coin_type and account_index should be the same in each output
            if (coin_type.is_some() && coin_type != Some(bip32_indices[1]))
                || (account_index.is_some() && account_index != Some(bip32_indices[2]))
            {
                return Err(crate::Error::InvalidBIP32ChainData);
            }

            coin_type = Some(bip32_indices[1]);
            account_index = Some(bip32_indices[2]);
            input_bip32_indices.push(LedgerBIP32Index {
                bip32_change: bip32_indices[3] | HARDENED,
                bip32_index: bip32_indices[4] | HARDENED,
            });
        }

        if coin_type.is_none() || account_index.is_none() {
            return Err(InputSelectionError::NoAvailableInputsProvided)?;
        }

        // unwrap values
        let coin_type = coin_type.unwrap() & !HARDENED;
        let bip32_account = account_index.unwrap() | HARDENED;

        // pack essence and hash into vec
        let essence_bytes = prepared_transaction.essence.pack_to_vec();
        let essence_hash = prepared_transaction.essence.hash().to_vec();

        // lock the mutex to prevent multiple simultaneous requests to a ledger
        let lock = self.mutex.lock().await;

        let ledger = get_ledger(coin_type, bip32_account, self.is_simulator)?;
        let blind_signing = needs_blind_signing(prepared_transaction, ledger.get_buffer_size());

        // if essence + bip32 input indices are larger than the buffer size or the essence contains
        // features / types that are not supported blind signing will be needed
        if blind_signing {
            // prepare signing
            log::debug!("[LEDGER] prepare_blind_signing");
            log::debug!("[LEDGER] {:?} {:?}", input_bip32_indices, essence_hash);
            ledger.prepare_blind_signing(input_bip32_indices, essence_hash)?;
        } else {
            // figure out the remainder address and bip32 index (if there is one)
            let (remainder_address, remainder_bip32): (Option<&Address>, LedgerBIP32Index) =
                match &prepared_transaction.remainder {
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
                            Some(&a.address),
                            LedgerBIP32Index {
                                bip32_change: remainder_bip32_indices[3] | HARDENED,
                                bip32_index: remainder_bip32_indices[4] | HARDENED,
                            },
                        )
                    }
                    None => (None, LedgerBIP32Index::default()),
                };

            let mut remainder_index = 0u16;
            if let Some(remainder_address) = remainder_address {
                match &prepared_transaction.essence {
                    TransactionEssence::Regular(essence) => {
                        // find the index of the remainder in the essence
                        // this has to be done because outputs in essences are sorted
                        // lexically and therefore the remainder is not always the last output.
                        // The index within the essence and the bip32 index will be validated
                        // by the hardware wallet.
                        // The outputs in the essence already are sorted
                        // at this place, so we can rely on their order and don't have to sort it again.
                        'essence_outputs: for output in essence.outputs().iter() {
                            if let Output::Basic(s) = output {
                                if let Some(address) = s.unlock_conditions().address() {
                                    if *remainder_address == *address.address() {
                                        break 'essence_outputs;
                                    }
                                }
                            } else {
                                log::debug!("[LEDGER] unsupported output");
                                return Err(crate::Error::LedgerMiscError);
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
                remainder_address.is_some(),
                remainder_index,
                remainder_bip32
            );
            ledger.prepare_signing(
                input_bip32_indices,
                essence_bytes,
                remainder_address.is_some(),
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
        drop(ledger);
        drop(lock);
        let mut unpacker = SliceUnpacker::new(&signature_bytes);

        // unpack signature to unlocks
        let mut unlocks = Vec::new();
        for _ in 0..input_len {
            let unlock = Unlock::unpack::<_, true>(&mut unpacker, &())?;
            // The ledger nano can return the same SignatureUnlocks multiple times, so only insert it once
            match unlock {
                Unlock::Signature(_) => {
                    if !unlocks.contains(&unlock) {
                        unlocks.push(unlock);
                    }
                }
                // Multiple reference unlocks with the same index are allowed
                _ => unlocks.push(unlock),
            }
        }

        // With blind signing the ledger only returns SignatureUnlocks, so we might have to merge them with
        // Alias/Nft/Reference unlocks
        if blind_signing {
            unlocks = merge_unlocks(prepared_transaction, unlocks.into_iter(), time)?;
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
    pub async fn get_ledger_nano_status(&self) -> LedgerNanoStatus {
        log::debug!("get_ledger_nano_status");
        // lock the mutex
        let _lock = self.mutex.lock().await;
        let transport_type = if self.is_simulator {
            TransportTypes::TCP
        } else {
            TransportTypes::NativeHID
        };

        log::debug!("get_opened_app");
        let app = match get_opened_app(&transport_type) {
            Ok((name, version)) => Some(LedgerApp { name, version }),
            _ => None,
        };

        log::debug!("get_app_config");
        // if IOTA or Shimmer app is opened, the call will always succeed, returning information like
        // device, debug-flag, version number, lock-state but here we only are interested in a
        // successful call and the locked-flag
        let (connected_, locked, blind_signing_enabled, device) =
            get_app_config(&transport_type).map_or((false, false, false, None), |config| {
                (
                    true,
                    // locked flag
                    config.flags & (1 << 0) != 0,
                    // blind signing enabled flag
                    config.flags & (1 << 1) != 0,
                    LedgerDeviceType::try_from(config.device).ok(),
                )
            });

        log::debug!("get_buffer_size");
        // get buffer size of connected device
        let buffer_size = get_buffer_size(&transport_type).ok();

        // We get the app info also if not the iota app is open, but another one
        // connected_ is in this case false, even tough the ledger is connected, that's why we always return true if we
        // got the app
        let connected = if app.is_some() { true } else { connected_ };
        LedgerNanoStatus {
            connected,
            locked,
            blind_signing_enabled,
            app,
            device,
            buffer_size,
        }
    }
}

// Merge signature unlocks with Alias/Nft/Reference unlocks
fn merge_unlocks(
    prepared_transaction_data: &PreparedTransactionData,
    mut unlocks: impl Iterator<Item = Unlock>,
    time: Option<u32>,
) -> crate::Result<Vec<Unlock>> {
    // The hashed_essence gets signed
    let hashed_essence = prepared_transaction_data.essence.hash();

    let time = time.unwrap_or_else(unix_timestamp_now);

    let mut merged_unlocks = Vec::new();
    let mut block_indexes = HashMap::<Address, usize>::new();

    // Assuming inputs_data is ordered by address type
    for (current_block_index, input) in prepared_transaction_data.inputs_data.iter().enumerate() {
        // Get the address that is required to unlock the input
        let TransactionEssence::Regular(regular) = &prepared_transaction_data.essence;
        let alias_transition = is_alias_transition(input, regular.outputs()).map(|t| t.0);
        let (input_address, _) =
            input
                .output
                .required_and_unlocked_address(time, input.output_metadata.output_id(), alias_transition)?;

        // Check if we already added an [Unlock] for this address
        match block_indexes.get(&input_address) {
            // If we already have an [Unlock] for this address, add a [Unlock] based on the address type
            Some(block_index) => match input_address {
                Address::Alias(_alias) => merged_unlocks.push(Unlock::Alias(AliasUnlock::new(*block_index as u16)?)),
                Address::Ed25519(_ed25519) => {
                    merged_unlocks.push(Unlock::Reference(ReferenceUnlock::new(*block_index as u16)?));
                }
                Address::Nft(_nft) => merged_unlocks.push(Unlock::Nft(NftUnlock::new(*block_index as u16)?)),
            },
            None => {
                // We can only sign ed25519 addresses and block_indexes needs to contain the alias or nft
                // address already at this point, because the reference index needs to be lower
                // than the current block index
                if !input_address.is_ed25519() {
                    return Err(InputSelectionError::MissingInputWithEd25519Address)?;
                }

                let unlock = unlocks
                    .next()
                    .ok_or(InputSelectionError::MissingInputWithEd25519Address)?;

                if let Unlock::Signature(signature_unlock) = &unlock {
                    let Signature::Ed25519(ed25519_signature) = signature_unlock.signature();
                    let ed25519_address = match input_address {
                        Address::Ed25519(ed25519_address) => ed25519_address,
                        _ => return Err(InputSelectionError::MissingInputWithEd25519Address)?,
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
                Address::Alias(AliasAddress::new(alias_output.alias_id_non_null(input.output_id()))),
                current_block_index,
            ),
            Output::Nft(nft_output) => block_indexes.insert(
                Address::Nft(NftAddress::new(nft_output.nft_id_non_null(input.output_id()))),
                current_block_index,
            ),
            _ => None,
        };
    }
    Ok(merged_unlocks)
}
