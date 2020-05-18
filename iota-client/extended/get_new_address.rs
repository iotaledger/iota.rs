use anyhow::Result;
use iota_bundle_preview::{Address, TransactionField};
use iota_crypto_preview::Kerl;
use iota_signing_preview::{
    IotaSeed, PrivateKey, PrivateKeyGenerator, PublicKey, WotsPrivateKeyGeneratorBuilder,
    WotsSecurityLevel,
};

use crate::Client;

/// Builder to construct GetNewAddress API
//#[derive(Debug)]
pub struct GetNewAddressBuilder<'a> {
    seed: &'a IotaSeed<Kerl>,
    index: u64,
    security: WotsSecurityLevel,
}

impl<'a> GetNewAddressBuilder<'a> {
    pub(crate) fn new(seed: &'a IotaSeed<Kerl>) -> Self {
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
                WotsPrivateKeyGeneratorBuilder::<Kerl>::default()
                    .security_level(self.security)
                    .build()
                    .unwrap()
                    .generate(self.seed, index)
                    .unwrap()
                    .generate_public_key()
                    .unwrap()
                    .trits()
                    .to_owned(),
            );

            if let Ok(false) = Client::is_address_used(&address).await {
                break Ok((index, address));
            }

            index += 1;
        }
    }
}
