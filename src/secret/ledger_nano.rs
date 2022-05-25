// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`LedgerSecretManager`].
//!
//! Ledger status codes: <https://github.com/iotaledger/ledger-iota-app/blob/53c1f96d15f8b014ba8ba31a85f0401bb4d33e18/src/iota_io.h#L54>.

use std::ops::Range;

use async_trait::async_trait;
use bee_block::{
    address::Address,
    unlock::{Unlock, Unlocks},
};
use tokio::sync::Mutex;

use super::{types::InputSigningData, GenerateAddressMetadata, SecretManage, SecretManageExt};
use crate::secret::{LedgerStatus, PreparedTransactionData, RemainderData};

use packable::PackableExt;

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
        // current ledger app only supports IOTA_COIN_TYPE and SHIMMER_COIN_TYPE
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

fn needs_blindsigning(prepared_transaction: &PreparedTransactionData) -> bool {
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
        } //_ => return true
    }
    false
}

#[async_trait]
impl SecretManageExt for LedgerSecretManager {
    async fn sign_transaction_essence(&self, prepared_transaction: &PreparedTransactionData) -> crate::Result<Unlocks> {
        // lock the mutex to prevent multiple simultaneous requests to a ledger
        let _lock = self.mutex.lock().await;

        let mut input_bip32_indices: Vec<iota_ledger::LedgerBIP32Index> = Vec::new();
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
            assert!(
                (coin_type.is_none() || coin_type == Some(bip32_indices[0]))
                    && (account_index.is_none() || account_index == Some(bip32_indices[1]))
            );
            coin_type = Some(bip32_indices[1]);
            account_index = Some(bip32_indices[2]);
            input_bip32_indices.push(iota_ledger::LedgerBIP32Index {
                bip32_change: bip32_indices[3] | HARDENED,
                bip32_index: bip32_indices[4] | HARDENED,
            });
        }

        assert!(coin_type.is_some() && account_index.is_some());

        // unwrap values
        let coin_type = coin_type.unwrap() & !HARDENED;
        let bip32_account = account_index.unwrap() | HARDENED;

        // pack essence and hash into vec
        let essence_bytes = prepared_transaction.essence.pack_to_vec();
        let essence_hash = prepared_transaction.essence.hash().to_vec();

        let ledger = iota_ledger::get_ledger(coin_type, bip32_account, self.is_simulator)?;

        if needs_blindsigning(&prepared_transaction) {
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
                        // todo error
                        None => return Err(crate::Error::NoInputs),
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
                "[LEDGER] {:?} {:?} {} {} {:?}",
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
        // unpack signature to unlockblocks
        let mut unlock_blocks = Vec::new();
        for _ in 0..input_len {
            let unlock_block = Unlock::unpack_verified(&mut readable).map_err(|_| crate::Error::PackableError)?;
            unlock_blocks.push(unlock_block);
        }
        Ok(unlock_blocks)
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

        log::info!("get_ledger");
        let (connected_, locked) = match iota_ledger::get_ledger(
            0x107a, /* FIXME */
            crate::secret::ledger_nano::HARDENED,
            self.is_simulator,
        )
        .map_err(Into::into)
        {
            Ok(_) => (true, false),
            Err(crate::Error::LedgerDongleLocked) => (true, true),
            Err(_) => (false, false),
        };
        // We get the app info also if not the iota app is open, but another one
        // connected_ is in this case false, even tough the ledger is connected, that's why we always return true if we
        // got the app
        let connected = if app.is_some() { true } else { connected_ };
        LedgerStatus { connected, locked, app }
    }
}
