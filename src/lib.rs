#![recursion_limit = "1024"]
#![allow(dead_code)]

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate ascii;
#[macro_use]
extern crate error_chain;
extern crate core;
extern crate crunchy;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate tiny_keccak;

pub mod model;
pub mod pow;
pub mod utils;
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}
pub mod iota_api;
pub mod iri_api;
