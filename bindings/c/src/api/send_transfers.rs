use anyhow::Result;
use iota::client::Transfer;
use crate::CSeed;

#[repr(C)]
pub struct CTransfers {
    // TODO method to add transfers
    inner: Vec<Transfer>,
}

#[no_mangle]
pub extern "C" fn iota_send_transfers(seed: *const CSeed, transfers: *mut CTransfers, mwm: u8, err: &mut u8) {
    *err = 0;

    send_transfers(seed, transfers, mwm).unwrap_or_else(|_| {
        *err = 1;
    })
}

fn send_transfers(seed: *const CSeed, transfers: *mut CTransfers, mwm: u8) -> Result<()> {
    let seed = unsafe {
        assert!(!seed.is_null());
        &(*seed).0
    };

    let transfers = unsafe{ (*transfers).inner.clone() };
    let _ = smol::block_on(async move {
        iota::Client::send_transfers(&seed)
            .transfers(transfers)
            .min_weight_magnitude(mwm)
            .send()
            .await
    })?;

     Ok(())
}
