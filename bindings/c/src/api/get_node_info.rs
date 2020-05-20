use std::ffi::CString;
use std::os::raw::c_char;

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
