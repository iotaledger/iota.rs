// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [Signer] with Stronghold as the backend.

use super::{GenerateAddressMetadata, InputSigningData, LedgerStatus, Signer, SignerHandle, SignerType};
use crate::Result;
use async_trait::async_trait;
use bee_message::{
    address::{Address, Ed25519Address},
    signature::{Ed25519Signature, Signature},
    unlock_block::{SignatureUnlockBlock, UnlockBlock},
};
use crypto::hashes::{blake2b::Blake2b256, Digest};
use iota_stronghold::{Location, ProcResult, Procedure, RecordHint, ResultMessage, SLIP10DeriveInput, Stronghold};
use log::warn;
use riker::system::ActorSystem;
use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;
use zeroize::Zeroize;

/// Stronghold vault path to secrets.
///
/// The value has been hard-coded historically.
const SECRET_VAULT_PATH: &[u8] = b"iota-wallet-secret";

/// Stronghold record path to a seed.
///
/// The value has been hard-coded historically.
const SEED_RECORD_PATH: &[u8] = b"iota-wallet-seed";

/// Stronghold record hint.
///
/// The value has been hard-coded historically.
const RECORD_HINT: &str = "wallet.rs-derive";

/// Stronghold record path to a derived SLIP-10 private key.
///
/// The value has been hard-coded historically.
const DERIVE_OUTPUT_RECORD_PATH: &[u8] = b"iota-wallet-derived";

/// Filename to the Stronghold vault.
///
/// The value has been hard-coded historically.
const STRONGHOLD_FILENAME: &str = "wallet.stronghold";

/// The client path for the seed.
///
/// The value has been hard-coded historically.
const PRIVATE_DATA_CLIENT_PATH: &[u8] = b"iota_seed";

/// Hash a password, deriving a key, for accessing Stronghold.
fn derive_key_from_password(password: &str) -> Vec<u8> {
    let mut buffer = [0u8; 64];

    // Safe to unwrap because rounds > 0.
    crypto::keys::pbkdf::PBKDF2_HMAC_SHA512(password.as_bytes(), b"wallet.rs", 100, &mut buffer).unwrap();

    buffer.into_iter().take(32).collect()
}

/// Stronghold as a [Signer].
#[derive(Zeroize)]
pub struct StrongholdSigner {
    #[zeroize(skip)]
    stronghold: Stronghold,
    key_data: Vec<u8>,
    #[zeroize(skip)]
    snapshot_path: PathBuf,
    #[zeroize(skip)]
    snapshot_loaded: bool,
}

#[async_trait]
impl Signer for StrongholdSigner {
    async fn get_ledger_status(&self, _is_simulator: bool) -> LedgerStatus {
        // Do nothing - this function is only useful for [LedgerSigner].
        LedgerStatus {
            connected: false,
            locked: false,
            app: None,
        }
    }

    async fn store_mnemonic(&mut self, _storage_path: &Path, mnemonic: String) -> Result<()> {
        // Stronghold arguments.
        let output = Location::Generic {
            vault_path: SECRET_VAULT_PATH.to_vec(),
            record_path: SEED_RECORD_PATH.to_vec(),
        };
        let hint = RecordHint::new("wallet.rs-seed").unwrap();

        // Trim the mnemonic, in case it hasn't been, as otherwise the restored seed would be wrong.
        let trimmed_mnemonic = mnemonic.trim().to_string();

        // Check if the mnemonic is valid.
        crypto::keys::bip39::wordlist::verify(&trimmed_mnemonic, &crypto::keys::bip39::wordlist::ENGLISH)
            .map_err(|e| crate::Error::InvalidMnemonic(format!("{:?}", e)))?;

        // Try to load the snapshot to see if we're creating a new Stronghold vault or not.
        //
        // XXX: The current design of [Error] doesn't allow us to see if it's really a "file does
        // not exist" error or not. Better throw errors other than that, but now we just leave it
        // like this, as if so then later operations would throw errors too.
        self.lazy_load_snapshot().await.unwrap_or(());

        // If the snapshot has successfully been loaded, then we need to check if there has been a
        // mnemonic stored in Stronghold or not to prevent overwriting it.
        if self.snapshot_loaded && self.stronghold.record_exists(output.clone()).await {
            return Err(crate::Error::StrongholdMnemonicAlreadyStored);
        }

        // Execute the BIP-39 recovery procedure to put it into the vault (in memory).
        self.bip39_recover(trimmed_mnemonic, None, output, hint).await?;

        // Persist Stronghold to the disk.
        self.write_all_to_snapshot(
            self.key_data.clone(),
            Some(STRONGHOLD_FILENAME.to_string()),
            Some(self.snapshot_path.clone()),
        )
        .await?;

        // Now we consider that the snapshot has been loaded; it's just in a reversed order.
        self.snapshot_loaded = true;

        Ok(())
    }

