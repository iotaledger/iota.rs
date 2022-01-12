// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::signing::{LedgerStatus, SignerHandle};

use std::{collections::HashMap, fmt, path::Path};

use bee_common::packable::Packable;
use bee_message::{address::Address, unlock_block::UnlockBlock};

use iota_ledger::LedgerBIP32Index;
use tokio::sync::Mutex;

// ledger status codes https://github.com/iotaledger/ledger-iota-app/blob/53c1f96d15f8b014ba8ba31a85f0401bb4d33e18/src/iota_io.h#L54

pub const HARDENED: u32 = 0x80000000;
const MAX_POOL_SIZE: usize = 10_000;

#[derive(Default)]
pub struct LedgerNanoSigner {
    pub is_simulator: bool,
    pub address_pool: Mutex<HashMap<Address, HashMap<AddressPoolEntry, [u8; 32]>>>,
    pub mutex: Mutex<()>,
}

impl LedgerNanoSigner {
    /// Create a new LedgerNanoSigner SignerHandle
    pub fn new(simulator: bool) -> SignerHandle {
        SignerHandle::new(Box::new(LedgerNanoSigner {
            is_simulator: simulator,
            ..Default::default()
        }))
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

#[derive(Hash, Eq, PartialEq)]
pub struct AddressPoolEntry {
    bip32_account: u32,
    bip32_index: u32,
    bip32_change: u32,
}

impl fmt::Display for AddressPoolEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08x}:{:08x}:{:08x}",
            self.bip32_account, self.bip32_change, self.bip32_index
        )
    }
}

#[async_trait::async_trait]
impl super::Signer for LedgerNanoSigner {
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

