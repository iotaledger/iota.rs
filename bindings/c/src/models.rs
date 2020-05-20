use anyhow::Result;
use iota::bundle::TransactionField;
use iota::bundle;
use iota::crypto::Kerl;
use iota::signing::{IotaSeed, Seed};
use iota::ternary::Trits;
use std::slice;

pub struct CSeed (pub(crate) IotaSeed<Kerl>);

#[no_mangle]
pub extern "C" fn iota_seed_new() -> *mut CSeed {
    Box::into_raw(Box::new(CSeed(IotaSeed::new())))
}

#[no_mangle]
pub extern "C" fn iota_seed_free(ptr: *mut CSeed) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

pub struct Address(pub(crate) bundle::Address);

#[no_mangle]
pub extern "C" fn iota_address_free(ptr: *mut Address) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}