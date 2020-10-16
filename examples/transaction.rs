use iota::{hex_to_address, BIP32Path, Client, Seed};

use std::num::NonZeroU64;

#[tokio::main]
async fn main() {
    let iota = Client::new() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let seed = Seed::from_ed25519_bytes(&[0u8; 32]).unwrap(); // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    let path = BIP32Path::from_str("m/0'/0'").unwrap(); // Insert your account path. Note that index must be hardened(like 0', 123').
    let balance = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            hex_to_address("6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92")
                .unwrap(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", balance);
}
