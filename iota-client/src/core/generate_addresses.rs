use crate::error::{Error, Result};
use bee_crypto::ternary::sponge::Kerl;
use bee_signing::ternary::{
    seed::Seed,
    wots::{WotsSecurityLevel, WotsSpongePrivateKeyGeneratorBuilder},
    PrivateKey, PrivateKeyGenerator, PublicKey,
};
use bee_transaction::bundled::{Address, BundledTransactionField};
use std::ops::Range;

/// Builder to construct GetAddress
pub struct AddressBuilder<'a> {
    seed: Option<&'a Seed>,
    security: WotsSecurityLevel,
    range: Range<usize>,
}

impl<'a> AddressBuilder<'a> {
    /// Create AddressBuilder
    pub fn builder() -> Self {
        Self {
            seed: None,
            security: WotsSecurityLevel::Medium,
            range: 0..20,
        }
    }

    /// Set the seed to the builder
    pub fn with_seed(mut self, seed: &'a Seed) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set security level
    pub fn with_security(mut self, security: u8) -> Self {
        self.security = match security {
            1 => WotsSecurityLevel::Low,
            2 => WotsSecurityLevel::Medium,
            3 => WotsSecurityLevel::High,
            _ => panic!("Invalid security level"),
        };
        self
    }

    /// Set range to the builder
    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.range = range;
        self
    }

    /// Generate addresses
    pub fn finish(self) -> Result<Vec<(usize, Address)>> {
        let seed = match self.seed {
            Some(s) => s,
            None => return Err(Error::MissingSeed),
        };

        let mut addresses = Vec::new();
        for index in self.range {
            // TODO impl Error trait in iota_signing_preview
            let address = Address::from_inner_unchecked(
                WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
                    .with_security_level(self.security)
                    .build()
                    .unwrap()
                    .generate_from_seed(seed, index)
                    .unwrap()
                    .generate_public_key()
                    .unwrap()
                    .as_trits()
                    .to_owned(),
            );

            addresses.push((index, address))
        }
        Ok(addresses)
    }
}
