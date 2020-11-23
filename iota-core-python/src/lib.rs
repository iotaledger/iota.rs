//! IOTA core python

#![deny(unused_extern_crates)]
use bee_signing_ext::{binary::BIP32Path, Seed};
use hex;
pub use iota_client::{hex_to_address, Client as RustClient, ClientBuilder};
use pyo3::prelude::*;
use std::num::NonZeroU64;

#[pyclass]
struct Client {
    client: RustClient,
}

#[pymethods]
impl Client {
    #[new]
    fn new(url: &str) -> Self {
        let client = RustClient::new() // Crate a client instance builder
            .node(url) // Insert the node here
            .unwrap()
            .build()
            .unwrap();
        Client { client }
    }
    fn send(&self, seed: &str, path: &str, address: &str, value: u64) -> String {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let seed = Seed::from_ed25519_bytes(&hex::decode(seed).unwrap()).unwrap();
        let mut message_id = String::new();
        rt.block_on(async {
            message_id = hex::encode(
                self.client
                    .send(&seed)
                    .path(&BIP32Path::from_str(path).unwrap())
                    // Insert the output address and ampunt to spent. The amount cannot be zero.
                    .output(
                        hex_to_address(address).unwrap(),
                        NonZeroU64::new(value).unwrap(),
                    )
                    .post()
                    .await
                    .unwrap(),
            );
        });
        message_id
    }
    fn get_address_balances(&self, address: &str) -> u64 {
        let address = hex_to_address(address).unwrap(); // Insert the address to search for
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let mut balance: u64 = 0;
        rt.block_on(async {
            balance = self.client.get_address().balance(&address).await.unwrap();
        });
        balance
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn iota_client(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    Ok(())
}
