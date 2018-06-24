use super::curl::Curl;
use super::kerl::Kerl;

use std::fmt;

pub const HASH_LENGTH: usize = 243;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    CURLP27,
    CURLP81,
    Kerl,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Sponge
where
    Self: Default + Clone + Send + 'static,
{
    /// Absorb trits into the sponge
    fn absorb(&mut self, trits: &[i8]);
    /// Squeeze trits out of the sponge
    fn squeeze(&mut self, out: &mut [i8]);
    /// Reset the sponge to initial state
    fn reset(&mut self);
}

pub fn hash_with_mode(mode: Mode, trits: &mut [i8], out: &mut [i8]) {
    match mode {
        Mode::CURLP27 | Mode::CURLP81 => {
            let mut curl = Curl::new(mode).unwrap();
            curl.absorb(trits);
            curl.squeeze(out);
        }
        Mode::Kerl => {
            let mut kerl = Kerl::default();
            kerl.absorb(trits);
            kerl.squeeze(out);
        }
    }
}
