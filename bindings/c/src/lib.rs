use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

use iota::crypto::Kerl;
use iota::signing::{
    IotaSeed, PrivateKey, PrivateKeyGenerator, PublicKey, Seed, WotsPrivateKeyGeneratorBuilder,
    WotsSecurityLevel,
};
use iota::ternary::Trits;

#[no_mangle]
pub extern "C" fn iota_init(url: *const c_char) {
    // Init async runtime first
    struct Pending;

    impl std::future::Future for Pending {
        type Output = ();
        fn poll(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            std::task::Poll::Pending
        }
    }
    std::thread::spawn(|| smol::run(Pending));

    // Add the node to the client instance
    let c_url = unsafe {
        assert!(!url.is_null());
        CStr::from_ptr(url)
    };
    let url = c_url.to_str().unwrap();
    iota::Client::add_node(url).unwrap();
}

#[no_mangle]
pub extern "C" fn iota_address_gen(seed: *const i8, index: u64) -> *const i8 {
    let seed = unsafe {
        assert!(!seed.is_null());

        slice::from_raw_parts(seed, 243)
    };
    let seed =
        IotaSeed::<Kerl>::from_buf(Trits::try_from_raw(seed, 243).unwrap().to_owned()).unwrap();

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
pub extern "C" fn iota_get_node_info() -> *mut GetNodeInfoResponse {
    let res = smol::block_on(async move { iota::Client::get_node_info().await.unwrap() });

    Box::into_raw(Box::new(GetNodeInfoResponse {
        app_name: CString::new(res.app_name).unwrap().into_raw(),
        app_version: CString::new(res.app_version).unwrap().into_raw(),
        latest_milestone_index: res.latest_milestone_index,
    }))
}
