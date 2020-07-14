use crate::error::Result;
use bee_crypto::ternary::Kerl;
use bee_signing::ternary::{
    PrivateKey, PrivateKeyGenerator, PublicKey, TernarySeed as Seed,
    wots::{
        WotsSecurityLevel,
        WotsSpongePrivateKeyGeneratorBuilder,
    },
};
use bee_transaction::bundled::{Address, BundledTransactionField};

use crate::Client;

/// Builder to construct GetNewAddress API
//#[derive(Debug)]
pub struct GetNewAddressBuilder<'a> {
    seed: &'a Seed<Kerl>,
    index: u64,
    security: WotsSecurityLevel,
}

impl<'a> GetNewAddressBuilder<'a> {
    pub(crate) fn new(seed: &'a Seed<Kerl>) -> Self {
        Self {
            seed: seed,
            index: 0,
            security: WotsSecurityLevel::Medium,
        }
    }

    /// Set key index to start search at
    pub fn index(mut self, index: u64) -> Self {
        self.index = index;
        self
    }

    /// Set security level
    pub fn security(mut self, security: u8) -> Self {
        self.security = match security {
            1 => WotsSecurityLevel::Low,
            2 => WotsSecurityLevel::Medium,
            3 => WotsSecurityLevel::High,
            _ => panic!("Invalid security level"),
        };
        self
    }

    /// Send GetNewAddress request
    pub async fn generate(self) -> Result<(u64, Address)> {
        let mut index = self.index;

        loop {
            // TODO impl Error trait in iota_signing_preview
            let address = Address::from_inner_unchecked(
                WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
                    .security_level(self.security)
                    .build()
                    .unwrap()
                    .generate_from_seed(self.seed, index)
                    .unwrap()
                    .generate_public_key()
                    .unwrap()
                    .to_trits()
                    .to_owned(),
            );

            if let Ok(false) = Client::is_address_used(&address).await {
                break Ok((index, address));
            }

            index += 1;
        }
    }
}
