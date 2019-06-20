use iota_conversion;
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
        HMAC {
            key: iota_conversion::trits_from_string(key),
        }
    }

    /// Using the key provided earlier, add an HMAC to provided
    /// Bundle
    pub fn add_hmac(&self, bundle: &mut Bundle) -> Result<()> {
        let mut curl = Curl::new(HashMode::CURLP27)?;
        let key = self.key.clone();
        for b in bundle.bundle_mut().iter_mut() {
            if b.value().unwrap_or_default() > 0 {
                let bundle_hash_trits =
                    iota_conversion::trits_from_string(&b.bundle().unwrap_or_default());
                let mut hmac = [0; 243];
                curl.reset();
                curl.absorb(&key)?;
                curl.absorb(&bundle_hash_trits)?;
                curl.squeeze(&mut hmac)?;
                let hmac_trytes = iota_conversion::trytes(&hmac);
                *b.signature_fragments_mut() = Some(
                    hmac_trytes
                        + &b.signature_fragments()
                            .unwrap_or_default()
                            .chars()
                            .skip(81)
                            .take(2106)
                            .collect::<String>(),
                );
            }
        }
        Ok(())
    }
}
