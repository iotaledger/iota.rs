#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Structs representing the various data types used by Iota

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