    async fn generate_address(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        _coin_type: u32,
        account_index: u32,
        address_index: u32,
        internal: bool,
        meta: super::GenerateAddressMetadata,
    ) -> crate::Result<bee_message::address::Address> {
        // lock the mutex
        let _lock = self.mutex.lock().await;

        let bip32_account = account_index | HARDENED;

        let bip32 = iota_ledger::LedgerBIP32Index {
            bip32_index: address_index | HARDENED,
            bip32_change: if internal { 1 } else { 0 } | HARDENED,
        };

        // if it's not for syncing, then it's a new receiving / remainder address
        // that needs shown to the user
        if !meta.syncing {
            log::info!("Interactive address display - not using address pool");

            // get ledger
            let ledger = iota_ledger::get_ledger(bip32_account, self.is_simulator)?;
            /*
                        let compiled_for = match ledger.is_debug_app() {
                            true => Network::Testnet,
                            false => Network::Mainnet,
                        };

                        // check if ledger app is compiled for the same network
                        if compiled_for != meta.network {
                            return Err(crate::Error::LedgerNetMismatch);
                        }
            */
            // and generate a single address that is shown to the user
            let addr = ledger.get_addresses(true, bip32, 1)?;
            return Ok(bee_message::address::Address::Ed25519(
                bee_message::address::Ed25519Address::new(*addr.first().unwrap()),
            ));
        }

        let ledger = iota_ledger::get_ledger(bip32_account, self.is_simulator)?;
        let addresses = ledger.get_addresses(false, bip32, 1)?;
        let iota_address =
            bee_message::address::Address::Ed25519(bee_message::address::Ed25519Address::new(addresses[0]));
        Ok(iota_address)
        // todo handle pool for wallet.rs
        // let pool_key = AddressPoolEntry {
        //     bip32_account,
        //     bip32_index: bip32.bip32_index,
        //     bip32_change: bip32.bip32_change,
        // };

        // let mut global_address_pool = self.address_pool.lock().await;
        // let addr_pool = {
        //     // get first address
        //     let first_public_address = account
        //         .public_addresses()
        //         .iter()
        //         .find(|e| *e.key_index() == 0 && !e.internal());

        //     let entry = match &first_public_address {
        //         Some(address) => global_address_pool.get_mut(&address.address().inner),
        //         None => None,
        //     };
        //     match entry {
        //         Some(address_entry) => address_entry,
        //         None => {
        //             log::info!(
        //                 "Account {} addresses or leder pool entry empty, creating first address for ledger address
        // pool", account.index()             );

        //             // get ledger
        //             let ledger = iota_ledger::get_ledger(bip32_account, self.is_simulator)?;

        //             // generate first address to check if the ledger has the correct seed
        //             let addr = ledger.get_first_address()?;
        //             let iota_address = bee_message::address::Address::Ed25519(
        //                 bee_message::address::Ed25519Address::new(addr),
        //             );

        //             if let Some(first_address) = &first_public_address {
        //                 if first_address.address().inner != iota_address {
        //                     return Err(crate::Error::LedgerMnemonicMismatch);
        //                 }
        //             }
        //             let mut address_pool = HashMap::new();
        //             address_pool.insert(
        //                 AddressPoolEntry {
        //                     bip32_account,
        //                     bip32_index: HARDENED,
        //                     bip32_change: HARDENED,
        //                 },
        //                 addr,
        //             );

        //             global_address_pool.insert(iota_address, address_pool);
        //             global_address_pool
        //                 .get_mut(&iota_address)
        //                 .expect("Missing pool address entry")
        //         }
        //     }
        // };

        // if !addr_pool.contains_key(&pool_key) {
        //     log::info!("Adress {} not found in address pool", pool_key);
        //     // if not, we add new entries to the pool but limit the pool size
        //     if addr_pool.len() > MAX_POOL_SIZE {
        //         log::debug!("address pool has too many entries");
        //         *addr_pool = HashMap::new();
        //     }

        //     let count = 15;
        //     let ledger = iota_ledger::get_ledger(bip32_account, self.is_simulator)?;
        //     /*
        //                 let compiled_for = match ledger.is_debug_app() {
        //                     true => Network::Testnet,
        //                     false => Network::Mainnet,
        //                 };

        //                 // check if ledger app is compiled for the same network
        //                 if compiled_for != meta.network {
        //                     return Err(crate::Error::LedgerNetMismatch);
        //                 }
        //     */
        //     let addresses = ledger.get_addresses(false, bip32, count)?;

        //     // now put all addresses into the pool
        //     for i in 0..count {
        //         addr_pool.insert(
        //             AddressPoolEntry {
        //                 bip32_account,
        //                 bip32_index: bip32.bip32_index + i as u32,
        //                 bip32_change: bip32.bip32_change,
        //             },
        //             *addresses.get(i).unwrap(),
        //         );
        //     }
        //     log::info!("New address pool size is {}", addr_pool.len());

        //     log::debug!("addresses in pool:");
        //     for key in addr_pool.keys() {
        //         log::debug!("{}", key);
        //     }
        // } else {
        //     log::info!("Got {} from pool", pool_key);
        // }
        // Ok(bee_message::address::Address::Ed25519(
        //     bee_message::address::Ed25519Address::new(addr_pool[&pool_key]),
        // ))
    }

    async fn sign_transaction_essence<'a>(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        _coin_type: u32,
        account_index: u32,
        essence: &bee_message::payload::transaction::TransactionEssence,
        inputs: &mut Vec<super::TransactionInput>,
        meta: super::SignMessageMetadata<'a>,
    ) -> crate::Result<Vec<bee_message::unlock_block::UnlockBlock>> {
        // lock the mutex
        let _lock = self.mutex.lock().await;

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
        let mut address_index_recorders: Vec<AddressIndexRecorder> = Vec::new();
        for input in inputs {
            address_index_recorders.push(AddressIndexRecorder {
                input: input.input.clone(),
                bip32: LedgerBIP32Index {
                    bip32_index: input.address_index as u32 | HARDENED,
                    bip32_change: if input.address_internal { 1 } else { 0 } | HARDENED,
                },
            });
        }
        address_index_recorders.sort_by(|a, b| a.input.cmp(&b.input));

        // now extract the bip32 indices in the right order
        let mut input_bip32_indices: Vec<LedgerBIP32Index> = Vec::new();
        for recorder in address_index_recorders {
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
                            bee_message::output::Output::Extended(s) => {
                                if *remainder_address.unwrap() == *s.address() {
                                    break;
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
        let essence_bytes = essence.pack_new();

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
            let unlock_block = UnlockBlock::unpack(&mut readable)?;
            unlock_blocks.push(unlock_block);
        }
        Ok(unlock_blocks)
    }
}
