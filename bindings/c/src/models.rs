use iota::bundle;
use iota::client::Transfer;
use iota::crypto::Kerl;
use iota::signing::{IotaSeed, Seed};

pub struct CSeed(pub(crate) IotaSeed<Kerl>);

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
pub extern "C" fn iota_address_new() -> *mut Address {
    Box::into_raw(Box::new(Address(bundle::Address::zeros())))
}

#[no_mangle]
pub extern "C" fn iota_address_free(ptr: *mut Address) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

pub struct Transfers(pub(crate) Vec<Transfer>);

#[no_mangle]
pub extern "C" fn iota_transfers_new() -> *mut Transfers {
    Box::into_raw(Box::new(Transfers(Vec::new())))
}

#[no_mangle]
pub extern "C" fn iota_transfers_add(ptr: *mut Transfers, address: *mut Address, value: u64) {
    let transfers = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let address = unsafe {
        assert!(!ptr.is_null());
        (*Box::from_raw(address)).0
    };

    transfers.0.push(Transfer {
        address,
        value,
        message: None,
        tag: None,
    });
}

#[no_mangle]
pub extern "C" fn iota_transfers_free(ptr: *mut Transfers) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

pub struct Bundle(pub(crate) Vec<bundle::Transaction>);

#[no_mangle]
pub extern "C" fn iota_bundle_new() -> *mut Bundle {
    Box::into_raw(Box::new(Bundle(Vec::new())))
}

#[no_mangle]
pub extern "C" fn iota_bundle_free(ptr: *mut Bundle) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
