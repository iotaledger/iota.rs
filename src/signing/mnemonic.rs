// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    signing::{SignerHandle, SignerType},
    Client, Result,
};
use bee_message::{
    address::{Address, Ed25519Address},
    payload::transaction::TransactionEssence,
    signature::{Ed25519Signature, Signature},
    unlock_block::{ReferenceUnlockBlock, SignatureUnlockBlock, UnlockBlock},
};
use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::slip10::{Chain, Curve, Seed},
};

use std::{collections::HashMap, ops::Deref, path::Path};

/// IOTA coin type https://github.com/satoshilabs/slips/blob/master/slip-0044.md
pub const IOTA_COIN_TYPE: u32 = 4218;

fn generate_address(
    seed: &Seed,
    coin_type: u32,
    account_index: u32,
    address_index: u32,
    internal: bool,
) -> crate::Result<Address> {
    // 44 is for BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    let chain = Chain::from_u32_hardened(vec![44, coin_type, account_index, internal as u32, address_index]);
    let public_key = seed
        .derive(Curve::Ed25519, &chain)?
        .secret_key()
        .public_key()
        .to_bytes();
    // Hash the public key to get the address
    let result = Blake2b256::digest(&public_key)
        .try_into()
        .map_err(|_e| crate::Error::Blake2b256Error("Hashing the public key while generating the address failed."));

    Ok(Address::Ed25519(Ed25519Address::new(result?)))
}

/// MnemonicSigner
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
    async fn get_ledger_status(&self, _is_simulator: bool) -> crate::signing::LedgerStatus {
        // dummy status, function is only required in the trait because we need it for the LedgerSigner
        crate::signing::LedgerStatus {
            connected: false,
            locked: false,
            app: None,
        }
    }

    // This function only makes sense for the Stronghold Signer
    async fn store_mnemonic(&mut self, _storage_path: &Path, _mnemonic: String) -> crate::Result<()> {
        Ok(())
    }

    async fn generate_address(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        coin_type: u32,
        account_index: u32,
        address_index: u32,
        internal: bool,
        _: super::GenerateAddressMetadata,
    ) -> crate::Result<Address> {
        generate_address(self.deref(), coin_type, account_index, address_index, internal)
    }

    async fn sign_transaction_essence<'a>(
        &mut self,
        // https://github.com/satoshilabs/slips/blob/master/slip-0044.md
        coin_type: u32,
        account_index: u32,
        essence: &TransactionEssence,
        inputs: &mut Vec<super::TransactionInput>,
        _: super::SignMessageMetadata<'a>,
    ) -> crate::Result<Vec<UnlockBlock>> {
        // order inputs https://github.com/luca-moser/protocol-rfcs/blob/signed-tx-payload/text/0000-transaction-payload/0000-transaction-payload.md
        inputs.sort_by(|a, b| a.input.cmp(&b.input));

        let hashed_essence = essence.hash();
        let mut unlock_blocks = Vec::new();
        let mut signature_indexes = HashMap::<String, usize>::new();

        for (current_block_index, input) in inputs.iter().enumerate() {
            // 44 is for BIP 44 (HD wallets) and 4218 is the registered index for IOTA https://github.com/satoshilabs/slips/blob/master/slip-0044.md
            let chain = Chain::from_u32_hardened(vec![
                // https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki#purpose
                44,
                coin_type,
                account_index,
                input.address_internal as u32,
                input.address_index,
            ]);
            // Check if current path is same as previous path
            // If so, add a reference unlock block
            // Format to differentiate between public and internal addresses
            let index = format!("{}{}", input.address_index, input.address_internal);
            if let Some(block_index) = signature_indexes.get(&index) {
                unlock_blocks.push(UnlockBlock::Reference(ReferenceUnlockBlock::new(*block_index as u16)?));
            } else {
                // If not, we need to create a signature unlock block
                let private_key = self.deref().derive(Curve::Ed25519, &chain)?.secret_key();
                let public_key = private_key.public_key().to_bytes();
                // The signature unlock block needs to sign the hash of the entire transaction essence of the
                // transaction payload
                let signature = Box::new(private_key.sign(&hashed_essence).to_bytes());
                unlock_blocks.push(UnlockBlock::Signature(SignatureUnlockBlock::new(Signature::Ed25519(
                    Ed25519Signature::new(public_key, *signature),
                ))));
                signature_indexes.insert(index, current_block_index);
            }
        }
        Ok(unlock_blocks)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn address() {
        use super::IOTA_COIN_TYPE;
        use crate::signing::{GenerateAddressMetadata, Network};

        let mnemonic = "giant dynamic museum toddler six deny defense ostrich bomb access mercy blood explain muscle shoot shallow glad autumn author calm heavy hawk abuse rally";
        let mnemonic_signer = super::MnemonicSigner::new(mnemonic).unwrap();

        let address = mnemonic_signer
            .lock()
            .await
            .generate_address(
                IOTA_COIN_TYPE,
                0,
                0,
                false,
                GenerateAddressMetadata {
                    syncing: false,
                    network: Network::Testnet,
                },
            )
            .await
            .unwrap();

        assert_eq!(
            address.to_bech32("atoi"),
            "atoi1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluehe53e".to_string()
        );
    }

    #[tokio::test]
    async fn seed_address() {
        use super::IOTA_COIN_TYPE;
        use crate::signing::{GenerateAddressMetadata, Network};

        let seed = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";
        let mnemonic_signer = super::MnemonicSigner::new_from_seed(seed).unwrap();

        let address = mnemonic_signer
            .lock()
            .await
            .generate_address(
                IOTA_COIN_TYPE,
                0,
                0,
                false,
                GenerateAddressMetadata {
                    syncing: false,
                    network: Network::Testnet,
                },
            )
            .await
            .unwrap();

        assert_eq!(
            address.to_bech32("atoi"),
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string()
        );
    }
}
