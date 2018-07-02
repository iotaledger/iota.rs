use crate::utils::converter;
use crate::model::Bundle;
use crate::pow::{Sponge, Mode, Curl};

use failure::Error;

const HMAC_ROUNDS: usize = 27;

pub struct HMAC {
    key: Vec<i8>,
}

impl HMAC {
    pub fn new(key: &str) -> HMAC {
        HMAC {
            key: converter::trits_from_string(key),
        }
    }

    pub fn add_hmac(&self, bundle: &mut Bundle) -> Result<(), Error> {
        let mut curl = Curl::new(Mode::CURLP27)?;
        let key = self.key.clone();
        for b in bundle.bundle_mut().iter_mut() {
            if b.value().unwrap_or_default() > 0 {
                let bundle_hash_trits = converter::trits_from_string(&b.bundle().unwrap_or_default());
                let mut hmac = [0; 243];
                curl.reset();
                curl.absorb(&key)?;
                curl.absorb(&bundle_hash_trits)?;
                curl.squeeze(&mut hmac)?;
                let hmac_trytes = converter::trytes(&hmac);
                *b.signature_fragments_mut() = Some(hmac_trytes + &b.signature_fragments().unwrap_or_default().chars().skip(81).take(2106).collect::<String>());
            }
        }
        Ok(())
    }
}