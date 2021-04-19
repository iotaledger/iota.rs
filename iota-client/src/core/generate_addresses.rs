use crate::error::{Error, Result};
use bee_transaction::bundled::{Address, BundledTransactionField};
use crypto::hashes::ternary::kerl::Kerl;
use crypto::keys::ternary::seed::Seed;
use crypto::keys::ternary::wots::sponge::WotsSpongePrivateKeyGeneratorBuilder;
use crypto::keys::ternary::wots::WotsSecurityLevel;
use crypto::keys::ternary::PrivateKeyGenerator;
use crypto::signatures::ternary::{PrivateKey, PublicKey};
use std::ops::Range;

/// Builder to construct GetAddress
pub struct AddressBuilder<'a> {
    seed: Option<&'a Seed>,
    security: WotsSecurityLevel,
    range: Range<u64>,
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
    pub fn with_range(mut self, range: Range<u64>) -> Self {
        self.range = range;
        self
    }

    /// Generate addresses
    pub fn finish(self) -> Result<Vec<(u64, Address)>> {
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
                    .generate_from_seed(seed, index as usize)
                    .unwrap()
                    .generate_public_key()
                    .unwrap()
                    .as_trits()
                    .to_owned(),
            );

            addresses.push((index as u64, address))
        }
        Ok(addresses)
    }
}
