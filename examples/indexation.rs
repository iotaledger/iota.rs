use hex;
use iota::Indexation;
use iota::{hex_to_message_id, Client, Message, Payload};

#[tokio::main]
async fn main() {
    let index = Indexation::new(
        String::from("Hello"),
        String::from("Tangle")
            .as_bytes()
            .to_vec()
            .into_boxed_slice(),
    );

    let client = Client::new()
        .nodes(&vec!["http://localhost:14265"])
        .unwrap()
        .build()
        .unwrap();

    let tips = client.get_tips().await.unwrap();

    let message = Message::builder()
        .parent1(tips.0)
        .parent2(tips.1)
        .payload(Payload::Indexation(Box::new(index)))
        .finish()
        .unwrap();
    println!("message: {:?}", message);
    let r = client.post_message(&message).await.unwrap();

    println!("MessageId {}", r);

    let r = client.get_message().index(&"Hello").await.unwrap();

    println!("{:#?}", r);

    let r = client
        .get_message()
        .data(&hex_to_message_id(r[0]).unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
    if let Payload::Indexation(i) = r.payload() {
        println!(
            "Data: {}",
            String::from_utf8(hex::decode(i.data()).unwrap()).expect("Found invalid UTF-8")
        );
    }
}
