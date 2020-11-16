use iota::{BIP32Path, Client, Seed};

#[tokio::main]
async fn main() {
    let iota = Client::new() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let seed = Seed::from_ed25519_bytes(&[0u8; 32]).unwrap(); // Insert your seed
    let path = BIP32Path::from_str("m/0'/0'").unwrap(); // Insert your account path. Note that index must be hardened(like 0', 123').

    let address = iota
        .get_unspent_address(&seed)
        .path(&path)
        .get()
        .await
        .unwrap();

    println!("Get an unspent address: {:#?}", address);

    let addresses = iota
        .find_addresses(&seed)
        .path(&path)
        .range(0..3)
        .get()
        .unwrap();

    println!("List of generated address: {:#?}", addresses);
}
