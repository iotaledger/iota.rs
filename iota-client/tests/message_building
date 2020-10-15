use bee_message::prelude::*;
use bee_signing_ext::binary::BIP32Path;
use iota_client::MessageJson;

use core::num::NonZeroU64;

#[tokio::test]
async fn message_with_indexation() {
    let mut id = [0u8; 32];
    hex::decode_to_slice(
        "644772e7ac4d68b780b78dfeaafe5acc80e4201202b541d2cf202f991cf6d7e0",
        &mut id,
    )
    .unwrap();
    let msg = Message::builder()
        .parent1(MessageId::new(id.clone()))
        .parent2(MessageId::new(id.clone()))
        .payload(Payload::Indexation(Box::new(Indexation::new(
            "0000".to_owned(),
            Box::new([0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x49, 0x6f, 0x74, 0x61]),
        ))))
        .build()
        .unwrap();

    let meg: MessageJson = (&msg).into();
    println!("{:#?}", meg);
    let x = serde_json::to_string(&meg).unwrap();
    println!("{:#?}", x);
    // let r = iota_client::Client::new()
    // .node("http://0.0.0.0:14265")
    // .unwrap()
    // .build()
    // .unwrap()
    // .post_messages(&msg)
    // .await
    // .unwrap();
}
// 63ec182d5ff9e228af83f8a00f0437ef6b061d6a64282f7dd623f94b621e0636
#[tokio::test]
async fn message_with_transaction() {
    let mut id = [0u8; 32];
    hex::decode_to_slice(
        "7b09039395583b8d126ff0921e6b225fcaaf4e4669678878236abd9ddf1abe05",
        &mut id,
    )
    .unwrap();
    let transaction = TransactionBuilder::new(&Seed::from_ed25519_bytes(&[0u8; 32]).unwrap())
        .set_inputs(vec![(
            UTXOInput::new(TransactionId::new([0; 32]), 0)
                .unwrap()
                .into(),
            BIP32Path::from_str("m/0'/0'/0'").unwrap(),
        )])
        .set_outputs(vec![SignatureLockedSingleOutput::new(
            Ed25519Address::new([0; 32]).into(),
            NonZeroU64::new(100).unwrap(),
        )
        .into()])
        .build()
        .unwrap();
    let parent1 = MessageId::new(id.clone());
    let parent2 = MessageId::new(id.clone());
    let message = MessageBuilder::new()
        .parent1(parent1)
        .parent2(parent2)
        .payload(Payload::Transaction(Box::new(transaction)))
        .build()
        .unwrap();
    // let x = serde_json::to_string(&message).unwrap();
    // println!("{:#?}", x);

    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .post_messages(&message)
        .await
        .unwrap();

    println!("{:#?}", r);
}
