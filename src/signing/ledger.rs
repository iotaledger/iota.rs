// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Ledger signer

use crate::signing::{LedgerStatus, SignerHandle, SignerType};

use bee_message::{
    address::Address,
    input::{Input, UtxoInput},
    payload::transaction::TransactionId,
    unlock_block::UnlockBlock,
};
use packable::PackableExt;

use iota_ledger::LedgerBIP32Index;
use tokio::sync::Mutex;

use std::{ops::Range, path::Path, str::FromStr};

// ledger status codes https://github.com/iotaledger/ledger-iota-app/blob/53c1f96d15f8b014ba8ba31a85f0401bb4d33e18/src/iota_io.h#L54

/// Hardened const for the bip path https://wiki.trezor.io/Hardened_and_non-hardened_derivation
pub const HARDENED: u32 = 0x80000000;

/// Ledger Signer
#[derive(Default)]
pub struct LedgerSigner {
    /// Specifies if a real Ledger hardware is used or only a simulator
    pub is_simulator: bool,
    /// Mutex to prevent multiple simultaneous requests to the ledger
    pub mutex: Mutex<()>,
}

impl LedgerSigner {
    /// Create a new LedgerSigner SignerHandle
    #[allow(clippy::new_ret_no_self)]
    pub fn new(simulator: bool) -> SignerHandle {
        let signer_type = if simulator {
            SignerType::LedgerNano
        } else {
            SignerType::LedgerNanoSimulator
        };
        SignerHandle::new(
            signer_type,
            Box::new(LedgerSigner {
                is_simulator: simulator,
                ..Default::default()
            }),
        )
    }
}

/// A record matching an Input with its address.
#[derive(Debug)]
struct AddressIndexRecorder {
    /// the input
    pub(crate) input: bee_message::input::Input,

    /// bip32 index
    pub(crate) bip32: LedgerBIP32Index,
}

