#[tokio::test]
async fn test_get_info() {
    iota_client::Client::get_info("http://0.0.0.0:14265").await.unwrap();
}

#[tokio::test]
async fn test_get_health() {
    iota_client::Client::get_health("http://0.0.0.0:14265").await.unwrap();
}