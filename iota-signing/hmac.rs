use iota_conversion::Trinary;
use iota_crypto::{Curl, HashMode, Sponge};
use iota_model::Bundle;

use crate::Result;

const HMAC_ROUNDS: usize = 27;

/// HMAC uses curl to provide an extra layer of verification
/// to bundles
///```rust
/// use iota_signing::{self, HMAC};
/// use iota_model::Bundle;
///
/// let mut bundle = Bundle::default();
/// let hmac = HMAC::new("apples");
/// hmac.add_hmac(&mut bundle);
///```
#[derive(Clone, Debug)]
pub struct HMAC {
    key: Vec<i8>,
}

impl HMAC {
    /// Creates a new HMAC instance using the provided key
    pub fn new(key: &str) -> HMAC {
        HMAC { key: key.trits() }
    }

    /// Using the key provided earlier, add an HMAC to provided
    /// Bundle
    pub fn add_hmac(&self, bundle: &mut Bundle) -> Result<()> {
        let mut curl = Curl::new(HashMode::CURLP27)?;
        let key = self.key.clone();
        for b in bundle.bundle.iter_mut() {
            if b.value > 0 {
                let bundle_hash_trits = b.bundle.trits();
                let mut hmac = [0; 243];
                curl.reset();
                curl.absorb(&key)?;
                curl.absorb(&bundle_hash_trits)?;
                curl.squeeze(&mut hmac)?;
                let hmac_trytes = hmac.trytes()?;
                b.signature_fragments = hmac_trytes
                    + &b.signature_fragments
                        .chars()
                        .skip(81)
                        .take(2106)
                        .collect::<String>();
            }
        }
        Ok(())
    }
}
