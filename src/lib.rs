#![allow(dead_code)]
#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

extern crate crunchy;
extern crate crossbeam;
extern crate num_cpus;
extern crate chrono;
extern crate reqwest;
extern crate rand;
extern crate regex;

/// Provides all crypto algorithms and data structures used by Iota
pub mod crypto;
/// Provides helper functions that make interacting with IRI easier
pub mod iota_api;
/// Provides methods to call IRI APIs
pub mod iri_api;
/// Provides the various struces used by Iota
pub mod model;
/// Provides multi-sig functionality
pub mod multisig;
/// Provides many useful helper functions that are used throughout
/// the library
pub mod utils;

use std::result;

/// This is the result type used throughout the library
pub type Result<T> = result::Result<T, failure::Error>;
