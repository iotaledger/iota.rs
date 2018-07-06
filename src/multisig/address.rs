use crate::crypto::{Kerl, Sponge, HASH_LENGTH};
use crate::utils::converter;
use crate::Result;

/// Facilitates the creation of a multisig address
/// using provided digests
#[derive(Copy, Clone, Debug, Default)]
pub struct Address {
    kerl: Kerl,
}

impl Address {
    /// Creates an instance of Address, with an optional
    /// digest to initialize with
    pub fn new(digest: Option<&str>) -> Result<Address> {
        let mut kerl = Kerl::default();
        if let Some(d) = digest {
            kerl.absorb(&converter::trits_from_string(d))?;
        }
        Ok(Address { kerl })
    }

    /// Absorbs the provided digest into the address
    pub fn absorb(&mut self, digests: &[String]) -> Result<()> {
        for digest in digests {
            self.kerl.absorb(&converter::trits_from_string(digest))?;
        }
        Ok(())
    }

    /// Consumes self and returns the address string
    pub fn finalize(mut self, digest: Option<&str>) -> Result<String> {
        if let Some(d) = digest {
            self.kerl.absorb(&converter::trits_from_string(d))?;
        }
        let mut address_trits = [0; HASH_LENGTH];
        self.kerl.squeeze(&mut address_trits)?;
        Ok(converter::trytes(&address_trits))
    }
}