#[async_trait::async_trait]
impl super::Signer for LedgerSigner {
    async fn get_ledger_status(&self, is_simulator: bool) -> LedgerStatus {
        log::info!("ledger get_opened_app");
        // lock the mutex
        let _lock = self.mutex.lock().await;
        let transport_type = match is_simulator {
            true => iota_ledger::TransportTypes::TCP,
            false => iota_ledger::TransportTypes::NativeHID,
        };

        let app = match iota_ledger::get_opened_app(&transport_type) {
            Ok((name, version)) => Some(crate::signing::types::LedgerApp { name, version }),
            _ => None,
        };

        log::info!("get_ledger");
        let (connected_, locked) =
            match iota_ledger::get_ledger(crate::signing::ledger::HARDENED, is_simulator).map_err(Into::into) {
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

    async fn store_mnemonic(&mut self, _: &Path, _mnemonic: String) -> crate::Result<()> {
        Err(crate::Error::InvalidMnemonic(String::from(
            "Can't store mnemonic to ledger",
        )))
    }

    async fn generate_addresses(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        _coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        meta: super::GenerateAddressMetadata,
    ) -> crate::Result<Vec<Address>> {
        // lock the mutex
        let _lock = self.mutex.lock().await;

        let bip32_account = account_index | HARDENED;

        let bip32 = iota_ledger::LedgerBIP32Index {
            bip32_index: address_indexes.start | HARDENED,
            bip32_change: if internal { 1 } else { 0 } | HARDENED,
        };
        // get ledger
        let ledger = iota_ledger::get_ledger(bip32_account, self.is_simulator)?;

        // let compiled_for = match ledger.is_debug_app() {
        // true => Network::Testnet,
        // false => Network::Mainnet,
        // };
        //
        // check if ledger app is compiled for the same network
        // if compiled_for != meta.network {
        // return Err(crate::Error::LedgerNetMismatch);
        // }

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
            ed25519_addresses.push(bee_message::address::Address::Ed25519(
                bee_message::address::Ed25519Address::new(address),
            ));
        }
        Ok(ed25519_addresses)
    }

    async fn sign_transaction_essence<'a>(
        &mut self,
        essence: &bee_message::payload::transaction::TransactionEssence,
        inputs: &mut Vec<super::InputSigningData>,
        meta: super::SignMessageMetadata<'a>,
    ) -> crate::Result<Vec<bee_message::unlock_block::UnlockBlock>> {
        // lock the mutex
        let _lock = self.mutex.lock().await;

        // todo don't use default 0, use account index from InputSigningData
        let account_index = 0;
        let bip32_account = account_index | HARDENED;
        let ledger = iota_ledger::get_ledger(bip32_account, self.is_simulator)?;
        // let compiled_for = match ledger.is_debug_app() {
        // true => Network::Testnet,
        // false => Network::Mainnet,
        // };
        //
        // check if ledger app is compiled for the same network
        // if compiled_for != meta.network {
        // return Err(crate::Error::LedgerNetMismatch);
        // }
        let input_len = inputs.len();

        // on essence finalization, inputs are sorted lexically before they are packed into bytes.
        // we need the correct order of the bip32 indices before we can call PrepareSigning, but
        // because inputs of the essence don't have bip32 indices, we need to sort it on our own too.
        let mut input_signing_data_entrys: Vec<AddressIndexRecorder> = Vec::new();
        for input_signing_data in inputs {
            let input = Input::Utxo(UtxoInput::new(
                TransactionId::from_str(&input_signing_data.output_response.transaction_id)?,
                input_signing_data.output_response.output_index,
            )?);
            // todo validate
            let address_index = u32::from_be_bytes(input_signing_data.chain.clone().unwrap().segments()[3].bs());
            let address_internal = u32::from_be_bytes(input_signing_data.chain.clone().unwrap().segments()[4].bs());
            input_signing_data_entrys.push(AddressIndexRecorder {
                input,
                bip32: LedgerBIP32Index {
                    bip32_index: address_index | HARDENED,
                    bip32_change: address_internal | HARDENED,
                },
            });
        }
        // input_signing_data_entrys.sort_by(|a, b| a.input.cmp(&b.input));

        // now extract the bip32 indices in the right order
        let mut input_bip32_indices: Vec<LedgerBIP32Index> = Vec::new();
        for recorder in input_signing_data_entrys {
            input_bip32_indices.push(recorder.bip32);
        }

        // figure out the remainder address and bip32 index (if there is one)
        let (has_remainder, remainder_address, remainder_bip32): (
            bool,
            Option<&bee_message::address::Address>,
            LedgerBIP32Index,
        ) = match meta.remainder_deposit_address {
            Some(a) => (
                true,
                Some(&a.address),
                LedgerBIP32Index {
                    bip32_index: a.key_index | HARDENED,
                    bip32_change: if a.internal { 1 } else { 0 } | HARDENED,
                },
            ),
            None => (false, None, LedgerBIP32Index::default()),
        };

        let mut remainder_index = 0u16;
        if has_remainder {
            match essence {
                bee_message::payload::transaction::TransactionEssence::Regular(essence) => {
                    // find the index of the remainder in the essence
                    // this has to be done because outputs in essences are sorted
                    // lexically and therefore the remainder is not always the last output.
                    // The index within the essence and the bip32 index will be validated
                    // by the hardware wallet.
                    // The outputs in the essence already are sorted (done by `essence_builder.finish`)
                    // at this place, so we can rely on their order and don't have to sort it again.
                    for output in essence.outputs().iter() {
                        match output {
                            bee_message::output::Output::Basic(s) => {
                                // todo verify if that's the correct expected behaviour
                                for block in s.unlock_conditions() {
                                    if let bee_message::output::UnlockCondition::Address(e) = block {
                                        if *remainder_address.unwrap() == *e.address() {
                                            break;
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

        // pack essence into bytes
        let essence_bytes = essence.pack_to_vec();

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
            let unlock_block = UnlockBlock::unpack_verified(&mut readable).map_err(|_| crate::Error::PackableError)?;
            unlock_blocks.push(unlock_block);
        }
        Ok(unlock_blocks)
    }
}