    async fn generate_addresses(
        &mut self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        _metadata: GenerateAddressMetadata,
    ) -> Result<Vec<Address>> {
        // Load the Stronghold snapshot if it hasn't been loaded yet.
        self.lazy_load_snapshot().await?;

        // Stronghold arguments.
        let seed_location = SLIP10DeriveInput::Seed(Location::Generic {
            vault_path: SECRET_VAULT_PATH.to_vec(),
            record_path: SEED_RECORD_PATH.to_vec(),
        });
        let derive_location = Location::Generic {
            vault_path: SECRET_VAULT_PATH.to_vec(),
            record_path: DERIVE_OUTPUT_RECORD_PATH.to_vec(),
        };
        let hint = RecordHint::new(RECORD_HINT).unwrap();

        // Addresses to return.
        let mut addresses = Vec::new();

        for address_index in address_indexes {
            // Stronghold 0.4.1 is still using an older version of iota-crypto, so we construct a different one here.
            let chain = crypto05::keys::slip10::Chain::from_u32_hardened(vec![
                44u32,
                coin_type,
                account_index,
                internal as u32,
                address_index,
            ]);

            // Derive a SLIP-10 private key in the vault.
            self.slip10_derive(chain, seed_location.clone(), derive_location.clone(), hint)
                .await?;

            // Get the Ed25519 public key from the derived SLIP-10 private key in the vault.
            let public_key = self.ed25519_public_key(derive_location.clone()).await?;

            // Hash the public key to get the address.
            let hash = Blake2b256::digest(&public_key);

            // Convert the hash into [Address].
            let address = Address::Ed25519(Ed25519Address::new(hash.into()));

            // Collect it.
            addresses.push(address)
        }

        Ok(addresses)
    }

    async fn signature_unlock(&mut self, input: &InputSigningData, essence_hash: &[u8; 32]) -> Result<UnlockBlock> {
        // Stronghold arguments.
        let seed_location = SLIP10DeriveInput::Seed(Location::Generic {
            vault_path: SECRET_VAULT_PATH.to_vec(),
            record_path: SEED_RECORD_PATH.to_vec(),
        });
        let derive_location = Location::Generic {
            vault_path: SECRET_VAULT_PATH.to_vec(),
            record_path: DERIVE_OUTPUT_RECORD_PATH.to_vec(),
        };
        let hint = RecordHint::new(RECORD_HINT).unwrap();

        // Stronghold asks for an older version of [Chain], so we have to perform a conversion here.
        let chain = {
            let raw: Vec<u32> = input
                .chain
                .as_ref()
                .unwrap()
                .segments()
                .iter()
                // XXX: "ser32(i)". RTFSC: [crypto::keys::slip10::Segment::from_u32()]
                .map(|seg| u32::from_be_bytes(seg.bs()))
                .collect();

            crypto05::keys::slip10::Chain::from_u32_hardened(raw)
        };

        // Derive a SLIP-10 private key in the vault.
        self.slip10_derive(chain, seed_location.clone(), derive_location.clone(), hint)
            .await?;

        // Get the Ed25519 public key from the derived SLIP-10 private key in the vault.
        let public_key = self.ed25519_public_key(derive_location.clone()).await?;

        // Sign the message with the derived SLIP-10 private key in the vault.
        let signature = self.ed25519_sign(derive_location.clone(), essence_hash).await?;

        // Convert the raw bytes into [UnlockBlock].
        let unlock_block = UnlockBlock::Signature(SignatureUnlockBlock::new(Signature::Ed25519(
            Ed25519Signature::new(public_key, signature),
        )));

        Ok(unlock_block)
    }
}

