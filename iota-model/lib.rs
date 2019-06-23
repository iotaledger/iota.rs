#![deny(unused_extern_crates)]

#[macro_use]
extern crate failure;

pub use self::bundle::*;
pub use self::input::*;
pub use self::inputs::*;
pub use self::neighbor::*;
pub use self::signature::*;
pub use self::transaction::*;
pub use self::transfer::*;

mod bundle;
mod input;
mod inputs;
mod neighbor;
mod signature;
mod transaction;
mod transfer;

mod trit_adder;

type Result<T> = ::std::result::Result<T, failure::Error>;

pub trait Trinary {
    fn trits(&self) -> Vec<Trit>;
    fn trytes(&self) -> Trytes;
}

pub type Trit = i8;

impl Trinary for Vec<Trit> {
    fn trits(&self) -> Vec<Trit> {
        self.to_vec()
    }
    fn trytes(&self) -> Trytes {
        iota_conversion::trytes(self)
    }
}

pub type Trytes = String;

impl Trinary for Trytes {
    fn trits(&self) -> Vec<Trit> {
        iota_conversion::trits_from_string(self)
    }
    fn trytes(&self) -> Trytes {
        self.clone()
    }
}