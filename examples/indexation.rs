use hex;
use iota::Indexation;
use iota::{Client, Message, Payload};

#[tokio::main]
async fn main() {
    let index = Indexation::new(String::from("Hello"), String::from("Tangle").as_bytes()).unwrap();

    let client = Client::builder()
        .nodes(&vec!["http://localhost:14265"])
        .unwrap()
        .build()
        .unwrap();

    let tips = client.get_tips().await.unwrap();

    let message = Message::builder()
        .with_network_id(0)
        .with_parent1(tips.0)
        .with_parent2(tips.1)
        .with_payload(Payload::Indexation(Box::new(index)))
        .finish()
        .unwrap();
    println!("message: {:?}", message);
    let r = client.post_message(&message).await.unwrap();

    println!("MessageId {}", r);

    let fetched_messages = client.get_message().index(&"Hello").await.unwrap();

    println!("{:#?}", fetched_messages);

    let r = client
        .get_message()
        .data(&fetched_messages[0])
        .await
        .unwrap();

    println!("{:#?}", r);
    if let Payload::Indexation(i) = r.payload().as_ref().unwrap() {
        println!(
            "Data: {}",
            String::from_utf8(hex::decode(i.data()).unwrap()).expect("Found invalid UTF-8")
        );
    }
}