impl StrongholdSigner {
    /// Create a `[StrongholdSigner]`.
    pub fn try_new(password: &str, snapshot_path: &Path) -> Result<StrongholdSigner> {
        let key_data = derive_key_from_password(password);
        let actor = ActorSystem::new()?;
        let options = Vec::new();

        Ok(Self {
            stronghold: Stronghold::init_stronghold_system(actor, PRIVATE_DATA_CLIENT_PATH.to_vec(), options),
            key_data,
            snapshot_path: snapshot_path.to_path_buf(),
            snapshot_loaded: false,
        })
    }

    /// Create a `[SignerHandle]` wrapping a `[StrongholdSigner]`.
    pub fn try_new_signer_handle(password: &str, snapshot_path: &Path) -> Result<SignerHandle> {
        let signer = Self::try_new(password, snapshot_path)?;

        Ok(SignerHandle {
            signer: Arc::new(Mutex::new(Box::new(signer))),
            signer_type: SignerType::Stronghold,
        })
    }

    /// Load the Stronghold snapshot from [Self::snapshot_path], if it hasn't been loaded yet.
    async fn lazy_load_snapshot(&mut self) -> Result<()> {
        if self.snapshot_loaded {
            return Ok(());
        }

        match self
            .stronghold
            .read_snapshot(
                PRIVATE_DATA_CLIENT_PATH.to_vec(),
                None,
                &self.key_data,
                Some(STRONGHOLD_FILENAME.to_string()),
                Some(self.snapshot_path.clone()),
            )
            .await
        {
            ResultMessage::Ok(_) => Ok(()),
            ResultMessage::Error(err) => Err(crate::Error::StrongholdProcedureError(err)),
        }?;

        self.snapshot_loaded = true;

        Ok(())
    }

    /// Execute [Procedure::BIP39Recover] in Stronghold to put a mnemonic into the Stronghold vault.
    async fn bip39_recover(
        &self,
        mnemonic: String,
        passphrase: Option<String>,
        output: Location,
        hint: RecordHint,
    ) -> Result<()> {
        match self
            .stronghold
            .runtime_exec(Procedure::BIP39Recover {
                mnemonic,
                passphrase,
                output,
                hint,
            })
            .await
        {
            // BIP-39 recovery success.
            ProcResult::BIP39Recover(ResultMessage::Ok(_)) => Ok(()),
            // BIP-39 recovery failure.
            // XXX: Should we create a separate error type for this error?
            ProcResult::BIP39Recover(ResultMessage::Error(err)) => Err(crate::Error::StrongholdProcedureError(err)),
            // Generic Stronghold procedure failure.
            ProcResult::Error(err) => Err(crate::Error::StrongholdProcedureError(err)),
            // Unexpected result type, which should never happen!
            err => {
                warn!(
                    "StrongholdSigner::bip39_recover(): unexpected result from Stronghold: {:?}",
                    err
                );
                Err(crate::Error::StrongholdProcedureError(format!("{:?}", err)))
            }
        }
    }

    /// Persist Stronghold to the disk by writing "all" into a "snapshot".
    async fn write_all_to_snapshot(
        &mut self,
        keydata: Vec<u8>,
        filename: Option<String>,
        path: Option<PathBuf>,
    ) -> Result<()> {
        match self.stronghold.write_all_to_snapshot(&keydata, filename, path).await {
            ResultMessage::Ok(_) => Ok(()),
            ResultMessage::Error(err) => Err(crate::Error::StrongholdProcedureError(err)),
        }
    }

