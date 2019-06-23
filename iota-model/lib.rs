#![deny(unused_extern_crates)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

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
