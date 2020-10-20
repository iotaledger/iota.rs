use iota::{hex_to_address, Client};

#[tokio::main]
async fn main() {
    let iota = Client::new() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let address =
        hex_to_address("6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92").unwrap(); // Insert the address to search for
    let balance = iota.get_address().balance(&address).await.unwrap();
    println!("The balance of {:?} is {:?}", address, balance);

    let outputs = iota.get_address().outputs(&address).await.unwrap();
    println!("The outputs of {:?} are {:?}", address, outputs);
}
