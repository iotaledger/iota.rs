use anyhow::Result;
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct GetNodeInfoResponse {
    pub app_name: *const c_char,
    pub app_version: *const c_char,
    pub latest_milestone_index: u32,
}

#[no_mangle]
pub extern "C" fn iota_get_node_info(err: &mut u8) -> *mut GetNodeInfoResponse {
    *err = 0;
    get_node_info().unwrap_or_else(|_| {
        *err = 1;
        std::ptr::null_mut()
    })
}

fn get_node_info() -> Result<*mut GetNodeInfoResponse> {
    let res = smol::block_on(async move { iota::Client::get_node_info().await })?;

    Ok(Box::into_raw(Box::new(GetNodeInfoResponse {
        app_name: CString::new(res.app_name)?.into_raw(),
        app_version: CString::new(res.app_version)?.into_raw(),
        latest_milestone_index: res.latest_milestone_index,
    })))
}
