use iota_client::hex_to_message_id;

#[tokio::test]
async fn test_get_info() {
    iota_client::Client::get_info("http://0.0.0.0:14265")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_health() {
    iota_client::Client::get_health("http://0.0.0.0:14265")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_tips() {
    iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_tips()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_message_by_index() {
    iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .index("TEST")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_message_data() {
    iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .data(
            &hex_to_message_id("63ec182d5ff9e228af83f8a00f0437ef6b061d6a64282f7dd623f94b621e0636")
                .unwrap(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_message_metadata() {
    iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .metadata(
            &hex_to_message_id("a008ce3354591950232c0dacdfcb17c4f6457c5bf407eff1befaab5fa7b3b7b3")
                .unwrap(),
        )
        .await
        .unwrap();
}

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

#[tokio::test]
async fn test_get_milestone() {
    iota_client::Client::new()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_milestone(1)
        .await
        .unwrap();
}