    /// Execute [Procedure::SLIP10Derive] in Stronghold to derive a SLIP-10 private key in the Stronghold vault.
    async fn slip10_derive(
        &self,
        // Stronghold 0.4.1 is still using an older version of iota-crypto, so we ask for a different one here.
        chain: crypto05::keys::slip10::Chain,
        input: SLIP10DeriveInput,
        output: Location,
        hint: RecordHint,
    ) -> Result<()> {
        match self
            .stronghold
            .runtime_exec(Procedure::SLIP10Derive {
                chain,
                input,
                output,
                hint,
            })
            .await
        {
            // SLIP-10 derivation success.
            // We don't care about the returned value, as later we use the output in vault.
            ProcResult::SLIP10Derive(ResultMessage::Ok(_)) => Ok(()),
            // SLIP-10 derivation failure.
            // XXX: Should we create a separate error type for this error?
            ProcResult::SLIP10Derive(ResultMessage::Error(err)) => Err(crate::Error::StrongholdProcedureError(err)),
            // Generic Stronghold procedure failure.
            ProcResult::Error(err) => Err(crate::Error::StrongholdProcedureError(err)),
            // Unexpected result type, which should never happen!
            err => {
                warn!(
                    "StrongholdSigner::slip10_derive(): unexpected result from Stronghold: {:?}",
                    err
                );
                Err(crate::Error::StrongholdProcedureError(format!("{:?}", err)))
            }
        }
    }

    /// Execute [Procedure::Ed25519PublicKey] in Stronghold to get an Ed25519 public key from the SLIP-10 private key
    /// located in `private_key`.
    async fn ed25519_public_key(&self, private_key: Location) -> Result<[u8; 32]> {
        match self
            .stronghold
            .runtime_exec(Procedure::Ed25519PublicKey { private_key })
            .await
        {
            // Ed25519 public key get success.
            ProcResult::Ed25519PublicKey(ResultMessage::Ok(pubkey)) => Ok(pubkey),
            // Ed25519 public key get failure.
            // XXX: Should we create a separate error type for this error?
            ProcResult::Ed25519PublicKey(ResultMessage::Error(err)) => Err(crate::Error::StrongholdProcedureError(err)),
            // Generic Stronghold procedure failure.
            ProcResult::Error(err) => Err(crate::Error::StrongholdProcedureError(err)),
            // Unexpected result type, which should never happen!
            err => {
                warn!(
                    "StrongholdSigner::ed25519_public_key(): unexpected result from Stronghold: {:?}",
                    err
                );
                Err(crate::Error::StrongholdProcedureError(format!("{:?}", err)))
            }
        }
    }

    /// Execute [Procedure::Ed25519Sign] in Stronghold to sign `msg` with `private_key` stored in the Stronghold vault.
    async fn ed25519_sign(&self, private_key: Location, msg: &[u8]) -> Result<[u8; 64]> {
        match self
            .stronghold
            .runtime_exec(Procedure::Ed25519Sign {
                private_key,
                msg: msg.to_vec(),
            })
            .await
        {
            // Ed25519 sign success.
            ProcResult::Ed25519Sign(ResultMessage::Ok(msg)) => Ok(msg),
            // Ed25519 sign failure.
            // XXX: Should we create a separate error type for this error?
            ProcResult::Ed25519Sign(ResultMessage::Error(err)) => Err(crate::Error::StrongholdProcedureError(err)),
            // Generic Stronghold procedure failure.
            ProcResult::Error(err) => Err(crate::Error::StrongholdProcedureError(err)),
            // Unexpected result type, which should never happen!
            err => {
                warn!(
                    "StrongholdSigner::ed25519_sign(): unexpected result from Stronghold: {:?}",
                    err
                );
                Err(crate::Error::StrongholdProcedureError(format!("{:?}", err)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn address() {
        use crate::{
            constants::IOTA_COIN_TYPE,
            signing::{GenerateAddressMetadata, Network},
        };

        let storage_path = Path::new("test.stronghold");
        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally";
        let signer = StrongholdSigner::try_new_signer_handle("", &storage_path).unwrap();

        signer
            .lock()
            .await
            .store_mnemonic(&storage_path, mnemonic.to_string())
            .await
            .unwrap();

        let addresses = signer
            .lock()
            .await
            .generate_addresses(
                IOTA_COIN_TYPE,
                0,
                0..1,
                false,
                GenerateAddressMetadata {
                    syncing: false,
                    network: Network::Testnet,
                },
            )
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e".to_string()
        );

        // Remove garbage after test, but don't care about the result
        std::fs::remove_file(storage_path).unwrap_or(());
    }
}
