#[test]
fn test_get_info() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let c = iota_client::Client::new().build().unwrap();
        let r = c.get_info("http://0.0.0.0:14265").await.unwrap();
        println!("{:#?}", r);
    });
}

#[test]
fn test_get_health() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let c = iota_client::Client::new().build().unwrap();
        let _ = c.get_health("http://0.0.0.0:14265").await.unwrap();
    });
}