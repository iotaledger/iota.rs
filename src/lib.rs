//! This library provides pretty much anything you could need to work with
//! Iota. The documentation is a work in progress, but if you need any help
//! I can usually be found on the Iota discord rust or development chats.
//!
//! Heres a quick example of how to send a transaction (Note that trytes is being
//! used as a seed here...don't do that)
//!```
//! #![feature(futures_api)]
//! #![feature(async_await)]
//! #![feature(await_macro)]
//! extern crate iota_lib_rs;
//! extern crate futures;
//!
//! use iota_lib_rs::iota_api;
//! use iota_lib_rs::options::SendTransferOptions;
//! use iota_lib_rs::utils::trytes_converter;
//! use iota_lib_rs::model::*;
//!
//! use futures::executor::block_on;
//! use futures::executor::ThreadPool;
//! use futures::task::SpawnExt;
//!
//! fn main() {
//!     let trytes = "HELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDHELLOWORLDD";
//!     let message = trytes_converter::to_trytes("Hello World").unwrap();
//!     let mut transfer = Transfer::default();
//!     *transfer.value_mut() = 0;
//!     *transfer.address_mut() = trytes.to_string();
//!     *transfer.message_mut() = message;
//!     let api = iota_api::API::new("https://pow3.iota.community");
//!     let options = SendTransferOptions{
//!         threads: None,
//!         inputs: None,
//!         reference: None,
//!         remainder_address: None,
//!         security: None,
//!         hmac_key: None,
//!     };
//!    // If you want to do this synchronously and block on the send_transfers call
//!    let mut tx = block_on(api.send_transfers(vec![transfer.clone()], trytes.to_string(), 3, 14, true, options.clone())).unwrap();
//!    println!("{:?}", tx);
//!
//!    // Create thread pool
//!    let mut thread_pool = ThreadPool::new().expect("Failed to create threadpool");
//!    // Spawn async request on pool
//!    let h = thread_pool.spawn_with_handle(async move {
//!         await!(api.send_transfers(vec![transfer], trytes.to_string(), 3, 14, true, options)).unwrap()
//!    }).unwrap();
//!    // Wait for completion
//!    tx = thread_pool.run(h);
//!
//!    println!("{:?}", tx);
//! }
//!```
#![allow(dead_code)]
#![feature(futures_api)]
#![feature(async_await)]
#![feature(await_macro)]
#![feature(nll)]

#[macro_use]
extern crate crunchy;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate crossbeam;
extern crate num_cpus;
extern crate rand;
extern crate regex;
extern crate reqwest;

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
/// Provides options structs for use throughout API
pub mod options;
/// Provides many useful helper functions that are used throughout
/// the library
pub mod utils;

use std::result;

/// This is the result type used throughout the library
pub type Result<T> = result::Result<T, failure::Error>;
