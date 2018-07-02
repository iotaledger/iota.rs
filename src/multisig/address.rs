use crate::pow::{Kerl, Sponge, HASH_LENGTH};
use crate::utils::converter;

#[derive(Default, Clone)]
pub struct Address {
    kerl: Kerl,
}

impl Address {
    pub fn new(digest: Option<&str>) -> Address {
        let mut kerl = Kerl::default();
        if let Some(d) = digest {
            kerl.absorb(&converter::trits_from_string(d));
        }
        Address { kerl }
    }

    pub fn absorb(&mut self, digests: &[String]) {
        for digest in digests {
            self.kerl.absorb(&converter::trits_from_string(digest));
        }
    }

    pub fn finalize(&mut self, digest: Option<&str>) -> String {
        if let Some(d) = digest {
            self.kerl.absorb(&converter::trits_from_string(d));
        }
        let mut address_trits = [0; HASH_LENGTH];
        self.kerl.squeeze(&mut address_trits);
        converter::trytes(&address_trits)
    }
}
