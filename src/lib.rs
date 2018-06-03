#![recursion_limit = "1024"]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate ascii;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate crunchy;
extern crate core;
extern crate tiny_keccak;

pub mod model;
pub mod pow;
pub mod utils;
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}
