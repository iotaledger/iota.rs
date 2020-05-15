use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

use iota::crypto::Kerl;
use iota::signing::{
    IotaSeed, PrivateKey, PrivateKeyGenerator, PublicKey, Seed, WotsPrivateKeyGeneratorBuilder,
    WotsSecurityLevel,
};
use iota::ternary::TritBuf;

#[no_mangle]
pub extern "C" fn iota_address_gen(seed: *const i8, index: u64) -> *const i8 {
    let seed = unsafe {
        assert!(!seed.is_null());

        slice::from_raw_parts(seed, 243)
    };
    let seed = IotaSeed::<Kerl>::from_buf(TritBuf::from_i8_unchecked(seed)).unwrap();

    WotsPrivateKeyGeneratorBuilder::<Kerl>::default()
        .security_level(WotsSecurityLevel::Low)
        .build()
        .unwrap()
        .generate(&seed, index)
        .unwrap()
        .generate_public_key()
        .unwrap()
        .as_bytes()
        .as_ptr()
}

#[repr(C)]
pub struct GetNodeInfoResponse {
    pub app_name: *const c_char,
    pub app_version: *const c_char,
    pub latest_milestone_index: u32,
}

#[no_mangle]
pub extern "C" fn get_node_info(url: *const c_char) -> *mut GetNodeInfoResponse {
    let c_url = unsafe {
        assert!(!url.is_null());
        CStr::from_ptr(url)
    };
    let url = c_url.to_str().unwrap();

    let iota = iota::Client::new(url).unwrap();
    let res = smol::run(async { iota.get_node_info().await.unwrap() });

    Box::into_raw(Box::new(GetNodeInfoResponse {
        app_name: CString::new(res.app_name).unwrap().into_raw(),
        app_version: CString::new(res.app_version).unwrap().into_raw(),
        latest_milestone_index: res.latest_milestone_index,
    }))
}
