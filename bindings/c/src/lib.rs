use std::ffi::CStr;
use std::os::raw::c_char;

mod api;
mod utils;

pub use api::*;
pub use utils::*;

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
