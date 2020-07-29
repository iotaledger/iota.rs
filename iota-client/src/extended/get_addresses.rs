use crate::error::Result;
use bee_crypto::ternary::sponge::Kerl;
use bee_signing::ternary::{
    seed::Seed,
    wots::{WotsSecurityLevel, WotsSpongePrivateKeyGeneratorBuilder},
    PrivateKey, PrivateKeyGenerator, PublicKey,
};
use bee_transaction::bundled::{Address, BundledTransactionField};

/// Builder to construct GetNewAddress API
//#[derive(Debug)]
pub struct GetAddressesBuilder<'a> {
    seed: &'a Seed,
    start: u64,
    end: u64,
    security: WotsSecurityLevel,
}

impl<'a> GetAddressesBuilder<'a> {
    pub(crate) fn new(seed: &'a Seed) -> Self {
        Self {
            seed,
            start: 0,
            end: 20,
            security: WotsSecurityLevel::Medium,
        }
    }

    /// Set key index to start search at
    pub fn start(mut self, start: u64) -> Self {
        self.start = start;
        self
    }

    /// Set key index to end the search
    pub fn end(mut self, end: u64) -> Self {
        self.end = end;
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
    pub async fn generate(self) -> Result<Vec<Address>> {
        let mut index = self.start;
        let mut res = Vec::new();

        while index != self.end {
            // TODO impl Error trait in iota_signing_preview
            let address = Address::from_inner_unchecked(
                WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
                    .with_security_level(self.security)
                    .build()
                    .unwrap()
                    .generate_from_seed(self.seed, index)
                    .unwrap()
                    .generate_public_key()
                    .unwrap()
                    .as_trits()
                    .to_owned(),
            );

            res.push(address);

            index += 1;
        }

        Ok(res)
    }
}
