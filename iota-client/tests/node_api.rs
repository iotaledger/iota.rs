// These are E2E test samples, so they are ignored by default.

use bee_common_ext::packable::Packable;
use bee_message::payload::transaction::TransactionEssenceBuilder;
use bee_message::prelude::*;
use bee_signing_ext::{
    binary::{BIP32Path, Ed25519PrivateKey, Ed25519Seed},
    Seed, Signer,
};
use iota_client::{hex_to_address, hex_to_message_id, hex_to_transaction_id};

use std::num::NonZeroU64;

#[ignore]
#[tokio::test]
async fn test_get_info() {
    iota_client::Client::get_info("http://0.0.0.0:14265")
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_health() {
    iota_client::Client::get_health("http://0.0.0.0:14265")
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_tips() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_tips()
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_post_message_with_indexation() {
    let index = Indexation::new(String::from("Hello"), Box::new([]));

    let client = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap();

    let tips = client.get_tips().await.unwrap();

    let message = Message::builder()
        .with_parent1(tips.0)
        .with_parent2(tips.1)
        .with_payload(Payload::Indexation(Box::new(index)))
        .finish()
        .unwrap();

    let r = client.post_message(&message).await.unwrap();

    println!("{}", r);
}

#[ignore]
#[tokio::test]
async fn test_post_message_with_transaction() {
    let client = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap();

    // let seed = Ed25519Seed::from_bytes(
    //     &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    // )
    // .unwrap();

    // let pri =
    //     Ed25519PrivateKey::generate_from_seed(&seed, &BIP32Path::from_str("m").unwrap()).unwrap();
    let private_key = Ed25519PrivateKey::from_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();
    let public_key = private_key.generate_public_key().to_bytes();
    //println!("{}", hex::encode(public_key));

    let mut output_address = [0u8; 32];
    hex::decode_to_slice(
        "6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92",
        &mut output_address,
    )
    .unwrap();
    let output_address = Ed25519Address::new(output_address);
    let inputs = client
        .get_address()
        .outputs(&output_address.into())
        .await
        .unwrap();

    let address = client
        .get_unspent_address(&Seed::from_ed25519_bytes(&[0u8; 32]).unwrap())
        .path(&BIP32Path::from_str("m").unwrap())
        .get()
        .await
        .unwrap();
    let output = Output::from(SignatureLockedSingleOutput::new(
        address.0,
        NonZeroU64::new(100).unwrap(),
    ));
    let essence = TransactionEssenceBuilder::new()
        .add_input(inputs[0].clone().into())
        .add_output(output)
        .finish()
        .unwrap();
    let mut serialized_essence = vec![];
    essence.pack(&mut serialized_essence).unwrap();

    let signature = Box::new(private_key.sign(&serialized_essence).to_bytes());
    let unlock = UnlockBlock::Signature(SignatureUnlock::Ed25519(Ed25519Signature::new(
        public_key, signature,
    )));

    let transaction = TransactionBuilder::new()
        .with_essence(essence)
        .add_unlock_block(unlock)
        .finish()
        .unwrap();

    //println!("{:#?}", transaction);
    let tips = client.get_tips().await.unwrap();
    let message = Message::builder()
        .with_parent1(tips.0)
        .with_parent2(tips.1)
        .with_payload(Payload::Transaction(Box::new(transaction)))
        .finish()
        .unwrap();

    let r = client.post_message(&message).await.unwrap();

    println!("{}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_by_index() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .index("HORNET Spammer")
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_data() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .data(
            &hex_to_message_id("abf677332011485dfd741df6900f92b615a26721e4e6adfa074dccacad471f1b")
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_metadata() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .metadata(
            &hex_to_message_id("dc9492aaf06d12fd3927a3ce6e5e278edce930e0fa13ec3a09148ace6fe9448a")
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_raw() {
    iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .raw(
            &hex_to_message_id("a008ce3354591950232c0dacdfcb17c4f6457c5bf407eff1befaab5fa7b3b7b3")
                .unwrap(),
        )
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_message_children() {
    iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .children(
            &hex_to_message_id("a008ce3354591950232c0dacdfcb17c4f6457c5bf407eff1befaab5fa7b3b7b3")
                .unwrap(),
        )
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_address_balance() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_address()
        .balance(
            &hex_to_address("6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92")
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_address_outputs() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_address()
        .outputs(
            &hex_to_address("d2adf03c21269b25a0bb4319471213161f2a4fb57b16cc2e505b87b2ca52d37d")
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_output() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_output(
            &UTXOInput::new(
                hex_to_transaction_id(
                    "0000000000000000000000000000000000000000000000000000000000000000",
                )
                .unwrap(),
                0,
            )
            .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_milestone() {
    let r = iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_milestone(2)
        .await
        .unwrap();

    println!("{:#?}", r);
}
