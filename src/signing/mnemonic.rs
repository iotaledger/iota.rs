// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{types::InputSigningData, SignMessageMetadata};
use crate::{
    constants::HD_WALLET_TYPE,
    signing::{SignerHandle, SignerType},
    Client, Result,
};
use bee_message::{
    address::{Address, Ed25519Address},
    signature::{Ed25519Signature, Signature},
    unlock_block::{SignatureUnlockBlock, UnlockBlock},
};
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::{Chain, Curve, Seed},
};
use std::ops::{Deref, Range};

fn generate_addresses(
    seed: &Seed,
    coin_type: u32,
    account_index: u32,
    address_indexes: Range<u32>,
    internal: bool,
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
        let public_key = seed
            .derive(Curve::Ed25519, &chain)?
            .secret_key()
            .public_key()
            .to_bytes();
        // Hash the public key to get the address
        let result = Blake2b256::digest(&public_key)
            .try_into()
            .map_err(|_e| crate::Error::Blake2b256Error("Hashing the public key while generating the address failed."));

        addresses.push(Address::Ed25519(Ed25519Address::new(result?)));
    }
    Ok(addresses)
}

/// MnemonicSigner, also used for seeds
pub struct MnemonicSigner(Seed);

impl Deref for MnemonicSigner {
    type Target = Seed;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MnemonicSigner {
    /// Create a new MnemonicSigner SignerHandle with a given BIP39 mnemonic from the English wordlist
    /// for more information see https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki
    #[allow(clippy::new_ret_no_self)]
    pub fn new(mnemonic: &str) -> Result<SignerHandle> {
        Ok(SignerHandle::new(
            SignerType::Mnemonic,
            Box::new(Self(Client::mnemonic_to_seed(mnemonic)?)),
        ))
    }
    /// Create a new MnemonicSigner SignerHandle with a given hex encoded seed
    pub fn new_from_seed(seed: &str) -> Result<SignerHandle> {
        Ok(SignerHandle::new(
            SignerType::Mnemonic,
            Box::new(Self(Seed::from_bytes(&hex::decode(seed)?))),
        ))
    }
}

#[async_trait::async_trait]
impl crate::signing::Signer for MnemonicSigner {
    async fn generate_addresses(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        coin_type: u32,
        account_index: u32,
        address_indexes: Range<u32>,
        internal: bool,
        _: super::GenerateAddressMetadata,
    ) -> crate::Result<Vec<Address>> {
        generate_addresses(self.deref(), coin_type, account_index, address_indexes, internal)
    }

    async fn signature_unlock<'a>(
        &mut self,
        input: &InputSigningData,
        essence_hash: &[u8; 32],
        _: &SignMessageMetadata<'a>,
    ) -> crate::Result<UnlockBlock> {
        // Get the private and public key for this Ed25519 address
        let private_key = self
            .deref()
            .derive(Curve::Ed25519, &input.chain.clone().expect("no chain in ed25519 input"))?
            .secret_key();
        let public_key = private_key.public_key().to_bytes();

        // The signature unlock block needs to sign the hash of the entire transaction essence of the
        // transaction payload
        let signature = private_key.sign(essence_hash).to_bytes();

        Ok(UnlockBlock::Signature(SignatureUnlockBlock::new(Signature::Ed25519(
            Ed25519Signature::new(public_key, signature),
        ))))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn address() {
        use crate::{
            constants::IOTA_COIN_TYPE,
            signing::{GenerateAddressMetadata, Network},
        };

        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally";
        let mnemonic_signer = super::MnemonicSigner::new(mnemonic).unwrap();

        let addresses = mnemonic_signer
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
    }

    #[tokio::test]
    async fn seed_address() {
        use crate::{
            constants::IOTA_COIN_TYPE,
            signing::{GenerateAddressMetadata, Network},
        };

        let seed = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";
        let mnemonic_signer = super::MnemonicSigner::new_from_seed(seed).unwrap();

        let addresses = mnemonic_signer
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
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string()
        );
    }
}
