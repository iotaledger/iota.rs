#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Crypto used by Iota

#[macro_use]
extern crate crunchy;
#[macro_use]
extern crate failure;

use std::fmt;

pub use self::curl::*;
pub use self::iss::*;
pub use self::kerl::*;

mod curl;
mod iss;
mod keccak;
mod kerl;

type Result<T> = ::std::result::Result<T, failure::Error>;

/// Mode allows for mode selection to rely on rusts type system
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HashMode {
    /// Curl with 27 rounds
    CURLP27,
    /// Curl with 81 rounds
    CURLP81,
    /// Curl with Keccak as the hashing algorithm
    Kerl,
}

impl fmt::Display for HashMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The sponge trait specifys the main functionality of all
/// sponges used throughout IOTA
pub trait Sponge
where
    Self: Default + Clone + Send + 'static,
{
    /// Absorb trits into the sponge
    ///
    /// * `trits` - A slice of trits whose length is a multiple of 243
    fn absorb(&mut self, trits: &[i8]) -> Result<()>;
    /// Squeeze trits out of the sponge and copy them into `out`
    ///
    /// * `out` - A slice of trits whose length is a multiple of 243
    fn squeeze(&mut self, out: &mut [i8]) -> Result<()>;
    /// Reset the sponge to initial state
    fn reset(&mut self);
}

/// Allows you to hash `trits` into `out` using the `mode` of your choosing
///```rust
/// use iota_crypto::{self, HashMode};
///
/// let input = [0; 243];
/// let mut out = [0; 243];
/// iota_crypto::hash_with_mode(HashMode::Kerl, &input, &mut out);
///```
pub fn hash_with_mode(mode: HashMode, trits: &[i8], out: &mut [i8]) -> Result<()> {
    ensure!(
        out.len() % 243 == 0,
        "Output slice length isn't a multiple of 243: {}",
        out.len()
    );
    match mode {
        HashMode::CURLP27 | HashMode::CURLP81 => {
            let mut curl = Curl::new(mode).unwrap();
            curl.absorb(trits)?;
            curl.squeeze(out)?;
        }
        HashMode::Kerl => {
            let mut kerl = Kerl::default();
            kerl.absorb(trits)?;
            kerl.squeeze(out)?;
        }
    }
    Ok(())
}
