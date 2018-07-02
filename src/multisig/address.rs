use crate::crypto::{Kerl, Sponge, HASH_LENGTH};
use crate::utils::converter;
use failure::Error;

#[derive(Default, Clone)]
pub struct Address {
    kerl: Kerl,
}

impl Address {
    pub fn new(digest: Option<&str>) -> Result<Address, Error> {
        let mut kerl = Kerl::default();
        if let Some(d) = digest {
            kerl.absorb(&converter::trits_from_string(d))?;
        }
        Ok(Address { kerl })
    }

    pub fn absorb(&mut self, digests: &[String]) -> Result<(), Error> {
        for digest in digests {
            self.kerl.absorb(&converter::trits_from_string(digest))?;
        }
        Ok(())
    }

    pub fn finalize(&mut self, digest: Option<&str>) -> Result<String, Error> {
        if let Some(d) = digest {
            self.kerl.absorb(&converter::trits_from_string(d))?;
        }
        let mut address_trits = [0; HASH_LENGTH];
        self.kerl.squeeze(&mut address_trits)?;
        Ok(converter::trytes(&address_trits))
    }
}
