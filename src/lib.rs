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

pub mod iota_api;
pub mod iri_api;
pub mod model;
pub mod pow;
pub mod utils;
