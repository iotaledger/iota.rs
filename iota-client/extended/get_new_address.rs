use anyhow::Result;
use bee_bundle::{Address, TransactionField};
use bee_crypto::Kerl;
use bee_signing::{
    IotaSeed, PrivateKey, PrivateKeyGenerator, PublicKey, WotsPrivateKeyGeneratorBuilder,
    WotsSecurityLevel,
};

use crate::Client;

/// Builder to construct GetNewAddress API
//#[derive(Debug)]
pub struct GetNewAddressBuilder<'a> {
    client: &'a Client<'a>,
    seed: Option<&'a IotaSeed<Kerl>>,
    index: u64,
    security: u8,
}

impl<'a> GetNewAddressBuilder<'a> {
    pub(crate) fn new(client: &'a Client<'a>) -> Self {
        Self {
            client,
            seed: None,
            index: 0,
            security: 2,
        }
    }

    /// Add iota seed
    pub fn seed(mut self, seed: &'a IotaSeed<Kerl>) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set key index to start search at
    pub fn index(mut self, index: u64) -> Self {
        self.index = index;
        self
    }

    /// Set security level
    pub fn security(mut self, security: u8) -> Self {
        self.security = security;
        self
    }

    /// Send GetNewAddress request
    pub async fn send(self) -> Result<Address> {
        let security = match self.security {
            1 => WotsSecurityLevel::Low,
            2 => WotsSecurityLevel::Medium,
            3 => WotsSecurityLevel::High,
            _ => return Err(anyhow!("Security level only supports 1~3")),
        };

        let seed = match self.seed {
            Some(s) => s,
            None => return Err(anyhow!("Seed is not provided")),
        };

        let mut index = self.index;

        loop {
            // TODO impl Error trait in bee_signing
            let address = Address::from_inner_unchecked(
                WotsPrivateKeyGeneratorBuilder::<Kerl>::default()
                    .security_level(WotsSecurityLevel::from(security))
                    .build()
                    .unwrap()
                    .generate(seed, index)
                    .unwrap()
                    .generate_public_key()
                    .unwrap()
                    .trits()
                    .to_owned(),
            );

            index += 1;

            if let Ok(false) = self.client.is_address_used(&address).await {
                break Ok(address);
            }
        }
    }
}
