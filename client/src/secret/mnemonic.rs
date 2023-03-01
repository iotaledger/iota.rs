// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`MnemonicSecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::{Chain, Curve, Seed},
};
use iota_types::block::{
    address::{Address, Ed25519Address},
    signature::{Ed25519Signature, Signature},
    unlock::{SignatureUnlock, Unlock},
};

use super::{types::InputSigningData, GenerateAddressOptions, SecretManage};
use crate::{constants::HD_WALLET_TYPE, secret::RemainderData, Client, Result};

/// Secret manager that uses only a mnemonic.
///
/// Computation are done in-memory. A mnemonic needs to be supplied upon the creation of [`MnemonicSecretManager`].
pub struct MnemonicSecretManager(Seed);

#[async_trait]
impl SecretManage for MnemonicSecretManager {
    async fn generate_addresses(
        &self,
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        _: Option<GenerateAddressOptions>,
    ) -> crate::Result<Vec<Address>> {
        let mut addresses = Vec::new();

        for address_index in address_indexes {
            let chain = Chain::from_u32_hardened(vec![
                HD_WALLET_TYPE,
                coin_type,
                account_index,
                internal as u32,
                address_index,
            ]);

            let public_key = self
                .0
                .derive(Curve::Ed25519, &chain)?
                .secret_key()
                .public_key()
                .to_bytes();

            // Hash the public key to get the address
            let result = Blake2b256::digest(public_key)
                .try_into()
                .map_err(|_e| crate::Error::Blake2b256("hashing the public key while generating the address failed."));

            addresses.push(Address::Ed25519(Ed25519Address::new(result?)));
        }

        Ok(addresses)
    }

    async fn signature_unlock(
        &self,
        input: &InputSigningData,
        essence_hash: &[u8; 32],
        _: &Option<RemainderData>,
    ) -> crate::Result<Unlock> {
        // The signature unlock block needs to sign the hash of the entire transaction essence of the
        // transaction payload
        let chain = input.chain.as_ref().unwrap();
        let ed25519_sig = self.sign_ed25519(essence_hash, chain).await?;

        Ok(Unlock::Signature(SignatureUnlock::new(Signature::Ed25519(ed25519_sig))))
    }

    async fn sign_ed25519(&self, msg: &[u8], chain: &Chain) -> crate::Result<Ed25519Signature> {
        // Get the private and public key for this Ed25519 address
        let private_key = self.0.derive(Curve::Ed25519, chain)?.secret_key();
        let public_key = private_key.public_key().to_bytes();
        let signature = private_key.sign(msg).to_bytes();

        Ok(Ed25519Signature::new(public_key, signature))
    }
}

impl MnemonicSecretManager {
    /// Create a new [`MnemonicSecretManager`] from a BIP-39 mnemonic in English.
    ///
    /// For more information, see <https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki>.
    pub fn try_from_mnemonic(mnemonic: &str) -> Result<Self> {
        Ok(Self(Client::mnemonic_to_seed(mnemonic)?))
    }

    /// Create a new [`MnemonicSecretManager`] from a hex-encoded raw seed string.
    pub fn try_from_hex_seed(hex: &str) -> Result<Self> {
        let bytes: Vec<u8> = prefix_hex::decode(hex)?;
        Ok(Self(Seed::from_bytes(&bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn address() {
        use crate::constants::IOTA_COIN_TYPE;

        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally";
        let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic).unwrap();

        let addresses = secret_manager
            .generate_addresses(IOTA_COIN_TYPE, 0, 0..1, false, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e".to_string()
        );
    }

    #[tokio::test]
    async fn seed_address() {
        use crate::constants::IOTA_COIN_TYPE;

        let seed = "0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";
        let secret_manager = MnemonicSecretManager::try_from_hex_seed(seed).unwrap();

        let addresses = secret_manager
            .generate_addresses(IOTA_COIN_TYPE, 0, 0..1, false, None)
            .await
            .unwrap();

        assert_eq!(
            addresses[0].to_bech32("atoi"),
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string()
        );
    }
}
